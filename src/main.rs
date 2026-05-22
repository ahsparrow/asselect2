use std::collections::HashSet;

use codee::string::JsonSerdeCodec;
use gloo::net::http::Request;
use leptos::ev;
use leptos::html::{a, button, div, header, p};
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;

use components::{
    about_tab::about_tab, airspace_tab::airspace_tab, extra_panel::extra_panel,
    extra_tab::extra_tab, notam_tab::notam_tab, option_tab::option_tab, tabs::tabs,
};
use features::{AirspaceFeature, LoaFeature, RatFeature, parse_airspace, parse_loa, parse_rat};
use settings::{ExtraType, Settings};

mod components;
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
                let airspace_features = parse_airspace(&airspace_text);
                let loa_features = parse_loa(&loa_text);
                let rat_features = parse_rat(&rat_text);

                // Create reactive view
                let view_fn = move || main_view(airspace_features, loa_features, rat_features);
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

fn get_rat_names(rats: &Vec<RatFeature>) -> Vec<String> {
    let mut names: Vec<String> = rats.iter().map(|f| f.group_name.to_string()).collect();

    // Remove duplicates
    let mut seen = HashSet::new();
    names.retain(|item| seen.insert(item.clone()));
    names
}

fn get_loa_names(loas: &Vec<LoaFeature>) -> Vec<String> {
    let mut names: Vec<String> = loas.iter().map(|f| f.group_name.to_string()).collect();

    // Remove duplicates
    let mut seen = HashSet::new();
    names.retain(|item| seen.insert(item.clone()));
    names
}

fn get_wave_names(airspace: &Vec<AirspaceFeature>) -> Vec<String> {
    airspace
        .iter()
        .filter(|a| a.stype == "NSGA")
        .map(|a| a.name.to_string())
        .collect()
}

fn main_view(
    airspace_features: Vec<AirspaceFeature>,
    loa_features: Vec<LoaFeature>,
    rat_features: Vec<RatFeature>,
) -> impl IntoView {
    let loa_names = get_loa_names(&loa_features);
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

    // Download button callback
    let download = move |_| {
        // Store settings
        set_local_settings.set(settings.get_untracked());
    };

    let tab_names = vec!["Main", "Option", "Extra", "NOTAM", "About"];

    let children = vec![
        airspace_tab().into_any(),
        option_tab().into_any(),
        extra_tab(
            vec![
                extra_panel(rat_names, ExtraType::Rat).into_any(),
                extra_panel(loa_names, ExtraType::Loa).into_any(),
                extra_panel(wave_names, ExtraType::Wave).into_any(),
            ],
            vec![
                "Temporary Restrictions",
                "Letters of Agreement",
                "Wave Boxes",
            ],
            vec![ExtraType::Rat, ExtraType::Loa, ExtraType::Wave],
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
                    .child(format!("AIRAC: {}", "TODO")),
            )),
        ),
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
