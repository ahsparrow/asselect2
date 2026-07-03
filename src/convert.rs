use std::collections::HashSet;
use std::fmt;
use std::fmt::Write;

use chrono::Utc;
use geo::Point;
use geo::geometry::Geometry;
use geojson::feature::Id;
use geojson::{Feature, FeatureCollection, GeometryValue};
use textwrap::{fill, indent};
use uuid::Uuid;

use crate::features::AirspaceFeature;
use crate::settings::Settings;

struct MyPoint(Point);

pub fn normalise_limit(limit: i32, _uom: &str, reference: &str) -> i32 {
    match reference {
        "SFC" => 0,
        "MSL" => limit,
        "STD" => limit * 100,
        _ => 0,
    }
}

fn oa_setting(setting: &str) -> String {
    match setting {
        "classa" => "A".to_string(),
        "classb" => "B".to_string(),
        "classc" => "C".to_string(),
        "classd" => "D".to_string(),
        "classe" => "E".to_string(),
        "classf" => "F".to_string(),
        "classg" => "G".to_string(),
        "ctr" => "CTR".to_string(),
        "danger" => "Q".to_string(),
        "glider" => "W".to_string(),
        "restricted" => "R".to_string(),
        _ => "UNKNOWN".to_string(),
    }
}

pub fn make_air_filter(
    settings: &Settings,
    replace_ids: &HashSet<&Uuid>,
) -> impl Fn(&&AirspaceFeature) -> bool {
    |a: &&AirspaceFeature| {
        let exclude = replace_ids.contains(&a.identifier)
            || a.atype == "GLIDER" && (settings.glider == "no" || settings.home == a.name)
            || a.atype == "MICROLIGHT" && settings.microlight == "no"
            || a.atype == "TRAINING" && settings.unlicensed == "no"
            || vec!["LASER", "GVS", "HIRTA"].contains(&a.atype.as_str())
            || (a.atype == "TRAG" || a.atype == "NSGA") && !settings.wave.contains(&a.name)
            || normalise_limit(a.lower_limit, &a.lower_limit_uom, &a.lower_limit_reference)
                > settings.max_level.parse::<i32>().unwrap_or(0) * 100;

        !exclude
    }
}

pub fn oa_type(feature: &AirspaceFeature, settings: &Settings) -> String {
    match feature.atype.as_str() {
        "ATZ" => oa_setting(&settings.atz),
        "CTA" | "CTR" | "TMA" => feature.classification.as_ref().unwrap().to_string(),
        "D" => {
            if feature.status.as_deref() == Some("ACTIVE") {
                "Q".to_string()
            } else {
                "G".to_string()
            }
        }
        "DZ" => "Q".to_string(),
        "GLIDER" => oa_setting(&settings.glider),
        "GVS" | "HIRTA" | "LASER" => oa_setting(&settings.hirta_gvs),
        "ILS" => match settings.ils.as_str() {
            "asatz" => oa_setting(&settings.atz),
            _ => oa_setting(&settings.ils),
        },
        "MATZ" => "MATZ".to_string(),
        "MICROLIGHT" => oa_setting(&settings.microlight),
        "NGSA" | "TRAG" => "W".to_string(),
        "P" => "P".to_string(),
        "R" => "R".to_string(),
        "RMZ" => "RMZ".to_string(),
        "TMZ" => "TMZ".to_string(),
        "TRAINING" => oa_setting(&settings.unlicensed),

        _ => "UNKNOWN".to_string(),
    }
}

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
        * I have tried my best to make this data accurate but there will still\n\
        * be errors. Don't blame me if you go somewhere you shouldn't have gone.\n\
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
    airspace: &Vec<&AirspaceFeature>,
    settings: &Settings,
    airac_date: &str,
    user_agent: &str,
    oatypes: Vec<String>,
) -> Result<String, fmt::Error> {
    let mut s = "".to_string();

    header(
        &mut s,
        airac_date,
        &Utc::now().to_rfc3339(),
        user_agent,
        &settings,
    )?;

    for (a, oa) in airspace.iter().zip(oatypes.into_iter()) {
        write!(s, "*\n")?;
        write!(s, "AC {}\n", oa)?;
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

pub fn serialize_airspace(features: &Vec<&AirspaceFeature>, oatypes: Vec<String>) -> String {
    let feature_collection: FeatureCollection = features
        .iter()
        .zip(oatypes.into_iter())
        .map(|(f, oa)| {
            let mut jf = Feature::from(GeometryValue::from(&f.geometry));
            jf.id = Some(Id::String(f.identifier.to_string()));
            jf.set_property("name", f.name.as_str());
            jf.set_property("atype", f.atype.as_str());
            if let Some(classification) = &f.classification {
                jf.set_property("classification", classification.as_str());
            }
            jf.set_property("oatype", oa);
            jf.set_property("lowerLimit", f.lower_limit);
            jf.set_property("lowerLimit_uom", f.lower_limit_uom.as_str());
            jf.set_property("lowerLimit_reference", f.lower_limit_reference.as_str());
            jf.set_property("upperLimit", f.upper_limit);
            jf.set_property("upperLimit_uom", f.upper_limit_uom.as_str());
            jf.set_property("upperLimit_reference", f.upper_limit_reference.as_str());
            if let Some(channel) = &f.channel {
                jf.set_property("channel", channel.as_str());
            }
            if let Some(status) = &f.status {
                jf.set_property("status", status.as_str());
            }
            jf
        })
        .collect();
    feature_collection.to_string()
}
