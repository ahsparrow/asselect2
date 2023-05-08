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

import datetime
import json
import textwrap

from flask import Blueprint, current_app, make_response, render_template, request

from asselect.openair import normlevel, openair

HEADER = """UK Airspace
Alan Sparrow (airspace@asselect.uk)

I have tried to make this data as accurate as possible but
there will still be errors. Don't blame me if you go somewhere you
should not have gone while using this data.

To the extent possible under law, Alan Sparrow has waived all
copyright and related or neighbouring rights to this file. The data
in this file is based on the work of others including: George Knight,
Geoff Brown, Peter Desmond and Rory O'Connor.  The data is originally
sourced from the UK Aeronautical Information Package (AIP).
"""

SCHEMA_VERSION = "1"

bp = Blueprint("blueprint", __name__)


@bp.route("/")
def index():
    schema = request.cookies.get("schema", "")
    if schema == SCHEMA_VERSION:
        settings = json.loads(request.cookies.get("settings", "{}"))
    else:
        settings = {}

    return render_template(
        "asselect.html",
        settings=settings,
        gliding_sites=current_app.config["GLIDING_SITES"],
        loas=current_app.config["LOAS"],
        rats=current_app.config["RATS"],
        wave_boxes=current_app.config["WAVE_BOXES"],
        airac_date=current_app.config["AIRAC_DATE"],
        release_text=current_app.config["RELEASE_TEXT"],
    )


@bp.route("/download")
def download():
    settings = request.args.to_dict()
    current_app.logger.info(f"Download - [{request.remote_addr}] {settings}")

    # Openair settings
    types = {
        "atz": settings["atz"],
        "ils": settings["atz"] if settings["ils"] == "ATZ" else settings["ils"],
        "noatz": settings["noatz"],
        "ul": settings["ul"],
        "glider": settings["glider"],
        "hirta": settings["hirta"],
        "obstacle": settings["obstacle"],
    }

    home = settings["home"]
    max_level = normlevel(settings["maxlevel"])
    append_frequency = settings["radio"] != ""
    format = {
        "OPENAIR": "openair",
        "RATONLY": "rat_only",
        "COMPETITION": "competition",
    }[settings["format"]]

    rat = [s[4:] for s in settings if s.startswith("rat-")]
    wave = [s[5:] for s in settings if s.startswith("wave-")]
    loa = [s[4:] for s in settings if s.startswith("loa-")]

    # Add default LOAs
    loa.extend(
        [
            loa["name"]
            for loa in current_app.config["YAIXM"]["loa"]
            if loa.get("default")
        ]
    )

    # Create OpenAir data
    oa_data = openair(
        current_app.config["YAIXM"],
        types,
        home=home,
        max_level=max_level,
        append_frequency=append_frequency,
        format=format,
        loa_names=loa,
        rat_names=rat,
        wave_names=wave,
    )

    # Add the header
    hdr = HEADER
    hdr += f"\n{current_app.config['RELEASE_TEXT']}\n"
    hdr += f"AIRAC: {current_app.config['AIRAC_DATE']}\n"

    now = datetime.datetime.now(datetime.timezone.utc)
    now = now.replace(microsecond=0)
    hdr += f"Produced by asselect.uk: {now.isoformat()}\n"

    commit = current_app.config["YAIXM"]["release"].get("commit", "Unknown")
    hdr += f"Commit: {commit}\n"
    hdr += "\n".join(textwrap.wrap(str(settings)))

    hdr = "\n".join(["* " + line if line else "*" for line in hdr.splitlines()])

    data = hdr + "\n" + oa_data

    # Add overlay text
    if settings["overlay"] == "FL105":
        with open(current_app.config["OVERLAY_105"]) as f:
            data += f.read()
    elif settings["overlay"] == "FL195":
        with open(current_app.config["OVERLAY_195"]) as f:
            data += f.read()
    elif settings["overlay"] == "ATZDZ":
        with open(current_app.config["OVERLAY_ATZDZ"]) as f:
            data += f.read()

    # Generate response
    resp = make_response(data.encode(encoding="ascii"))
    resp.headers["Content-Type"] = "text/plain"
    resp.headers["Content-Disposition"] = "attachment; filename=openair.txt"
    resp.set_cookie("settings", value=json.dumps(settings), max_age=63072000)
    resp.set_cookie("schema", value=SCHEMA_VERSION, max_age=63072000)
    return resp
