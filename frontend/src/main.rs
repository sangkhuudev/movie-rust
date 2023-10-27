#![allow(non_snake_case)]
use dioxus::prelude::*;

mod components;
mod models;

use components::{Footer, Header, FilmModal};

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello Rust"
        }
        main {
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {}
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
            }
            Footer {}
            FilmModal {
                on_create_or_update: move |_| {},
                on_cancel: move |_| {},
            }
        }
    })
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("frontend"));
    dioxus_web::launch(App);
}
