import json

from flask import Blueprint, current_app, make_response, render_template, request

from asselect.openair import normlevel, openair

bp = Blueprint("blueprint", __name__)


@bp.route("/")
def index():
    settings = json.loads(request.cookies.get("settings", "{}"))

    return render_template(
        "asselect.html",
        settings=settings,
        gliding_sites=current_app.config["GLIDING_SITES"],
        loas=current_app.config["LOAS"],
        rats=current_app.config["RATS"],
        wave_boxes=current_app.config["WAVE_BOXES"],
    )


@bp.route("/download")
def download():
    settings = request.args.to_dict()

    types = {
        "atz": settings["atz"],
        "ils": settings["atz"] if settings["ils"] == "ATZ" else settings["ils"],
        "noatz": settings["noatz"],
        "ul": settings["ul"],
        "glider": settings["glider"],
        "hirta": settings["hirta"],
        "obstacle": settings["obstacle"]
    }

    home = settings["home"]
    max_level = normlevel(settings["maxlevel"])
    append_frequency = settings["radio"] != ""

    rat = [s[4:] for s in settings if s.startswith("rat-")]
    loa = [s[4:] for s in settings if s.startswith("loa-")]
    wave = [s[5:] for s in settings if s.startswith("wave-")]

    oa_data = openair(
        current_app.config["YAIXM"],
        types,
        home=home,
        max_level=max_level,
        append_frequency=append_frequency,
        loa_names=loa,
        rat_names=rat,
        wave_names=wave,
    )

    resp = make_response(oa_data.encode(encoding="ascii"))
    resp.headers["Content-Type"] = "text/plain"
    resp.headers["Content-Disposition"] = "attachment; filename=opeair.txt"
    resp.set_cookie("settings", value=json.dumps(settings), max_age=63072000)

    return resp
