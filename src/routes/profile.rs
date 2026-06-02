use crate::components::Menu;
use crate::components::Profilecardform;
use dioxus::prelude::*;

#[component]
pub fn Profileroute() -> Element {
    rsx! {
        Menu {  }
        Profilecardform {  }
    }
}
