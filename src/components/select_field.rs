use leptos::ev;
use leptos::html::{div, label, option, select};
use leptos::prelude::*;

use crate::Settings;

pub fn select_field(
    setter: WriteSignal<Settings>,
    getter: ReadSignal<Settings>,
    label_str: &str,
    setting: &str,
    options: &Vec<&str>,
    values: &Vec<&str>,
) -> impl IntoView + use<> {
    let setting1 = setting.to_string();
    let setting2 = setting.to_string();

    div()
        .class("field")
        .child(
            label().class("label").child((
                label_str.to_string(),
                div().class("control").child(
                    div().class("select is-fullwidth").child(
                        select()
                            .prop("value", move || getter.with(|s| s.get(&setting1)))
                            .on(ev::change, move |ev| {
                                setter.update(|s| s.update(&setting2, &event_target_value(&ev)))
                            })
                            .child(
                                options
                                    .iter()
                                    .zip(values)
                                    .map(|(o, v)| {
                                        let setting = setting.to_string();
                                        let v = v.to_string();
                                        option().value(v.to_string()).child(o.to_string()).selected(
                                            move || *v == getter.with(|s| s.get(&setting)),
                                        )
                                    })
                                    .collect_view(),
                            ),
                    ),
                ),
            )),
        )
        .into_any()
}
