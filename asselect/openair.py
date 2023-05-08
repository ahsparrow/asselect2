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

import re

# Openair lat/lon format
LATLON_FMT = (
    "{0[d]:02d}:{0[m]:02d}:{0[s]:02d} {0[ns]} {1[d]:03d}:{1[m]:02d}:{1[s]:02d} {1[ew]}"
)

# Latitude/longitude regex, pattern is: [D]DDMMSS[.s[s[s]]]H
DDMMSSH_PATTERN = "(?P<d>[0-9]{2}|[01][0-9]{2})(?P<m>[0-5][0-9])(?P<s>[0-5][0-9](\.[0-9]{1,3})?)(?P<h>[NESW])"
DDMMSSH_RE = re.compile(DDMMSSH_PATTERN)


# Return DMS values for floating point degrees value
def dms(deg):
    if deg <= 0:
        ns = "S"
        ew = "W"
        deg = -deg
    else:
        ns = "N"
        ew = "E"

    secs = round(deg * 3600)
    mins, secs = divmod(secs, 60)
    degs, mins = divmod(mins, 60)
    return {"d": degs, "m": mins, "s": secs, "ns": ns, "ew": ew}


# Convert latitude or longitude string to floating point degrees
def parse_ddmmssh(deg_str):
    m = DDMMSSH_RE.match(deg_str)
    if m is None:
        return None

    deg = int(m.group("d")) + int(m.group("m")) / 60 + float(m.group("s")) / 3600
    if m.group("h") in "SW":
        deg = -deg
    return deg


# Return "normalised" level from SFC, altitude or flight level
def normlevel(value):
    if value.startswith("FL"):
        return int(value[2:]) * 100
    elif value.endswith("ft"):
        return int(value.split()[0])
    else:
        return 0


def level(level_str):
    if level_str.endswith("ft"):
        # Altitude
        return level_str[:-3] + "ALT"
    else:
        # SFC or FL
        return level_str


def latlon(latlon_str):
    lat, lon = [parse_ddmmssh(d) for d in latlon_str.split()]
    return LATLON_FMT.format(dms(lat), dms(lon))


# Load and flatten airspace data
def load_airspace(data):
    airspace = [
        {
            "boundary": volume["boundary"],
            "class": volume.get("class") or feature.get("class"),
            "feature_id": feature.get("id"),
            "feature_name": feature["name"],
            "id": volume.get("id"),
            "localtype": feature.get("localtype"),
            "lower": volume["lower"],
            "name": volume.get("name"),
            "normlower": normlevel(volume["lower"]),
            "rules": ",".join(feature.get("rules", []) + volume.get("rules", [])),
            "seqno": str(s)
            if (s := volume.get("seqno"))
            else "ABCDEFGHIJKLM"[n]
            if len(feature["geometry"]) > 1
            else None,
            "type": feature["type"],
            "upper": volume["upper"],
        }
        for feature in data
        for n, volume in enumerate(feature["geometry"])
    ]
    return airspace


def load_services(data):
    services = {
        control: service["frequency"]
        for service in data
        for control in service["controls"]
    }
    return services


def merge_loa(airspace, loa_data):
    # Add new features
    add = [load_airspace(area["add"]) for loa in loa_data for area in loa["areas"]]
    flat_add = [item for sublist in add for item in sublist]
    airspace.extend(flat_add)

    # Replace existing volumes
    replace_vols = []
    for loa in loa_data:
        for area in loa["areas"]:
            for replace in area.get("replace", []):
                # Find volume to be replaced
                for vol in airspace:
                    if vol.get("id") == replace["id"]:
                        break

                # Delete old volume
                airspace.remove(vol)

                # Make new volumes
                for v in replace["geometry"]:
                    vol["boundary"] = v["boundary"]
                    vol["upper"] = v["upper"]
                    vol["lower"] = v["lower"]
                    airspace.append(vol.copy())

    return airspace


