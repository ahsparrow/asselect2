use leptos::html::{a, div};
use leptos::prelude::*;

pub fn notam_tab() -> impl IntoView {
    div().child((
        div().class("subtitle") .child("Navigation Warnings"),
        div().class("block")
            .child("The PDFs below show a summary of the navigation warning NOTAMs relevant to cross country gliding. The PDFs are refreshed during the day at approximately ten minutes to the hour."),
        div().class("block ml-4")
            .child(a()
                .attr("href", "https://navplot.asselect.uk/today_south.pdf")
                .attr("download", "")
                .child("Download Today (England/Wales) PDF")),
        div().class("block ml-4")
            .child(a()
                .attr("href", "https://navplot.asselect.uk/today_north.pdf")
                .attr("download", "")
                .child("Download Today (North England/Scotland) PDF")),
        div().class("block ml-4")
            .child(a()
                .attr("href", "https://navplot.asselect.uk/tomorrow_south.pdf")
                .attr("download", "")
                .child("Download Tomorrow (England/Wales) PDF")),
        div().class("block ml-4")
            .child(a()
                .attr("href", "https://navplot.asselect.uk/tomorrow_north.pdf")
                .attr("download", "")
                .child("Download Tomorrow (North England/Scotland) PDF")),
    ))
}
