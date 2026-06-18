import logging
import sys
import socket
from app import create_app, reset_database

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


def is_port_available(port):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        sock.bind(('0.0.0.0', port))
        sock.close()
        return True, None
    except socket.error as e:
        sock.close()
        return False, str(e)


def main():
    port = 5000
    if '--reset-db' in sys.argv:
        reset_database()
        logger.info('Database reset complete')
        return
    available, error = is_port_available(port)
    if not available:
        logger.error('Port %s is already in use: %s', port, error)
        sys.exit(1)
    app = create_app()
    logger.info('Server starting on http://localhost:%s', port)
    app.run(host='0.0.0.0', port=port, debug=True, use_reloader=False)


if __name__ == '__main__':
    main()
