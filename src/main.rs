#![allow(non_snake_case)]

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

use crate::types::GetGistResponse;

pub mod types;

static GIST_API_URL: &str = "https://api.github.com/gists/";
static GIST_ID: &str = "09c714ff93cd8afa00d00094a56bdd8e";

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let get_gist = use_future(cx, (), |_| async move {
        reqwest::get(format!("{}{}", GIST_API_URL, GIST_ID))
            .await
            .unwrap()
            .json::<GetGistResponse>()
            .await
    });

    cx.render(match get_gist.value() {
        Some(Ok(response)) => rsx! {
            div {
                p {format!("{:#?}", response)}
            }
        },
        Some(Err(_)) => rsx! { div { "Fetch failed" } },
        None => rsx! { div { "Fetching..." } },
    })
}
