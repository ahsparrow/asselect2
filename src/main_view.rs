use std::collections::{BTreeMap, HashMap, HashSet};

use codee::string::JsonSerdeCodec;
use gloo::file::{Blob, ObjectUrl};
use leptos::ev;
use leptos::html::{A, a, button, div, header};
use leptos::prelude::*;
use leptos::web_sys;
use leptos_use::storage::use_local_storage;
use uuid::Uuid;

use crate::components::{
    about_tab::about_tab, airspace_tab::airspace_tab, extra_tab::extra_tab, loa_panel::loa_panel,
    notam_tab::notam_tab, option_tab::option_tab, rat_panel::rat_panel, tabs::tabs,
    wave_panel::wave_panel,
};
use crate::convert::{make_air_filter, oa_type, openair, serialize_airspace};
use crate::features::{AirspaceFeature, parse_obstacle};
use crate::settings::Settings;

#[derive(Clone, Debug)]
pub struct OverlayData {
    pub overlay_195: Option<String>,
    pub overlay_105: Option<String>,
    pub overlay_atzdz: Option<String>,
}

pub fn main_view(
    airspace_features: Vec<AirspaceFeature>,
    loa_features: Vec<AirspaceFeature>,
    rat_features: Vec<AirspaceFeature>,
    airac_date: String,
    overlay: LocalResource<OverlayData>,
    obstacle: LocalResource<Option<String>>,
) -> impl IntoView {
    let glider_names = get_glider_names(&airspace_features);
    let exclusive_loas = get_loa_names(&loa_features);
    let rat_names = get_rat_names(&rat_features);
    let mut wave_names = get_wave_names(&airspace_features);
    wave_names.sort();

    // Local settings storage
    let (local_settings, set_local_settings, _) =
        use_local_storage::<Settings, JsonSerdeCodec>("settings");

    // Make copy of settings so store value is only updated on download
    let (settings, set_settings) = signal(local_settings.get_untracked());
    provide_context(settings);
    provide_context(set_settings);

    let download_node_ref = NodeRef::<A>::new();

    // Download button callback
    let airac_date_string = airac_date.clone();
    let download = move |_| {
        let untracked_settings = settings.get_untracked();

        // Store settings
        set_local_settings.set(untracked_settings.clone());

        // Filter LOAs
        let mut loa: Vec<&AirspaceFeature> = loa_features
            .iter()
            .filter(|a| {
                untracked_settings
                    .get_loa()
                    .contains(a.group_name.as_ref().unwrap())
            })
            .collect();

        // Airspace volumes to be replaced by LOAs
        let replace_arefs: HashSet<&Uuid> = loa
            .iter()
            .filter(|a| a.aref.is_some())
            .map(|a| a.aref.as_ref().unwrap())
            .collect();

        // Filter airspace
        let mut airspace: Vec<&AirspaceFeature> = airspace_features
            .iter()
            .filter(make_air_filter(&untracked_settings, &replace_arefs))
            .collect();

        // Append LOAs
        airspace.append(&mut loa);

        // Filter and append RA(T)s
        let rat = rat_features.iter().filter(|a| {
            untracked_settings
                .get_rat()
                .contains(a.group_name.as_ref().unwrap())
        });

        if untracked_settings.format == "ratonly" {
            airspace = rat.collect();
        } else {
            airspace.extend(rat);
        }

        // Append obstacles
        let mut obstacle_features: Vec<AirspaceFeature> = vec![];
        if untracked_settings.obstacle != "no" {
            if let Some(obstacle_response) = obstacle.get() {
                if let Some(data) = obstacle_response {
                    obstacle_features.append(&mut parse_obstacle(data.as_str()));
                    airspace.extend(obstacle_features.iter());
                }
            }
        }

        // OpenAir types
        let oatypes: Vec<String> = airspace
            .iter()
            .map(|a| oa_type(a, &untracked_settings))
            .collect();

        let a = download_node_ref.get().unwrap();
        if untracked_settings.format == "geojson" {
            let blob = Blob::new(serialize_airspace(&airspace, oatypes).as_str());
            let object_url = ObjectUrl::from(blob);

            a.set_download("asselect.geojson");
            a.set_href(&object_url);
        } else {
            // Browser user agent
            let user_agent = web_sys::window()
                .and_then(|w| w.navigator().user_agent().ok())
                .unwrap_or_default();

            // make openair data
            let openair_data = match untracked_settings.format.as_str() {
                "openair" | "competition" | "ratonly" => {
                    leptos::logging::log!("openair");
                    openair(
                        &airspace,
                        &untracked_settings,
                        &airac_date_string,
                        &user_agent,
                        oatypes,
                    )
                    .unwrap_or("* ERROR formatting OpenAir data\n".to_string())
                }
                "overlay" => "".to_string(),
                _ => "* ERROR unknown format option\n".to_string(),
            };

            // Get overlay data
            let overlay_data =
                if untracked_settings.overlay != "no" && untracked_settings.format != "ratonly" {
                    if let Some(overlay_data) = overlay.get() {
                        let x = match untracked_settings.overlay.as_str() {
                            "fl195" => overlay_data.overlay_195.clone(),
                            "fl105" => overlay_data.overlay_105.clone(),
                            "atzdz" => overlay_data.overlay_atzdz.clone(),
                            _ => None,
                        };
                        x.unwrap_or("* ERROR missing overlay data\n".to_string())
                    } else {
                        "* ERROR overlay data not loaded\n".to_string()
                    }
                } else {
                    "".to_string()
                };

            let blob = Blob::new((openair_data + overlay_data.as_str()).as_str());
            let object_url = ObjectUrl::from(blob);

            let fname = match settings.get().format.as_str() {
                "overlay" => "overlay.txt".to_string(),
                _ => format!("uk{}.txt", &airac_date_string),
            };

            a.set_download(&fname);
            a.set_href(&object_url);
        }

        // Trigger "donwload"
        a.click();
    };

    let tab_names = vec!["Main", "Option", "Extra", "NOTAM", "About"];

    let children = vec![
        airspace_tab(glider_names).into_any(),
        option_tab().into_any(),
        extra_tab(
            rat_panel(rat_names).into_any(),
            loa_panel(exclusive_loas).into_any(),
            wave_panel(wave_names).into_any(),
        )
        .into_any(),
        notam_tab().into_any(),
        about_tab().into_any(),
    ];

    (
        // Page header
        header()
            .class("hero is-small has-background-primary-soft block")
            .child(
                div().class("hero-body").child(
                    div().class("container").child(
                        div()
                            .class("title is-4 has-text-primary-soft-invert")
                            .child("ASSelect - UK Airspace"),
                    ),
                ),
            ),
        // Tabs
        div()
            .class("container block")
            .child(tabs(tab_names, children)),
        // Buttons
        div().class("container block").child(
            div().class("mx-4").child((
                button()
                    .attr("type", "submit")
                    .class("button is-primary has-text-primary-100")
                    .on(ev::click, download)
                    .child("Get Airspace"),
                a()
                    .class("is-pulled-right")
                    .attr("href", format!("https://www.aurora.nats.co.uk/htmlAIP/Publications/{}-AIRAC/html/index-en-GB.html", airac_date))
                    .attr("target", "_blank")
                    .child(format!("AIRAC: {}", airac_date)),
            )),
        ),
        // For data download
        a().hidden(true).node_ref(download_node_ref),
    )
}

