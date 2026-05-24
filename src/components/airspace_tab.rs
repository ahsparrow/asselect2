// Copyright 2024, Alan Sparrow
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
use leptos::html::div;
use leptos::prelude::*;

use crate::components::select_field::select_field;
use crate::settings::Settings;

pub fn airspace_tab(gliding_sites: Vec<String>) -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");
    let getter = use_context::<ReadSignal<Settings>>().expect("to find getter");

    let gsite_options: Vec<&str> = vec!["No"]
        .into_iter()
        .chain(gliding_sites.iter().map(AsRef::as_ref))
        .collect();
    let gsite_values: Vec<&str> = vec!["no"]
        .into_iter()
        .chain(gliding_sites.iter().map(AsRef::as_ref))
        .collect();

    div().child(div().class("box").child((
        div().class("columns").child((
            div().class("column is-one-third").child(select_field(
                setter,
                getter,
                "ATZ",
                "atz",
                &vec!["Class D", "Control Zone"],
                &vec!["classd", "ctr"],
            )),
            div().class("column is-one-third").child(select_field(
                setter,
                getter,
                "ILS Feather",
                "ils",
                &vec!["As ATZ", "Class F", "Class G"],
                &vec!["asatz", "classf", "classg"],
            )),
        )),
        div().class("columns").child((
            div().class("column is-one-third").child(select_field(
                setter,
                getter,
                "Non-ATZ Airfield",
                "unlicensed",
                &vec!["No", "Class F", "Class G"],
                &vec!["no", "classf", "classg"],
            )),
            div().class("column is-one-third").child(select_field(
                setter,
                getter,
                "Microlight Airfield",
                "microlight",
                &vec!["No", "Class F", "Class G"],
                &vec!["no", "classf", "classg"],
            )),
        )),
        div().class("columns").child((
            div().class("column is-one-third").child(select_field(
                setter,
                getter,
                "Glider Airfield",
                "glider",
                &vec!["No", "Gliding Sector", "Class F", "Class G"],
                &vec!["no", "glider", "classf", "classg"],
            )),
            div().class("column is-one-third").child(select_field(
                setter,
                getter,
                "Exclude Home Airfield",
                "home",
                &gsite_options,
                &gsite_values,
            )),
        )),
    )))
}
