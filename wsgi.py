from werkzeug.middleware.proxy_fix import ProxyFix

from asselect.asselect import create_app

# Create the application
app = create_app("ASSELECT_CONFIG")

# Fix to run behind NGINX reverse-proxy
nginx_app = ProxyFix(app, x_for=1, x_proto=1, x_host=1, x_prefix=1)
