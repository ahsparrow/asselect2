import json
import os
import tomllib

from flask import Flask

import asselect.blueprints.index


def load_yaixm(app, yaixm_file):
    yaixm = json.load(open(yaixm_file))

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


def create_app(config):
    # Load config data
    with open(os.getenv(config), "rb") as f:
        config = tomllib.load(f)["flask"]

    app = Flask("asselect")
    load_yaixm(app, config["yaixm_file"])

    app.register_blueprint(asselect.blueprints.index.bp)

    # Remove template whitespace
    app.jinja_env.lstrip_blocks = True
    app.jinja_env.trim_blocks = True

    return app
