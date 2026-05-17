use codee::string::JsonSerdeCodec;
use leptos::ev;
use leptos::html::{a, button, div, header};
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;

use components::{
    about_tab::about_tab, airspace_tab::airspace_tab,
    extra_tab::extra_tab, notam_tab::notam_tab, option_tab::option_tab, tabs::tabs,
};
use settings::Settings;

mod components;
mod settings;

fn app() -> impl IntoView {
    main_view()
}

fn main_view() -> impl IntoView {
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
        extra_tab().into_any(),
        notam_tab().into_any(),
        about_tab().into_any(),
    ];

    (
        // Page header
        header().class("hero is-small has-background-primary-soft block")
            .child(div().class("hero-body")
                .child(div().class("container")

                    .child(div() .class("title is-4 has-text-primary-soft-invert")
                        .child("ASSelect - UK Airspace"),
                    ),
                ),
            ),

        // Tabs
        div().class("container block")
            .child(tabs(tab_names, children)),

        // Buttons
        div().class("container block").child(
            div().class("mx-4")
                .child((
                    button().attr("type", "submit").class("button is-primary has-text-primary-100")
                    .on(ev::click, download)
                    .child("Get Airspace"),

                    a().id("airac-button").class("button is-text is-pulled-right")
                    .child(format!("AIRAC: {}", "TODO")),
                )),
        )
    )
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app)
}


