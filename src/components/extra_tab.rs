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
use leptos::html::{div, header, input, p};
use leptos::prelude::*;

use crate::settings::{ExtraType, Settings};

pub fn extra_tab(children: Vec<AnyView>, names: Vec<&str>, ids: Vec<ExtraType>) -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");

    let (get, set) = signal(0);

    names
        .iter()
        .zip(children)
        .zip(ids)
        .enumerate()
        .map(|(n, ((name, cld), id))| {
            div().class("card block").child((
                header()
                    .class("card-header is-clickable")
                    .on(ev::click, move |_| set.set(n))
                    .child((
                        p().class("card-header-title").child(name.to_string()),
                        div().hidden(move || get.get() != n).child(
                            div().class("card-header-icon").child(
                                input()
                                    .r#type("button")
                                    .class("button is-info is-soft is-small ml-2")
                                    .value("Clear")
                                    .on(ev::click, move |_| setter.update(|s| s.clear_extra(id))),
                            ),
                        ),
                    )),
                div()
                    .class("card-content")
                    .hidden(move || get.get() != n)
                    .child(cld),
            ))
        })
        .collect_view()
}
