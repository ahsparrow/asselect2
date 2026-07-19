use leptos::ev;
use leptos::html::{div, input, label, p};
use leptos::prelude::*;

use crate::settings::Settings;

pub fn rat_panel(names: Vec<String>) -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");
    let getter = use_context::<ReadSignal<Settings>>().expect("to find getter");

    if names.len() > 0 {
        div()
            .class("columns is-multiline")
            .child(
                names
                    .into_iter()
                    .map(|n| {
                        let nc1 = n.clone();
                        let nc2 = n.clone();
                        div().class("column is-one-third").child(
                            div().class("field").child(
                                label().class("checkbox").child((
                                    input()
                                        .r#type("checkbox")
                                        .class("mr-2")
                                        .prop("checked", move || {
                                            getter.with(|s| s.get_rat().contains(&nc1))
                                        })
                                        .on(ev::input, move |ev| {
                                            setter.update(|s| {
                                                s.set_rat(&nc2, event_target_checked(&ev))
                                            })
                                        }),
                                    n,
                                )),
                            ),
                        )
                    })
                    .collect_view(),
            )
            .into_any()
    } else {
        p().child("No temporary restrictions").into_any()
    }
}
