import multiprocessing
from pathlib import Path

bind = "127.0.0.1:8001"
workers = multiprocessing.cpu_count() + 1
loglevel = "info"
errorlog = str(Path.home() / "log" / "asselect.log")
wsgi_app = "wsgi:create_gunicorn_app"

