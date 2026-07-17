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
use leptos::html::{a, address, div, li, p, ul};
use leptos::prelude::*;

pub fn about_tab() -> impl IntoView {
    div().class("content").child((
        p().child(
            "ASSelect provides airspace data for UK glider pilots."
        ),
        div().class("subtitle").child("Output Format"),
        ul().child((
            li().child("OpenAir - Normal format for your flight instruments"),
            li().child("RA(T) Only - No airspace, just the temporary restricted areas"),
            li().child("Overlay Only - No airspace, just the \"numbers\""),
            li().child("Competition - For competition organisers"),
        )),
        div().class("subtitle").child("Data"),
        p().child((
            "Airspace data is updated on a fixed ",
            a().class("text-primary")
                .href("https://nats-uk.ead-it.com/cms-nats/export/sites/default/en/Publications/publication-schedule/10-year-AIRAC.pdf")
                .child("four week schedule"),
            ". This site is usually updated about two weeks before the AIRAC effective date.",
        )),
        p().child((
            "The airspace data is generated directly from the UK AIP, see ",
            a().class("text-primary").href("https://github.com/ahsparrow/aip_airspace").child("GitHub"),
            " for details.",
         )),
         div().class("subtitle").child("Contact"),
         address().child((
            "Comments, corrections and complaints to: ",
            a().href("mailto:web@asselect.uk").child("Alan Sparrow"),
        )),
    ))
}