def load_obstacles(data):
    obstacles = [
        {
            "boundary": [
                {"circle": {"centre": obstacle["position"], "radius": "0.5 nm"}}
            ],
            "name": obstacle["name"],
            "normlower": 0,
            "lower": "SFC",
            "upper": obstacle["elevation"],
            "type": "D_OTHER",
            "localtype": "OBSTACLE",
            "rules": [],
        }
        for obstacle in data
    ]

    return obstacles


def namer(volume, append_freq, append_seqno):
    if volume["name"]:
        name = volume["name"]
    else:
        name = volume["feature_name"]

        if localtype := volume["localtype"]:
            if localtype in ["NOATZ", "UL"]:
                name += " A/F"
            elif localtype in ["MATZ", "DZ", "GVS", "HIRTA", "ILS", "LASER"]:
                name += " " + localtype

        elif volume["type"] == "ATZ":
            name += " ATZ"

        elif "RAZ" in volume["rules"]:
            name += " " + "RAZ"

        if append_seqno and (seqno := volume["seqno"]):
            name += f"-{seqno}"

        qualifiers = [q for q in ["SI", "NOTAM"] if q in volume["rules"]]
        if qualifiers:
            name += f" ({'/'.join(qualifiers)})"

    if append_freq and volume.get("frequency"):
        name += f" {volume['frequency']:.03f}"

    return name


def typer(volume, types, format):
    if "NOTAM" in volume["rules"]:
        out = "G"
    elif "TMZ" in volume["rules"]:
        out = "TMZ"
    elif "RMZ" in volume["rules"]:
        out = "RMZ"
    else:
        comp = format == "competition"
        vol_type = volume["type"]
        if vol_type == "ATZ":
            out = types["atz"]
        elif vol_type == "D":
            out = "P" if comp and "SI" in volume["rules"] else "Q"
        elif vol_type == "D_OTHER":
            if volume["localtype"] == "GLIDER":
                out = "W"
            elif comp and volume["localtype"] == "DZ" and "INTENSE" in volume["rules"]:
                out = "P"
            elif volume["localtype"] in ["HIRTA", "GVS", "LASER"]:
                out = types["hirta"]
            elif volume["localtype"] == "OBSTACLE":
                out = types["obstacle"]
            else:
                out = "Q"
        elif vol_type == "OTHER":
            vol_localtype = volume["localtype"]
            if vol_localtype == "GLIDER":
                out = "W" if "LOA" in volume["rules"] else types["glider"]
            elif vol_localtype in ["ILS", "NOATZ", "UL"]:
                out = types[vol_localtype.lower()]
            elif vol_localtype in ["MATZ", "TMZ", "RMZ"]:
                out = vol_localtype
            elif vol_localtype == "RAT":
                out = "P"
            else:
                out = "OTHER"
        elif vol_type in ["P", "R", "TMZ", "RMZ"]:
            out = vol_type
        else:
            out = volume["class"]

    return out


def make_filter(types, max_level, home, wave):
    def func(data):
        exc = False

        # Training airfields
        exc = exc or data["localtype"] == "NOATZ" and not types["noatz"]

        # Microlight strips
        exc = exc or data["localtype"] == "UL" and not types["ul"]

        # HIRTAs, etc
        exc = exc or (
            data["localtype"] in ["HIRTA", "GVS", "LASER"] and not types["hirta"]
        )

        # Gliding sites
        exc = exc or (
            data["type"] == "OTHER"
            and data["localtype"] == "GLIDER"
            and not "LOA" in data["rules"]
            and (not types["glider"] or home == data["feature_name"])
        )

        # Maximum level
        exc = exc or data["normlower"] >= max_level

        # Wave boxes (excluded by default)
        exc = exc or (
            data["type"] == "D_OTHER"
            and data["localtype"] == "GLIDER"
            and "LOA" not in data["rules"]
            and data["feature_name"] not in wave
        )

        return not exc

    return func


def openair_type(vol, types, format):
    oa_type = typer(vol, types, format)
    yield f"AC {oa_type}"


def openair_name(vol, append_freq, format):
    name = namer(vol, append_freq, format == "competition")
    yield f"AN {name}"


def openair_frequency(vol):
    freq = vol.get("frequency")
    if freq:
        yield (f"AF {freq:.03f}")


