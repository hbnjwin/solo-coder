from flask import request, jsonify
from datetime import datetime
import random
import time
from app import db
from app.models import App, ABTest, ABTestVariant, CallLog
from app.stats import perform_ab_test_analysis

def register_routes(app):
    @app.route('/api/apps', methods=['GET'])
    def get_apps():
        apps = App.query.order_by(App.created_at.desc()).all()
        return jsonify([app.to_dict() for app in apps])

    @app.route('/api/apps', methods=['POST'])
    def create_app():
        data = request.get_json()
        app = App(
            name=data.get('name', ''),
            description=data.get('description', ''),
            system_prompt=data.get('system_prompt', ''),
            model_id=data.get('model_id', 'gpt-3.5-turbo'),
            temperature=data.get('temperature', 0.7),
            max_tokens=data.get('max_tokens', 1000)
        )
        db.session.add(app)
        db.session.commit()
        return jsonify(app.to_dict()), 201

    @app.route('/api/apps/<int:app_id>', methods=['GET'])
    def get_app(app_id):
        app = App.query.get_or_404(app_id)
        return jsonify(app.to_dict())

    @app.route('/api/apps/<int:app_id>', methods=['PUT'])
    def update_app(app_id):
        app = App.query.get_or_404(app_id)
        data = request.get_json()
        app.name = data.get('name', app.name)
        app.description = data.get('description', app.description)
        app.system_prompt = data.get('system_prompt', app.system_prompt)
        app.model_id = data.get('model_id', app.model_id)
        app.temperature = data.get('temperature', app.temperature)
        app.max_tokens = data.get('max_tokens', app.max_tokens)
        app.updated_at = datetime.utcnow()
        db.session.commit()
        return jsonify(app.to_dict())

    @app.route('/api/apps/<int:app_id>', methods=['DELETE'])
    def delete_app(app_id):
        app = App.query.get_or_404(app_id)
        db.session.delete(app)
        db.session.commit()
        return jsonify({'message': 'App deleted successfully'})

    @app.route('/api/apps/<int:app_id>/ab-tests', methods=['GET'])
    def get_ab_tests(app_id):
        app = App.query.get_or_404(app_id)
        tests = ABTest.query.filter_by(app_id=app_id).order_by(ABTest.created_at.desc()).all()
        return jsonify([t.to_dict() for t in tests])

    @app.route('/api/apps/<int:app_id>/ab-tests', methods=['POST'])
    def create_ab_test(app_id):
        app = App.query.get_or_404(app_id)
        data = request.get_json()

        test_name = data.get('test_name')
        if not test_name:
            return jsonify({'error': 'test_name is required'}), 400

        variants_data = data.get('variants', [])
        if len(variants_data) < 2 or len(variants_data) > 4:
            return jsonify({'error': 'Must provide 2-4 variants'}), 400

        target_type = data.get('target_type', 'all')
        if target_type not in ['prompt', 'model', 'params', 'all']:
            return jsonify({'error': 'Invalid target_type. Must be prompt, model, params, or all'}), 400

        running_test = ABTest.query.filter_by(app_id=app_id, status='running').first()
        if running_test:
            return jsonify({'error': 'There is already a running A/B test for this app'}), 400

        traffic_percentages = [v.get('traffic_percentage') for v in variants_data]
        if any(tp is not None for tp in traffic_percentages):
            total = sum(tp for tp in traffic_percentages if tp is not None)
            if abs(total - 100) > 0.01:
                return jsonify({'error': 'Traffic percentages must sum to 100'}), 400
            default_tp = 0
        else:
            default_tp = 100 / len(variants_data)

        ab_test = ABTest(
            app_id=app_id,
            test_name=test_name,
            description=data.get('description', ''),
            target_type=target_type,
            status='running',
            start_time=datetime.utcnow()
        )
        db.session.add(ab_test)
        db.session.flush()

        for i, v_data in enumerate(variants_data):
            tp = v_data.get('traffic_percentage', default_tp)
            variant = ABTestVariant(
                ab_test_id=ab_test.id,
                variant_name=v_data.get('variant_name', f'Variant {chr(65 + i)}'),
                is_control=v_data.get('is_control', i == 0),
                traffic_percentage=tp,
                system_prompt=v_data.get('system_prompt', app.system_prompt),
                model_id=v_data.get('model_id', app.model_id),
                temperature=v_data.get('temperature', app.temperature),
                max_tokens=v_data.get('max_tokens', app.max_tokens)
            )
            db.session.add(variant)

        db.session.commit()
        return jsonify(ab_test.to_dict()), 201

    @app.route('/api/apps/<int:app_id>/ab-tests/<int:test_id>', methods=['GET'])
    def get_ab_test(app_id, test_id):
        ab_test = ABTest.query.filter_by(id=test_id, app_id=app_id).first_or_404()
        return jsonify(ab_test.to_dict())

    @app.route('/api/apps/<int:app_id>/ab-tests/<int:test_id>/stop', methods=['POST'])
    def stop_ab_test(app_id, test_id):
        ab_test = ABTest.query.filter_by(id=test_id, app_id=app_id).first_or_404()
        ab_test.status = 'completed'
        ab_test.end_time = datetime.utcnow()
        db.session.commit()
        return jsonify(ab_test.to_dict())

    @app.route('/api/apps/<int:app_id>/ab-tests/<int:test_id>/results', methods=['GET'])
    def get_ab_test_results(app_id, test_id):
        ab_test = ABTest.query.filter_by(id=test_id, app_id=app_id).first_or_404()
        variants = ab_test.variants

        variant_results = []
        variant_data_for_stats = []

        for variant in variants:
            logs = CallLog.query.filter_by(variant_id=variant.id).all()
            call_count = len(logs)
            success_count = sum(1 for log in logs if log.satisfaction_score is not None and log.satisfaction_score >= 4)

            if call_count > 0:
                avg_response_time = sum(log.response_time_ms for log in logs if log.response_time_ms) / call_count
                total_tokens = sum(log.tokens_consumed for log in logs)
                avg_tokens = total_tokens / call_count
                scored_logs = [log for log in logs if log.satisfaction_score is not None]
                avg_satisfaction = sum(log.satisfaction_score for log in scored_logs) / len(scored_logs) if scored_logs else None
                satisfaction_rate = success_count / call_count if call_count > 0 else 0
            else:
                avg_response_time = 0
                total_tokens = 0
                avg_tokens = 0
                avg_satisfaction = None
                satisfaction_rate = 0

            result = {
                'variant_id': variant.id,
                'variant_name': variant.variant_name,
                'is_control': variant.is_control,
                'traffic_percentage': variant.traffic_percentage,
                'call_count': call_count,
                'success_count': success_count,
                'avg_response_time_ms': round(avg_response_time, 2),
                'total_tokens_consumed': total_tokens,
                'avg_tokens_per_call': round(avg_tokens, 2),
                'avg_satisfaction_score': round(avg_satisfaction, 2) if avg_satisfaction is not None else None,
                'satisfaction_rate': round(satisfaction_rate, 4)
            }
            variant_results.append(result)
            variant_data_for_stats.append({
                'success_count': success_count,
                'call_count': call_count
            })

        stats_analysis = perform_ab_test_analysis(variant_data_for_stats)

        best_variant = None
        if variant_results:
            scored_variants = [v for v in variant_results if v['avg_satisfaction_score'] is not None]
            if scored_variants:
                best_variant = max(scored_variants, key=lambda v: v['satisfaction_rate'])
            else:
                best_variant = max(variant_results, key=lambda v: v['call_count'])

        total_calls = sum(v['call_count'] for v in variant_results)

        return jsonify({
            'test_id': ab_test.id,
            'test_name': ab_test.test_name,
            'status': ab_test.status,
            'target_type': ab_test.target_type,
            'start_time': ab_test.start_time.isoformat() if ab_test.start_time else None,
            'end_time': ab_test.end_time.isoformat() if ab_test.end_time else None,
            'total_calls': total_calls,
            'variant_results': variant_results,
            'statistical_analysis': stats_analysis,
            'recommended_variant': best_variant['variant_id'] if best_variant else None,
            'recommended_variant_name': best_variant['variant_name'] if best_variant else None
        })

    @app.route('/api/apps/<int:app_id>/call', methods=['POST'])
    def call_app(app_id):
        app = App.query.get_or_404(app_id)
        data = request.get_json()
        user_query = data.get('query')
        if not user_query:
            return jsonify({'error': 'query is required'}), 400

        running_test = ABTest.query.filter_by(app_id=app_id, status='running').first()
        selected_variant = None
        ab_test_id = None
        variant_id = None

        system_prompt = app.system_prompt
        model_id = app.model_id
        temperature = app.temperature
        max_tokens = app.max_tokens

        if running_test and running_test.variants:
            selected_variant = select_variant(running_test.variants)
            ab_test_id = running_test.id
            variant_id = selected_variant.id
            system_prompt = selected_variant.system_prompt or system_prompt
            model_id = selected_variant.model_id or model_id
            temperature = selected_variant.temperature if selected_variant.temperature is not None else temperature
            max_tokens = selected_variant.max_tokens or max_tokens

        start_time = time.time()

        response_text = generate_mock_response(user_query, system_prompt, model_id)

        end_time = time.time()
        response_time_ms = int((end_time - start_time) * 1000)
        tokens_consumed = estimate_tokens(user_query, response_text)

        call_log = CallLog(
            app_id=app_id,
            ab_test_id=ab_test_id,
            variant_id=variant_id,
            user_query=user_query,
            system_prompt=system_prompt,
            model_id=model_id,
            temperature=temperature,
            max_tokens=max_tokens,
            response=response_text,
            response_time_ms=response_time_ms,
            tokens_consumed=tokens_consumed
        )
        db.session.add(call_log)
        db.session.commit()

        return jsonify({
            'response': response_text,
            'call_log_id': call_log.id,
            'ab_test_id': ab_test_id,
            'variant_id': variant_id,
            'variant_name': selected_variant.variant_name if selected_variant else None,
            'model_id': model_id,
            'response_time_ms': response_time_ms,
            'tokens_consumed': tokens_consumed
        })

    @app.route('/api/call-logs/<int:log_id>/feedback', methods=['POST'])
    def submit_feedback(log_id):
        call_log = CallLog.query.get_or_404(log_id)
        data = request.get_json()
        satisfaction_score = data.get('satisfaction_score')
        if satisfaction_score is not None:
            if satisfaction_score < 1 or satisfaction_score > 5:
                return jsonify({'error': 'satisfaction_score must be between 1 and 5'}), 400
            call_log.satisfaction_score = satisfaction_score
        call_log.feedback = data.get('feedback', call_log.feedback)
        db.session.commit()
        return jsonify(call_log.to_dict())

    @app.route('/api/apps/<int:app_id>/call-logs', methods=['GET'])
    def get_call_logs(app_id):
        app = App.query.get_or_404(app_id)
        logs = CallLog.query.filter_by(app_id=app_id).order_by(CallLog.created_at.desc()).limit(100).all()
        return jsonify([log.to_dict() for log in logs])

def select_variant(variants):
    total_percentage = sum(v.traffic_percentage for v in variants)
    random_value = random.uniform(0, total_percentage)
    cumulative = 0
    for variant in variants:
        cumulative += variant.traffic_percentage
        if random_value <= cumulative:
            return variant
    return variants[-1]

def generate_mock_response(query, system_prompt, model_id):
    responses = [
        f"根据您的问题「{query}」，我为您提供以下解答：这是一个关于该主题的详细说明。",
        f"您好！关于「{query}」，我的分析如下：这涉及到多个方面的考量。",
        f"收到您的问题「{query}」。基于我的知识库，我可以告诉您：这是一个很好的问题。",
        f"感谢您的提问！针对「{query}」，我的建议是：需要综合考虑各种因素。"
    ]
    return random.choice(responses)

def estimate_tokens(query, response):
    total_text = query + " " + response
    return len(total_text.split()) * 1.3
