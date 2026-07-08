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

use crate::settings::Settings;

pub fn extra_tab(rat_panel: AnyView, loa_panel: AnyView, wave_panel: AnyView) -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");

    let (get, set) = signal(0);

    div().child((
        div().class("card block").child((
            header()
                .class("card-header is-clickable")
                .on(ev::click, move |_| set.set(0))
                .child((
                    p().class("card-header-title")
                        .child("Temporary Restrictions"),
                    div().hidden(move || get.get() != 0).child(
                        div().class("card-header-icon").child(
                            input()
                                .r#type("button")
                                .class("button is-info is-soft is-small ml-2")
                                .value("Clear")
                                .on(ev::click, move |_| setter.update(|s| s.clear_rat())),
                        ),
                    ),
                )),
            div()
                .class("card-content")
                .hidden(move || get.get() != 0)
                .child(rat_panel),
        )),
        div().class("card block").child((
            header()
                .class("card-header is-clickable")
                .on(ev::click, move |_| set.set(1))
                .child((
                    p().class("card-header-title").child("Local Agreements"),
                    div().hidden(move || get.get() != 1).child(
                        div().class("card-header-icon").child(
                            input()
                                .r#type("button")
                                .class("button is-info is-soft is-small ml-2")
                                .value("Clear")
                                .on(ev::click, move |_| setter.update(|s| s.clear_loa())),
                        ),
                    ),
                )),
            div()
                .class("card-content")
                .hidden(move || get.get() != 1)
                .child(loa_panel),
        )),
        div().class("card block").child((
            header()
                .class("card-header is-clickable")
                .on(ev::click, move |_| set.set(2))
                .child((
                    p().class("card-header-title").child("Wave Boxes"),
                    div().hidden(move || get.get() != 2).child(
                        div().class("card-header-icon").child(
                            input()
                                .r#type("button")
                                .class("button is-info is-soft is-small ml-2")
                                .value("Clear")
                                .on(ev::click, move |_| setter.update(|s| s.clear_wave())),
                        ),
                    ),
                )),
            div()
                .class("card-content")
                .hidden(move || get.get() != 2)
                .child(wave_panel),
        )),
    ))
}
