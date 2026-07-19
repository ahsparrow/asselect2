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