def openair_point(point):
    yield f"DP {latlon(point)}"


def openair_circle(circle):
    centre = circle["centre"]
    radius = circle["radius"].split()[0]

    yield f"V X={latlon(centre)}"
    yield f"DC {radius}"


def openair_arc(arc, prev):
    yield "V D=+" if arc["dir"] == "cw" else "V D=-"
    yield f"V X={latlon(arc['centre'])}"
    yield f"DB {latlon(prev)}, {latlon(arc['to'])}"


def openair_boundary(boundary):
    first_point = None
    if "line" in boundary[0]:
        first_point = boundary[0]["line"][0]

    for segment in boundary:
        if "line" in segment:
            for point in segment["line"]:
                yield from openair_point(point)
            last_point = segment["line"][-1]

        elif "circle" in segment:
            yield from openair_circle(segment["circle"])

        elif "arc" in segment:
            yield from openair_arc(segment["arc"], last_point)
            last_point = segment["arc"]["to"]

    # Close the polygon if necessary
    if first_point and first_point != last_point:
        yield from openair_point(first_point)


def openair_generator(airspace, types, format, append_frequency):
    for a in airspace:
        yield "*"
        yield from openair_type(a, types, format)
        yield from openair_name(a, append_frequency, format)
        yield from openair_frequency(a)
        yield f"AL {level(a['lower'])}"
        yield f"AH {level(a['upper'])}"
        yield from openair_boundary(a["boundary"])


def openair(
    data,
    types,
    format="openair",
    home="",
    max_level=19500,
    append_frequency=False,
    loa_names=[],
    wave_names=[],
    rat_names=[]
):
    rats = load_airspace(data["rat"])
    if format == "rat_only":
        # Only return selected RATs
        airspace = [rat for rat in rats if rat["feature_name"] in rat_names]
    else:
        airspace = load_airspace(data["airspace"])
        services = load_services(data["service"])

        # Merge selected LOAs
        loa_data = [loa for loa in data["loa"] if loa["name"] in loa_names]
        airspace = merge_loa(airspace, loa_data)

        # Add RATs
        airspace.extend([rat for rat in rats if rat["feature_name"] in rat_names])

        # Add obstacles
        if types.get("obstacle"):
            obstacles = load_obstacles(data["obstacle"])
            airspace.extend(obstacles)

        # Filter airspace
        airspace = list(
            filter(make_filter(types, max_level, home, wave_names), airspace)
        )

        # Merge frequencies
        for volume in airspace:
            if frequency := services.get(volume.get("feature_id")):
                volume["frequency"] = frequency

    return "".join(
        f"{line}\n"
        for line in openair_generator(airspace, types, format, append_frequency)
    )

    return oa


def default_openair(data):
    types = {
        "atz": Type.CTR,
        "ils": Type.G,
        "noatz": Type.G,
        "ul": None,
        "hirta": None,
        "glider": Type.W,
    }
    loa_names = [loa["name"] for loa in data["loa"] if loa.get("default")]
    return openair(data, types, append_frequency=True, loa_names=loa_names)


if __name__ == "__main__":
    import argparse
    import json
    import sys

    parser = argparse.ArgumentParser()
    parser.add_argument(
        "yaixm_file",
        help="JSON input file (default stdin)",
        type=argparse.FileType("rt"),
        default=sys.stdin,
        nargs="?",
    )
    parser.add_argument(
        "openair_file",
        help="OpenAir output file (default stdout)",
        type=argparse.FileType("wt"),
        default=sys.stdout,
        nargs="?",
    )
    args = parser.parse_args()

    yaixm = json.load(args.yaixm_file)

    types = {
        "atz": "CTR",
        "ils": "G",
        "noatz": "F",
        "ul": "",
        "hirta": "",
        "glider": "W",
        "obstacle": "G",
    }

    rat_names = []
    wave_names = []
    loa_names = []

    oa = openair(
        yaixm,
        types,
        max_level=19500,
        append_frequency=True,
        home="RIVAR HILL",
        loa_names=loa_names,
        rat_names=rat_names,
        wave_names=wave_names,
    )

    args.openair_file.write(oa)
