// Copyright 2026, Alan Sparrow
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
use leptos::html::{a, div, li, nav, ul};
use leptos::prelude::*;

pub fn tabs(tab_names: Vec<&str>, children: Vec<AnyView>) -> impl IntoView {
    let (selected, set_selected) = signal(0);

    (
        nav().class("tabs").child(
            ul().child(
                tab_names
                    .into_iter()
                    .enumerate()
                    .map(|(index, tab_name)| {
                        li().class(move || {
                            if selected.get() == index {
                                Some("is-active")
                            } else {
                                None
                            }
                        })
                        .child(
                            a().on(ev::click, move |_| set_selected.set(index))
                                .child(tab_name.to_string()),
                        )
                    })
                    .collect_view(),
            ),
        ),
        div().class("mx-4").child(
            children
                .into_iter()
                .enumerate()
                .map(|(index, child)| div().hidden(move || index != selected.get()).child(child))
                .collect_view(),
        ),
    )
}
