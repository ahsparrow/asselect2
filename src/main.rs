use futures::join;
use gloo::net::http::Request;
use leptos::html::p;
use leptos::prelude::*;

use features::{parse_airspace, parse_loa, parse_rat};
use main_view::{OverlayData, main_view};
use settings::Settings;

mod components;
mod convert;
mod features;
mod main_view;
mod settings;

fn app() -> impl IntoView {
    // Reactive data getters
    let async_airspace = LocalResource::new(|| fetch_data("airspace.geojson"));
    let async_loa = LocalResource::new(|| fetch_data("loa.geojson"));
    let async_rat = LocalResource::new(|| fetch_data("rat.geojson"));
    let async_obstacle = LocalResource::new(|| fetch_data("obstacle.geojson"));

    let async_overlay = LocalResource::new(|| async {
        let overlay_195 = fetch_data("overlay_195.txt");
        let overlay_105 = fetch_data("overlay_105.txt");
        let overlay_atzdz = fetch_data("overlay_atzdz.txt");
        let (o_195, o_105, o_atzdz) = join!(overlay_195, overlay_105, overlay_atzdz);
        OverlayData {
            overlay_195: o_195,
            overlay_105: o_105,
            overlay_atzdz: o_atzdz,
        }
    });

    move || -> AnyView {
        if let (Some(airspace), Some(loa), Some(rat)) =
            (async_airspace.get(), async_loa.get(), async_rat.get())
        {
            if let (Some(airspace_text), Some(loa_text), Some(rat_text)) = (airspace, loa, rat) {
                let (airspace_features, airac_date) = parse_airspace(&airspace_text);
                let loa_features = parse_loa(&loa_text);
                let rat_features = parse_rat(&rat_text);

                // Create reactive view
                let view_fn = move || {
                    main_view(
                        airspace_features,
                        loa_features,
                        rat_features,
                        airac_date,
                        async_overlay,
                        async_obstacle,
                    )
                };
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
