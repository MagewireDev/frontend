use crate::components::button::{Button, ButtonVariant};
use crate::components::card::*;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    rsx! {
        Card { style: "width: 100%; max-width: 24rem;",
            CardHeader {
                CardTitle { "Login to your account" }
            }
            CardContent {
                form { id: "login-form",
                    div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                        div { style: "display: grid; gap: 0.5rem;",
                            Label { html_for: "email", "Email" }
                        }
                        div { style: "display: grid; gap: 0.5rem;",
                            div { style: "display: flex; align-items: center;",
                                Label { html_for: "password", "Password" }
                                a {
                                    href: "#",
                                    style: "margin-left: auto; font-size: 0.875rem; color: var(--secondary-color-5); text-decoration: underline; text-underline-offset: 4px;",
                                    "Forgot your password?"
                                }
                            }
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
                    "Login"
                }
                Button { variant: ButtonVariant::Outline, style: "width: 100%;", "Sign up" }
            }
        }
    }
}
