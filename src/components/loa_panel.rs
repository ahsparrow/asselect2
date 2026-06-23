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
