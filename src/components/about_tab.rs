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
            "The airspace data is updated on the AIRAC four week ",
            a().class("text-primary")
                .href("https://nats-uk.ead-it.com/cms-nats/export/sites/default/en/Publications/publication-schedule/10-year-AIRAC.pdf")
                .child("schedule"),
            ", usually about two weeks before the AIRAC effective date. ",
            "The data is generated automatically from the UK AIP (see ",
            a().class("text-primary").href("https://github.com/ahsparrow/aip_airspace").child("GitHub"),
            " for details.)",
         )),
        p().child("Third party applications are free to access the data used by ASSelect from these links -"),
        ul().child((
            li().child((
                "Airspace - ",
                a().class("text-primary").href("/airspace.geojson").child("https://asselect.uk/airspace.geojson"),
            )),
            li().child((
                "Temporary restrictions - ",
                a().class("text-primary").href("/rat.geojson").child("https://asselect.uk/rat.geojson"),
            )),
            li().child((
                "Letters of agreement - ",
                a().class("text-primary").href("/loa.geojson").child("https://asselect.uk/loa.geojson"),
            )),
        )),
         div().class("subtitle").child("Contact"),
         address().child((
            "Comments, corrections and complaints to: ",
            a().href("mailto:web@asselect.uk").child("Alan Sparrow"),
        )),
    ))
}