// Get list of RA(T)s (maintaining original order)
fn get_rat_names(rats: &Vec<AirspaceFeature>) -> Vec<String> {
    let mut names: Vec<String> = rats
        .iter()
        .map(|f| f.group_name.as_ref().unwrap().to_string())
        .collect();

    names.dedup();
    names
}

// Get alphabetically ordered map of LOAs
fn get_loa_names(loas: &Vec<AirspaceFeature>) -> BTreeMap<String, Vec<String>> {
    // de-duplicated LOA names
    let names: HashSet<&str> = loas
        .into_iter()
        .map(|f| f.group_name.as_ref().unwrap().as_str())
        .collect();

    // map of LOA name to replacement identifiers
    let replace_ids: HashMap<String, HashSet<String>> = names
        .iter()
        .map(|n| {
            (
                n.to_string(),
                loas.iter()
                    .filter(|x| x.group_name.as_ref().unwrap() == *n && x.aref.is_some())
                    .map(|x| x.aref.as_ref().unwrap().to_string())
                    .collect(),
            )
        })
        .collect();

    // map of LOA name to mutually exclusive LOAs
    replace_ids
        .iter()
        .map(|(n1, ids)| {
            (
                n1.to_string(),
                replace_ids
                    .iter()
                    .filter(|(n2, other_ids)| {
                        n1 != *n2 && ids.intersection(&other_ids).count() != 0
                    })
                    .map(|(n, _)| n.to_string())
                    .collect(),
            )
        })
        .collect()
}

fn get_wave_names(airspace: &Vec<AirspaceFeature>) -> Vec<String> {
    airspace
        .iter()
        .filter(|a| vec!["NSGA", "TRAG"].contains(&(a.atype.as_str())))
        .map(|a| a.name.to_string())
        .collect()
}

fn get_glider_names(airspace: &Vec<AirspaceFeature>) -> Vec<String> {
    airspace
        .iter()
        .filter(|a| a.atype == "GLIDER")
        .map(|a| a.name.to_string())
        .collect()
}
