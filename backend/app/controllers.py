from flask import Blueprint, jsonify
from app import margin as domain

bp = Blueprint('api', __name__, url_prefix='/api')


@bp.get('/health')
def health():
    return jsonify({'status': 'ok', 'module': 'margin'})


@bp.get('/demo')
def demo():
    try:
        return jsonify({'ok': True, 'data': domain.demo()})
    except Exception as exc:  # surface base errors clearly
        return jsonify({'ok': False, 'error': str(exc)}), 500
