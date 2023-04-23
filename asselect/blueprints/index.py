import json

from flask import Blueprint, current_app, make_response, render_template, request

bp = Blueprint("index", __name__)


@bp.route("/")
def index():
    settings = json.loads(request.cookies.get("settings", "{}"))

    return render_template(
        "asselect.html",
        settings=settings,
        gliding_sites=current_app.config["GLIDING_SITES"],
        rats=current_app.config["RATS"],
    )


@bp.route("/download")
def download():
    settings = request.args.to_dict()
    print(settings)

    dos_data = "FOOBAR"
    resp = make_response(dos_data.encode(encoding="ascii"))
    resp.headers["Content-Type"] = "text/plain"
    resp.headers["Content-Disposition"] = "attachment; filename=foobar.txt"
    resp.set_cookie("settings", value=json.dumps(settings), max_age=63072000)

    return resp
