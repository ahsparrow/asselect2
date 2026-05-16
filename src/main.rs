use leptos::html::{p};
use leptos::prelude::*;

fn app() -> impl IntoView {
    p().child("Hello Alan")
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app)
}
