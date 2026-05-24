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

pub fn option_tab() -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");
    let getter = use_context::<ReadSignal<Settings>>().expect("to find getter");

    div().child(div().class("box").child((
        div().class("columns").child((
            div().class("column is-one-third").child(select_field(
                setter,
                getter,
                "Format",
                "format",
                &vec!["OpenAir", "RA(T) Only", "Competition"],
                &vec!["openair", "ratonly", "competition"],
            )),
            div().class("column is-one-third").child(select_field(
                setter,
                getter,
                "Maximum Level",
                "max_level",
                &vec!["Unlimited", "FL195", "FL125", "FL105", "FL65"],
                &vec!["660", "195", "125", "105", "65"],
            )),
        )),
        div().class("columns").child((
            div().class("column is-one-third").child(select_field(
                setter,
                getter,
                "HIRTA/GVS",
                "hirta_gvs",
                &vec!["No", "Danger", "Restricted"],
                &vec!["no", "danger", "restricted"],
            )),
            div().class("column is-one-third").child(select_field(
                setter,
                getter,
                "Obstacle",
                "obstacle",
                &vec!["No", "Danger", "Class F", "Class G"],
                &vec!["no", "danger", "classf", "classg"],
            )),
        )),
        div().class("columns").child((
            div().class("column is-one-third").child(select_field(
                setter,
                getter,
                "Radio Frequency",
                "radio",
                &vec!["No", "Add to name"],
                &vec!["no", "yes"],
            )),
            div().class("column is-one-third").child(select_field(
                setter,
                getter,
                "Altitude Overlay",
                "overlay",
                &vec![
                    "No",
                    "Bases to FL195",
                    "Bases to FL105",
                    "Bases to FL105 and ATZ/DZ",
                    "Bases to FL105 and ATZ/DZ (Overlay only)",
                ],
                &vec!["no", "fl195", "fl105", "atzdz"],
            )),
        )),
    )))
}
