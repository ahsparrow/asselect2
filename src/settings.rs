// Copyright 2026, Alan Sparrow
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExtraType {
    Rat,
    Loa,
    Wave,
}

// Settings
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    pub atz: String,
    pub ils: String,
    pub unlicensed: String,
    pub microlight: String,
    pub gliding: String,
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
            gliding: "no".to_string(),
            home: "".to_string(),
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
            "gliding" => self.gliding = val,
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
            "gliding" => self.gliding.clone(),
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

    pub fn set_extra(&mut self, id: ExtraType, value: &str, add: bool) {
        let x = match id {
            ExtraType::Rat => &mut self.rat,
            ExtraType::Loa => &mut self.loa,
            ExtraType::Wave => &mut self.wave,
        };

        if add {
            x.insert(value.to_string());
        } else {
            x.remove(value);
        }
    }

    pub fn get_extra(&self, id: ExtraType) -> &HashSet<String> {
        match id {
            ExtraType::Rat => &self.rat,
            ExtraType::Loa => &self.loa,
            ExtraType::Wave => &self.wave,
        }
    }

    pub fn clear_extra(&mut self, id: ExtraType) {
        match id {
            ExtraType::Rat => &self.rat.clear(),
            ExtraType::Loa => &self.loa.clear(),
            ExtraType::Wave => &self.wave.clear(),
        };
    }
}
