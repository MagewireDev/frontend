use crate::components::button::{Button, ButtonVariant};
use crate::components::card::*;
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::tabs::*;
use dioxus::prelude::*;

#[component]
pub fn Profilecard() -> Element {
    rsx! {
        Card { style: "width: 100%; max-width: 24rem;",
            CardHeader {
                CardTitle { "Profile" }
            }
            CardContent {
                form { id: "profile-form",
                    div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                        div { style: "display: grid; gap: 0.5rem;",
                            Profiletabs { }
                        }
                    }
                }
            }
            CardFooter { style: "flex-direction: column; gap: 0.5rem;",
                Button {
                    variant: ButtonVariant::Primary,
                    r#type: "submit",
                    form: "login-form",
                    style: "width: 100%;",
                    "Like"
                }
                Button { variant: ButtonVariant::Outline, style: "width: 100%;", "Skip" }
            }
        }
    }
}

#[component]
pub fn Profilecardform() -> Element {
    rsx! {
        Card { style: "width: 100%; max-width: 24rem;",
            CardHeader {
                CardTitle { "Your Profile" }
            }
            CardContent {
                form { id: "profile-form",
                    div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                        div { style: "display: grid; gap: 0.5rem;",
                            Profiletabsform { }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Matchcard() -> Element {
    rsx! {
        Card { style: "width: 100%; max-width: 24rem;",
            CardHeader {
                CardTitle { "Match" }
            }
            CardContent {
                div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                    div { style: "display: grid; gap: 0.5rem;",
                        Matchtabs { }
                    }
                }
            }
        }
    }
}

#[component]
fn Profiletabs() -> Element {
    rsx! {
        Tabs {
            default_value: "tab1".to_string(),
            horizontal: true,
            max_width: "16rem",
            TabList {
                TabTrigger { value: "tab1".to_string(), index: 0usize, "Name" }
                TabTrigger { value: "tab2".to_string(), index: 1usize, "Zodiac" }
                TabTrigger { value: "tab3".to_string(), index: 2usize, "Bio" }
            }
            TabContent { index: 0usize, value: "tab1".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Amanda Mortensen"
                }
            }
            TabContent {
                index: 1usize,
                value: "tab2".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Virgo"
                }
            }
            TabContent { index: 2usize, value: "tab3".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Sød og glad"
                }
            }
        }
    }
}

#[component]
fn Profiletabsform() -> Element {
    rsx! {
        Tabs {
            default_value: "tab1".to_string(),
            horizontal: true,
            max_width: "16rem",
            TabList {
                TabTrigger { value: "tab1".to_string(), index: 0usize, "Name" }
                TabTrigger { value: "tab2".to_string(), index: 1usize, "Zodiac" }
                TabTrigger { value: "tab3".to_string(), index: 2usize, "Bio" }
            }
            TabContent { index: 0usize, value: "tab1".to_string(),
                div { style: "display: grid; gap: 0.5rem;",
                    Label { html_for: "new_name", "New name:" }
                    Input {
                       id: "name",
                       r#type: "name",
                       placeholder: "Niels",
                    }
                }
            }
            TabContent {
                index: 1usize,
                value: "tab2".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Virgo"
                }
            }
            TabContent { index: 2usize, value: "tab3".to_string(),
                 div { style: "display: grid; gap: 0.5rem;",
                    Label { html_for: "new_bio", "New bio:" }
                    Input {
                       id: "bio",
                       r#type: "bio",
                    }
                }
           }
        }
    }
}

#[component]
fn Matchtabs() -> Element {
    rsx! {
        Tabs {
            default_value: "tab1".to_string(),
            horizontal: true,
            max_width: "16rem",
            TabList {
                TabTrigger { value: "tab1".to_string(), index: 0usize, "Name" }
                TabTrigger { value: "tab2".to_string(), index: 1usize, "Zodiac" }
                TabTrigger { value: "tab3".to_string(), index: 2usize, "Bio" }
            }
            TabContent { index: 0usize, value: "tab1".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Amanda Mortensen"
                }
            }
            TabContent {
                index: 1usize,
                value: "tab2".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Virgo"
                }
            }
            TabContent { index: 2usize, value: "tab3".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Sød og glad"
                }
            }
        }
    }
}
