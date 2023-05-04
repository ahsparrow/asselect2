# Copyright 2023 Alan Sparrow
#
# This file is part of ASSelect
#
# ASSelect is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# ASSelect is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with ASSelect.  If not, see <http://www.gnu.org/licenses/>.

import logging

from werkzeug.middleware.proxy_fix import ProxyFix

import asselect.asselect


# Create the application
def create_app():
    return asselect.asselect.create_app()


# Factory function for running with gunicorn
def create_gunicorn_app():
    app = asselect.asselect.create_app()

    gunicorn_logger = logging.getLogger("gunicorn.error")
    app.logger.handlers = gunicorn_logger.handlers
    app.logger.setLevel(gunicorn_logger.level)

    return ProxyFix(app, x_for=1, x_proto=1, x_host=1, x_prefix=1)
