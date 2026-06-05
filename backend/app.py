import logging
import sys
import socket
import subprocess
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

def get_pid_using_port(port):
    try:
        result = subprocess.run(
            ['netstat', '-ano'],
            capture_output=True,
            text=True,
            timeout=3
        )
        for line in result.stdout.split('\n'):
            if f':{port}' in line and 'LISTENING' in line:
                parts = line.split()
                pid = parts[-1]
                if pid.isdigit():
                    return int(pid)
    except Exception:
        pass
    return None

def is_our_process(pid):
    try:
        result = subprocess.run(
            ['wmic', 'process', 'where', f'ProcessId={pid}', 'get', 'CommandLine'],
            capture_output=True,
            text=True,
            timeout=3
        )
        cmdline = result.stdout.lower()
        return 'app.py' in cmdline or 'python' in cmdline
    except Exception:
        return False

def main():
    port = 5000

    if '--reset-db' in sys.argv:
        logger.info('Resetting database...')
        reset_database()
        logger.info('Database reset complete')
        return

    available, error = is_port_available(port)
    if not available:
        pid = get_pid_using_port(port)
        if pid and is_our_process(pid):
            logger.info('')
            logger.info('========================================')
            logger.info('Server is already running!')
            logger.info('')
            logger.info(f'URL: http://localhost:{port}')
            logger.info(f'PID: {pid}')
            logger.info('')
            logger.info('The backend service is already up and running.')
            logger.info('No need to start another instance.')
            logger.info('========================================')
            logger.info('')
            sys.exit(0)
        else:
            logger.error('')
            logger.error('========================================')
            logger.error(f'ERROR: Port {port} is already in use!')
            if pid:
                logger.error(f'       (Used by process PID: {pid})')
            logger.error('')
            logger.error('Please stop the other process using port 5000,')
            logger.error('or choose a different port manually.')
            logger.error('========================================')
            logger.error('')
            sys.exit(1)

    app = create_app()
    logger.info(f'Server starting on http://localhost:{port}')
    app.run(host='0.0.0.0', port=port, debug=True, use_reloader=False)

if __name__ == '__main__':
    main()
