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
    //#[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    //geometry: geo::geometry::Geometry,
}

#[derive(serde::Deserialize)]
pub struct RatFeature {
    pub group_name: String,
    //#[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    //geometry: geo::geometry::Geometry,
}

pub fn parse_airspace(text: &String) -> Vec<AirspaceFeature> {
    let io_reader = std::io::BufReader::new(text.as_bytes());
    let feature_reader = geojson::FeatureReader::from_reader(io_reader);

    feature_reader
        .deserialize::<AirspaceFeature>()
        .unwrap()
        .into_iter()
        .map(|f| f.expect("valid airspace feature"))
        .collect()
}

pub fn parse_loa(text: &String) -> Vec<LoaFeature> {
    let io_reader = std::io::BufReader::new(text.as_bytes());
    let feature_reader = geojson::FeatureReader::from_reader(io_reader);

    feature_reader
        .deserialize::<LoaFeature>()
        .unwrap()
        .into_iter()
        .map(|f| f.expect("valid LOA feature"))
        .collect()
}

pub fn parse_rat(text: &String) -> Vec<RatFeature> {
    let io_reader = std::io::BufReader::new(text.as_bytes());
    let feature_reader = geojson::FeatureReader::from_reader(io_reader);

    feature_reader
        .deserialize::<RatFeature>()
        .unwrap()
        .into_iter()
        .map(|f| f.expect("valid RAT feature"))
        .collect()
}
