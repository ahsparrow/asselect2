use std::fmt;
use std::fmt::Write;

use chrono::Utc;
use geo::Point;
use geo::geometry::Geometry;
use textwrap::{fill, indent};

use crate::features::AirspaceFeature;
use crate::settings::Settings;

struct MyPoint(Point);

fn degrees_to_dms(degrees: f64) -> (u32, u32, u32) {
    let mut sec = (degrees * 3600.0).round() as u32;
    let mut min = sec / 60;
    sec = sec % 60;
    let deg = min / 60;
    min = min % 60;

    (deg, min, sec)
}

impl fmt::Display for MyPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lon = self.0.x();
        let lat = self.0.y();

        let lat_ns = if lat >= 0.0 { "N" } else { "S" };
        let lon_ew = if lon >= 0.0 { "E" } else { "W" };

        let lat_dms = degrees_to_dms(lat.abs());
        let lon_dms = degrees_to_dms(lon.abs());

        write!(
            f,
            "{:02}:{:02}:{:02} {} {:03}:{:02}:{:02} {}",
            lat_dms.0, lat_dms.1, lat_dms.2, lat_ns, lon_dms.0, lon_dms.1, lon_dms.2, lon_ew
        )
    }
}

fn header(
    buf: &mut String,
    airac_date: &str,
    timestamp: &str,
    user_agent: &str,
    settings: &Settings,
) -> Result<(), fmt::Error> {
    write!(
        buf,
        "* UK Airspace\n\
        * Alan Sparrow (airspace@asselect.uk)\n\
        *\n\
        * I have tried to make this data as accurate as possible but\n\
        * there will still be errors. Don't blame me if you go somewhere you\n\
        * shouldn't have gone.\n\
        *\n\
        * To the extent possible under law, Alan Sparrow has waived all\n\
        * copyright and related or neighbouring rights to this file. The data\n\
        * in this file is sourced from the UK Aeronautical Information\n\
        * Package (AIP).\n\
        *\n\
        * AIRAC: {}\n\
        * Produced: {}\n\
        * User agent: {}\n\
        {}\n",
        airac_date,
        timestamp,
        user_agent,
        &indent(&fill(format!("{:?}", settings).as_str(), 70), "* ")
    )
}

fn format_limit(limit: i32, _uom: &str, reference: &str) -> String {
    match reference {
        "SFC" => "SFC".to_string(),
        "STD" => format!("FL{}", limit),
        "MSL" => format!("{} ft", limit),
        _ => "UNKNOWN".to_string(),
    }
}

fn write_geometry(buf: &mut String, feature: &AirspaceFeature) -> Result<(), fmt::Error> {
    match &feature.geometry {
        Geometry::Point(p) => write!(
            buf,
            "V X={}\n\
             DC {:.2}\n",
            MyPoint(*p),
            &feature.radius.unwrap() / 1852.0
        )?,
        Geometry::Polygon(poly) => {
            for p in poly.exterior().points() {
                write!(buf, "DP {}\n", MyPoint(p))?
            }
        }
        _ => (),
    }
    Ok(())
}

pub fn openair(
    airspace: &Vec<AirspaceFeature>,
    settings: &Settings,
    airac_date: &str,
    user_agent: &str,
) -> Result<String, fmt::Error> {
    let mut s = "".to_string();

    header(
        &mut s,
        airac_date,
        &Utc::now().to_rfc3339(),
        user_agent,
        &settings,
    )?;

    let filtered_airspace = airspace.iter().filter(|_| true);

    for a in filtered_airspace {
        write!(s, "*\n")?;
        write!(s, "AC G\n")?;
        write!(s, "AN {}\n", a.name)?;
        write!(
            s,
            "AL {}\n",
            format_limit(a.lower_limit, &a.lower_limit_uom, &a.lower_limit_reference)
        )?;
        write!(
            s,
            "AH {}\n",
            format_limit(a.upper_limit, &a.upper_limit_uom, &a.upper_limit_reference)
        )?;
        write_geometry(&mut s, &a)?;
    }
    Ok(s)
}
