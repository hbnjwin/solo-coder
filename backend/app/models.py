from datetime import datetime
from app import db
import json

class App(db.Model):
    __tablename__ = 'apps'
    
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(100), nullable=False)
    description = db.Column(db.Text)
    system_prompt = db.Column(db.Text)
    model_id = db.Column(db.String(50), default='gpt-3.5-turbo')
    temperature = db.Column(db.Float, default=0.7)
    max_tokens = db.Column(db.Integer, default=1000)
    created_at = db.Column(db.DateTime, default=datetime.utcnow)
    updated_at = db.Column(db.DateTime, default=datetime.utcnow, onupdate=datetime.utcnow)
    
    ab_tests = db.relationship('ABTest', backref='app', lazy=True, cascade='all, delete-orphan')
    call_logs = db.relationship('CallLog', backref='app', lazy=True, cascade='all, delete-orphan')
    
    def to_dict(self):
        return {
            'id': self.id,
            'name': self.name,
            'description': self.description,
            'system_prompt': self.system_prompt,
            'model_id': self.model_id,
            'temperature': self.temperature,
            'max_tokens': self.max_tokens,
            'created_at': self.created_at.isoformat(),
            'updated_at': self.updated_at.isoformat()
        }

class ABTest(db.Model):
    __tablename__ = 'ab_tests'
    
    id = db.Column(db.Integer, primary_key=True)
    app_id = db.Column(db.Integer, db.ForeignKey('apps.id'), nullable=False)
    test_name = db.Column(db.String(100), nullable=False)
    description = db.Column(db.Text)
    target_type = db.Column(db.String(20), nullable=False)
    status = db.Column(db.String(20), default='running')
    start_time = db.Column(db.DateTime, default=datetime.utcnow)
    end_time = db.Column(db.DateTime)
    created_at = db.Column(db.DateTime, default=datetime.utcnow)
    
    variants = db.relationship('ABTestVariant', backref='ab_test', lazy=True, cascade='all, delete-orphan')
    call_logs = db.relationship('CallLog', backref='ab_test', lazy=True)
    
    def to_dict(self):
        return {
            'id': self.id,
            'app_id': self.app_id,
            'test_name': self.test_name,
            'description': self.description,
            'target_type': self.target_type,
            'status': self.status,
            'start_time': self.start_time.isoformat() if self.start_time else None,
            'end_time': self.end_time.isoformat() if self.end_time else None,
            'created_at': self.created_at.isoformat(),
            'variants': [v.to_dict() for v in self.variants]
        }

class ABTestVariant(db.Model):
    __tablename__ = 'ab_test_variants'
    
    id = db.Column(db.Integer, primary_key=True)
    ab_test_id = db.Column(db.Integer, db.ForeignKey('ab_tests.id'), nullable=False)
    variant_name = db.Column(db.String(50), nullable=False)
    is_control = db.Column(db.Boolean, default=False)
    traffic_percentage = db.Column(db.Float, nullable=False)
    system_prompt = db.Column(db.Text)
    model_id = db.Column(db.String(50))
    temperature = db.Column(db.Float)
    max_tokens = db.Column(db.Integer)
    created_at = db.Column(db.DateTime, default=datetime.utcnow)
    
    call_logs = db.relationship('CallLog', backref='variant', lazy=True)
    
    def to_dict(self):
        return {
            'id': self.id,
            'ab_test_id': self.ab_test_id,
            'variant_name': self.variant_name,
            'is_control': self.is_control,
            'traffic_percentage': self.traffic_percentage,
            'system_prompt': self.system_prompt,
            'model_id': self.model_id,
            'temperature': self.temperature,
            'max_tokens': self.max_tokens,
            'created_at': self.created_at.isoformat()
        }

class CallLog(db.Model):
    __tablename__ = 'call_logs'
    
    id = db.Column(db.Integer, primary_key=True)
    app_id = db.Column(db.Integer, db.ForeignKey('apps.id'), nullable=False)
    ab_test_id = db.Column(db.Integer, db.ForeignKey('ab_tests.id'))
    variant_id = db.Column(db.Integer, db.ForeignKey('ab_test_variants.id'))
    user_query = db.Column(db.Text, nullable=False)
    system_prompt = db.Column(db.Text)
    model_id = db.Column(db.String(50), nullable=False)
    temperature = db.Column(db.Float, nullable=False)
    max_tokens = db.Column(db.Integer)
    response = db.Column(db.Text)
    response_time_ms = db.Column(db.Integer)
    tokens_consumed = db.Column(db.Integer, default=0)
    satisfaction_score = db.Column(db.Integer)
    feedback = db.Column(db.Text)
    created_at = db.Column(db.DateTime, default=datetime.utcnow)
    
    def to_dict(self):
        return {
            'id': self.id,
            'app_id': self.app_id,
            'ab_test_id': self.ab_test_id,
            'variant_id': self.variant_id,
            'user_query': self.user_query,
            'system_prompt': self.system_prompt,
            'model_id': self.model_id,
            'temperature': self.temperature,
            'max_tokens': self.max_tokens,
            'response': self.response,
            'response_time_ms': self.response_time_ms,
            'tokens_consumed': self.tokens_consumed,
            'satisfaction_score': self.satisfaction_score,
            'feedback': self.feedback,
            'created_at': self.created_at.isoformat()
        }
