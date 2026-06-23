use geojson::FeatureCollection;
use geojson::de::deserialize_feature_collection_str_to_vec;

#[derive(serde::Deserialize)]
pub struct AirspaceFeature {
    pub name: String,
    pub stype: String,
    //#[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    //geometry: geo::geometry::Geometry,
}

#[derive(serde::Deserialize)]
pub struct LoaFeature {
    pub group_name: String,
    pub aref: Option<String>,
    //#[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    //geometry: geo::geometry::Geometry,
}

#[derive(serde::Deserialize)]
pub struct RatFeature {
    pub group_name: String,
    //#[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    //geometry: geo::geometry::Geometry,
}

pub fn parse_airspace(text: &String) -> (Vec<AirspaceFeature>, String) {
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

    let features =
        deserialize_feature_collection_str_to_vec(text).expect("can't deserialize airspace data");

    (features, airac_date)
}

pub fn parse_loa(text: &String) -> Vec<LoaFeature> {
    deserialize_feature_collection_str_to_vec(text).expect("can't deserialize LOA data")
}

pub fn parse_rat(text: &String) -> Vec<RatFeature> {
    deserialize_feature_collection_str_to_vec(text).expect("can't deserialize RAT data")
}
