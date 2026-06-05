﻿from flask import Flask
from flask_cors import CORS
from flask_sqlalchemy import SQLAlchemy
from config import Config
import logging

db = SQLAlchemy()

logger = logging.getLogger(__name__)

def create_app(init_db=True):
    app = Flask(__name__)
    app.config.from_object(Config)
    
    CORS(app)
    db.init_app(app)
    
    from app import models
    
    if init_db:
        with app.app_context():
            db.create_all()
            logger.info('Database tables initialized successfully')
    
    from app.controllers import register_routes
    register_routes(app)
    
    return app

def reset_database():
    import os
    db_path = Config.SQLALCHEMY_DATABASE_URI.replace('sqlite:///', '')
    if os.path.exists(db_path):
        os.remove(db_path)
        logger.info(f'Database file removed: {db_path}')
    
    app = create_app(init_db=False)
    with app.app_context():
        db.create_all()
        logger.info('Database recreated successfully')
