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
use leptos::ev;
use leptos::html::{div, input, label, p};
use leptos::prelude::*;

use crate::settings::{ExtraType, Settings};

pub fn extra_panel(names: Vec<String>, id: ExtraType) -> impl IntoView {
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
                                            getter.with(|s| s.get_extra(id).contains(&nc1))
                                        })
                                        .on(ev::input, move |ev| {
                                            setter.update(|s| {
                                                s.set_extra(id, &nc2, event_target_checked(&ev))
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
