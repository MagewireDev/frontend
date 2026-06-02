use crate::components::Menu;
use crate::components::Profilecard;
use dioxus::prelude::*;

#[component]
pub fn Discoverroute() -> Element {
    rsx! {
        Menu {  }
        Profilecard {  }
    }
}
