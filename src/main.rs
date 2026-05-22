use std::collections::HashSet;

use codee::string::JsonSerdeCodec;
use geojson::FeatureCollection;
use gloo::net::http::Request;
use leptos::ev;
use leptos::html::{a, button, div, header, p};
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;

use components::{
    about_tab::about_tab, airspace_tab::airspace_tab, extra_panel::extra_panel,
    extra_tab::extra_tab, notam_tab::notam_tab, option_tab::option_tab, tabs::tabs,
};
use settings::{ExtraType, Settings};

mod components;
mod settings;

fn app() -> impl IntoView {
    let async_airspace = LocalResource::new(|| fetch_data("airspace.geojson"));
    let async_loa = LocalResource::new(|| fetch_data("loa.geojson"));
    let async_rat = LocalResource::new(|| fetch_data("rat.geojson"));

    move || -> AnyView {
        if let (Some(airspace), Some(rat), Some(loa)) =
            (async_airspace.get(), async_rat.get(), async_loa.get())
        {
            if let (Some(_airspace_text), Some(rat_text), Some(_loa_text)) = (airspace, rat, loa) {
                if let Ok::<FeatureCollection, _>(rat_fc) = rat_text.parse() {
                    // This needs to use view! macro, otherwise reactive system breaks. Don't know why
                    view! {<MainView rat_fc=rat_fc.clone() />}.into_any()
                } else {
                    p().child("Error parsing airspace data").into_any()
                }
            } else {
                p().child("Error getting airspace data").into_any()
            }
        } else {
            p().child("Getting airspace data, please wait...")
                .into_any()
        }
    }
}

#[component]
fn MainView(rat_fc: FeatureCollection) -> impl IntoView {
    let mut rat_names: Vec<String> = rat_fc
        .features
        .iter()
        .map(|f| {
            f.properties
                .as_ref()
                .unwrap()
                .get("rat_name")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect();

    let mut seen = HashSet::new();
    rat_names.retain(|item| seen.insert(item.clone()));

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
            vec![extra_panel(rat_names, ExtraType::Rat).into_any()],
            vec!["Temporary Restrictions"],
            vec![ExtraType::Rat],
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
