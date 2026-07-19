use std::collections::BTreeMap;

use leptos::ev;
use leptos::html::{div, input, label};
use leptos::prelude::*;

use crate::settings::Settings;

pub fn loa_panel(loas: BTreeMap<String, Vec<String>>) -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");
    let getter = use_context::<ReadSignal<Settings>>().expect("to find getter");

    div()
        .class("columns is-multiline")
        .child(
            loas.iter()
                .map(|(n, e)| {
                    let nc1 = n.clone();
                    let nc2 = n.clone();
                    div().class("column is-one-third").child(
                        div().class("field").child(
                            label().class("checkbox").child((
                                input()
                                    .r#type("checkbox")
                                    .class("mr-2")
                                    .prop("checked", move || {
                                        getter.with(|s| s.get_loa().contains(&nc1))
                                    })
                                    .on(ev::input, {
                                        let exclusive = e.clone();
                                        move |ev| {
                                            if event_target_checked(&ev) {
                                                setter.update(|s| s.set_loa(&nc2));

                                                // Remove any mutually exclusive LOAs
                                                for loa in &exclusive {
                                                    setter.update(|s| s.unset_loa(loa))
                                                }
                                            } else {
                                                setter.update(|s| s.unset_loa(&nc2))
                                            }
                                        }
                                    }),
                                n.to_string(),
                            )),
                        ),
                    )
                })
                .collect_view(),
        )
        .into_any()
}
