use crate::components::Login;
use dioxus::prelude::*;

#[component]
pub fn Loginroute() -> Element {
    rsx! {
        Login {}
    }
}
