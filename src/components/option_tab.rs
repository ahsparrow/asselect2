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
                &vec![
                    "OpenAir",
                    "RA(T) Only",
                    "Overlay Only",
                    "Competition",
                    "GeoJSON",
                ],
                &vec!["openair", "ratonly", "overlay", "competition", "geojson"],
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
