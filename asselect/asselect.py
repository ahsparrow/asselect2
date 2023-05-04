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

import json
import os

from flask import Flask

import asselect.blueprint


def load_config(app):
    app.config.from_prefixed_env()

    yaixm = json.load(open(app.config["YAIXM_FILE"]))
    app.config["YAIXM"] = yaixm
    app.config["RATS"] = [rat["name"] for rat in yaixm["rat"]]
    app.config["LOAS"] = [loa["name"] for loa in yaixm["loa"] if not loa.get("default")]
    app.config["WAVE_BOXES"] = sorted(
        [
            feature["name"]
            for feature in yaixm["airspace"]
            if feature["type"] == "D_OTHER" and feature.get("localtype") == "GLIDER"
        ]
    )
    app.config["GLIDING_SITES"] = sorted(
        [
            feature["name"]
            for feature in yaixm["airspace"]
            if feature["type"] == "OTHER" and feature.get("localtype") == "GLIDER"
        ]
    )
    app.config["AIRAC_DATE"] = yaixm["release"]["airac_date"][:10]
    app.config["RELEASE_TEXT"] = yaixm["release"]["note"]


def create_app():
    app = Flask("asselect")
    load_config(app)

    app.register_blueprint(asselect.blueprint.bp)

    # Remove template whitespace
    app.jinja_env.lstrip_blocks = True
    app.jinja_env.trim_blocks = True

    return app
