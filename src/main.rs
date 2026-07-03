use std::collections::{BTreeMap, HashMap, HashSet};

use codee::string::JsonSerdeCodec;
use gloo::file::{Blob, ObjectUrl};
use gloo::net::http::Request;
use leptos::ev;
use leptos::html::{A, a, button, div, header, p};
use leptos::prelude::*;
use leptos::web_sys;
use leptos_use::storage::use_local_storage;
use uuid::Uuid;

use components::{
    about_tab::about_tab, airspace_tab::airspace_tab, extra_tab::extra_tab, loa_panel::loa_panel,
    notam_tab::notam_tab, option_tab::option_tab, rat_panel::rat_panel, tabs::tabs,
    wave_panel::wave_panel,
};
use convert::{make_air_filter, oa_type, openair, serialize_airspace};
use features::{AirspaceFeature, parse_airspace, parse_loa, parse_rat};
use settings::Settings;

mod components;
mod convert;
mod features;
mod settings;

fn app() -> impl IntoView {
    // Reactive data getters
    let async_airspace = LocalResource::new(|| fetch_data("airspace.geojson"));
    let async_loa = LocalResource::new(|| fetch_data("loa.geojson"));
    let async_rat = LocalResource::new(|| fetch_data("rat.geojson"));

    move || -> AnyView {
        if let (Some(airspace), Some(loa), Some(rat)) =
            (async_airspace.get(), async_loa.get(), async_rat.get())
        {
            if let (Some(airspace_text), Some(loa_text), Some(rat_text)) = (airspace, loa, rat) {
                let (airspace_features, airac_date) = parse_airspace(&airspace_text);
                let loa_features = parse_loa(&loa_text);
                let rat_features = parse_rat(&rat_text);

                // Create reactive view
                let view_fn =
                    move || main_view(airspace_features, loa_features, rat_features, airac_date);
                view_fn().into_any()
            } else {
                p().child("Error getting airspace data").into_any()
            }
        } else {
            p().child("Getting airspace data, please wait...")
                .into_any()
        }
    }
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

fn main_view(
    airspace_features: Vec<AirspaceFeature>,
    loa_features: Vec<AirspaceFeature>,
    rat_features: Vec<AirspaceFeature>,
    airac_date: String,
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
        airspace.extend(rat);

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
            let od = openair(
                &airspace,
                &untracked_settings,
                &airac_date_string,
                &user_agent,
                oatypes,
            );

            let blob = Blob::new(od.expect("format error").as_str());
            let object_url = ObjectUrl::from(blob);

            let fname = if settings.get().format == "overlay" {
                "overlay.txt".to_string()
            } else {
                format!("uk{}.txt", &airac_date_string)
            };

            a.set_download(&fname);
            a.set_href(&object_url);
        }
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
                a().id("airac-button")
                    .class("button is-text is-pulled-right")
                    .attr("href", format!("https://www.aurora.nats.co.uk/htmlAIP/Publications/{}-AIRAC/html/index-en-GB.html", airac_date))
                    .child(format!("AIRAC: {}", airac_date)),
            )),
        ),
        // For data download
        a().hidden(true).node_ref(download_node_ref),
    )
}

// Get data from server
async fn fetch_data(url: &str) -> Option<String> {
    let result = Request::get(url).send().await;
    match result {
        Ok(response) => response.text().await.ok(),
        _ => None,
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app)
}
