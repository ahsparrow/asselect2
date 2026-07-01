use geo::geometry::Geometry;
use geojson::FeatureCollection;
use geojson::de::deserialize_feature_collection_str_to_vec;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AirspaceFeature {
    #[serde(skip_serializing)]
    pub identifier: Uuid,
    #[serde(skip_serializing)]
    pub aref: Option<Uuid>,
    #[serde(alias = "loa_name", alias = "rat_name", skip_serializing)]
    pub group_name: Option<String>,
    pub name: String,
    pub atype: String,
    pub classification: Option<String>,
    #[serde(rename = "lowerLimit")]
    pub lower_limit: i32,
    #[serde(rename = "lowerLimit_uom")]
    pub lower_limit_uom: String,
    #[serde(rename = "lowerLimitReference")]
    pub lower_limit_reference: String,
    #[serde(rename = "upperLimit")]
    pub upper_limit: i32,
    #[serde(rename = "upperLimit_uom")]
    pub upper_limit_uom: String,
    #[serde(rename = "upperLimitReference")]
    pub upper_limit_reference: String,
    pub radius: Option<f64>,
    #[serde(
        deserialize_with = "geojson::de::deserialize_geometry",
        serialize_with = "geojson::ser::serialize_geometry"
    )]
    pub geometry: Geometry,
}

pub fn parse_airspace(text: &String) -> (Vec<AirspaceFeature>, String) {
    // Parse feature collection to get AIRAC date
    let fc = text
        .parse::<FeatureCollection>()
        .expect("invalid airspace GeoJSON");
    let airac_date = fc
        .foreign_members
        .expect("missing foreign members")
        .get("airac_date")
        .expect("missing AIRAC date")
        .as_str()
        .unwrap()
        .to_string();

    // Deserialize to airspace feqture vector
    let features =
        deserialize_feature_collection_str_to_vec(text).expect("can't deserialize airspace data");

    (features, airac_date)
}

pub fn serialize_airspace(features: &Vec<AirspaceFeature>) -> String {
    geojson::ser::to_feature_collection_string(features).unwrap()
}

pub fn parse_loa(text: &String) -> Vec<AirspaceFeature> {
    deserialize_feature_collection_str_to_vec(text).expect("can't deserialize LOA data")
}

pub fn parse_rat(text: &String) -> Vec<AirspaceFeature> {
    deserialize_feature_collection_str_to_vec(text).expect("can't deserialize RAT data")
}
