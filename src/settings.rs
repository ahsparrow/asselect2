use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// Settings
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    pub atz: String,
    pub ils: String,
    pub unlicensed: String,
    pub microlight: String,
    pub glider: String,
    pub home: String,
    pub hirta_gvs: String,
    pub obstacle: String,
    pub max_level: String,
    pub radio: String,
    pub format: String,
    pub overlay: String,
    #[serde(default)]
    pub loa: HashSet<String>,
    #[serde(default)]
    pub rat: HashSet<String>,
    #[serde(default)]
    pub wave: HashSet<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            atz: "ctr".to_string(),
            ils: "asatz".to_string(),
            unlicensed: "no".to_string(),
            microlight: "no".to_string(),
            glider: "no".to_string(),
            home: "no".to_string(),
            hirta_gvs: "no".to_string(),
            obstacle: "no".to_string(),
            max_level: "660".to_string(),
            radio: "no".to_string(),
            format: "openair".to_string(),
            overlay: "no".to_string(),
            loa: HashSet::new(),
            rat: HashSet::new(),
            wave: HashSet::new(),
        }
    }
}

impl Settings {
    pub fn update(&mut self, name: &str, value: &str) {
        let val = value.to_string();

        match name {
            "atz" => self.atz = val,
            "ils" => self.ils = val,
            "unlicensed" => self.unlicensed = val,
            "microlight" => self.microlight = val,
            "glider" => self.glider = val,
            "hirta_gvs" => self.hirta_gvs = val,
            "obstacle" => self.obstacle = val,
            "format" => self.format = val,
            "max_level" => self.max_level = val,
            "radio" => self.radio = val,
            "overlay" => self.overlay = val,
            "home" => self.home = val,
            _ => (),
        }
    }

    pub fn get(&self, name: &str) -> String {
        match name {
            "atz" => self.atz.clone(),
            "ils" => self.ils.clone(),
            "unlicensed" => self.unlicensed.clone(),
            "microlight" => self.microlight.clone(),
            "glider" => self.glider.clone(),
            "hirta_gvs" => self.hirta_gvs.clone(),
            "obstacle" => self.obstacle.clone(),
            "format" => self.format.clone(),
            "max_level" => self.max_level.clone(),
            "radio" => self.radio.clone(),
            "overlay" => self.overlay.clone(),
            "home" => self.home.clone(),
            _ => "".to_string(),
        }
    }

    pub fn set_loa(&mut self, value: &str) {
        self.loa.insert(value.to_string());
    }

    pub fn unset_loa(&mut self, value: &str) {
        self.loa.remove(value);
    }

    pub fn get_loa(&self) -> &HashSet<String> {
        &self.loa
    }

    pub fn clear_loa(&mut self) {
        self.loa.clear();
    }

    pub fn set_rat(&mut self, value: &str, add: bool) {
        if add {
            self.rat.insert(value.to_string());
        } else {
            self.rat.remove(value);
        }
    }

    pub fn get_rat(&self) -> &HashSet<String> {
        &self.rat
    }

    pub fn clear_rat(&mut self) {
        self.rat.clear();
    }

    pub fn set_wave(&mut self, value: &str, add: bool) {
        if add {
            self.wave.insert(value.to_string());
        } else {
            self.wave.remove(value);
        }
    }

    pub fn get_wave(&self) -> &HashSet<String> {
        &self.wave
    }

    pub fn clear_wave(&mut self) {
        self.wave.clear();
    }
}
