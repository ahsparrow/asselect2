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
