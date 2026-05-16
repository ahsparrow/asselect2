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

// Settings
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    pub atz: String,
    pub ils: Option<String>,
    pub unlicensed: Option<String>,
    pub microlight: Option<String>,
    pub gliding: Option<String>,
    pub home: Option<String>,
    pub hirta_gvs: Option<String>,
    pub obstacle: Option<String>,
    pub max_level: u16,
    pub radio: bool,
    pub format: String,
    pub overlay: Option<String>,
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
            ils: None,
            unlicensed: None,
            microlight: None,
            gliding: None,
            home: None,
            hirta_gvs: None,
            obstacle: None,
            max_level: 660,
            radio: false,
            format: "openair".to_string(),
            overlay: None,
            loa: HashSet::new(),
            rat: HashSet::new(),
            wave: HashSet::new(),
        }
    }
}
