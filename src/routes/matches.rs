use crate::components::Matchcard;
use crate::components::Menu;
use dioxus::prelude::*;

#[component]
pub fn Matchesroute() -> Element {
    rsx! {
        Menu {  }
        Matchcard {  }
    }
}
