use crate::api::models::DiscoveryResponse;
use crate::api::models::LikeRequest;
use crate::api::models::UpdateProfileRequest;
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::*;
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::tabs::*;
use dioxus::prelude::*;
use dioxus_router::navigator;

async fn discover(liker_id: i32) -> Result<DiscoveryResponse, reqwest::Error> {
    let profile = reqwest::Client::new()
        .get(format!("http://localhost:8000/discoveries/{liker_id}"))
        .send()
        .await?
        .error_for_status()?
        .json::<DiscoveryResponse>()
        .await?;

    Ok(profile)
}

async fn like(liker_id: i32, liked_id: i32, status: bool) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .post("http://localhost:8000/likes")
        .json(&LikeRequest {
            liker_id,
            liked_id,
            status,
        })
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

#[component]
pub fn Profilecard(liker_id: i32) -> Element {
    let mut profile = use_signal(|| None::<DiscoveryResponse>);
    let mut error = use_signal(|| None::<String>);

    use_effect(move || {
        spawn(async move {
            match discover(liker_id).await {
                Ok(next_profile) => profile.set(Some(next_profile)),
                Err(err) => error.set(Some(format!("Initial Discovery failed: {err}"))),
            }
        });
    });
    let like_current = move |_| async move {
        if let Some(p) = profile() {
            match like(liker_id, p.discovery_id, true).await {
                Ok(_) => match discover(liker_id).await {
                    Ok(next_profile) => profile.set(Some(next_profile)),
                    Err(err) => error.set(Some(format!("Discovery failed: {err}"))),
                },
                Err(err) => error.set(Some(format!("Like failed: {err}"))),
            }
        }
    };
    let skip_current = move |_| async move {
        if let Some(p) = profile() {
            match like(liker_id, p.discovery_id, false).await {
                Ok(_) => match discover(liker_id).await {
                    Ok(next_profile) => profile.set(Some(next_profile)),
                    Err(err) => error.set(Some(format!("Discovery failed: {err}"))),
                },
                Err(err) => error.set(Some(format!("Skip failed: {err}"))),
            }
        }
    };
    rsx! {
        Card { style: "width: 100%; max-width: 24rem;",
            CardHeader {
                CardTitle { "Profile" }
            }
            CardContent {
                match profile() {
                    Some(p) => rsx! {
                        div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                            div { style: "display: grid; gap: 0.5rem;",
                                Profiletabs {
                                    display_name: p.display_name.clone(),
                                    bio: p.bio.clone(),
                                    zodiac: p.zodiac.clone(),
                                }
                            }
                        }
                    },
                    None => rsx! {
                        p { "No profile loaded yet." }
                    }
                }
            }
            CardFooter { style: "flex-direction: column; gap: 0.5rem;",
                Button {
                    variant: ButtonVariant::Primary,
                    style: "width: 100%;",
                    onclick: like_current,
                    "Like"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    style: "width: 100%;",
                    onclick: skip_current,
                    "Skip"
                }
            }
            if let Some(message) = error() {
                p {style: "color: red; font-size: o.875rem;", "{message}" }
            }
        }
    }
}

#[component]
fn Profiletabs(display_name: String, bio: String, zodiac: String) -> Element {
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
                    "{display_name}"
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
                    "{zodiac}"
                }
            }
            TabContent { index: 2usize, value: "tab3".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "{bio}"
                }
            }
        }
    }
}
async fn update_profile(
    display_name: String,
    bio: String,
    zodiac: String,
) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .put("http://localhost:8000/profiles")
        .json(&UpdateProfileRequest {
            display_name,
            bio,
            zodiac,
        })
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}
#[component]
pub fn Profilecardform() -> Element {
    rsx! {
        Card { style: "width: 100%; max-width: 24rem;",
            CardHeader {
                CardTitle { "Your Profile" }
            }
            CardContent {
                div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                     div { style: "display: grid; gap: 0.5rem;",
                        Profiletabsform { }
                    }
                }
            }
        }
    }
}

#[component]
fn Profiletabsform() -> Element {
    let mut bio = use_signal(String::new);
    let mut display_name = use_signal(String::new);
    let mut zodiac = use_signal(String::new);
    let mut success = use_signal(|| None::<String>);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let nav = navigator();

    rsx! {
        form {
            id: "update-form",

            onsubmit: move |event| async move {
                event.prevent_default();

                loading.set(true);
                success.set(None);
                error.set(None);

                let result = update_profile(display_name(), bio(), zodiac()).await;

                loading.set(false);

                match result {
                    Ok(_) => {
                        success.set(Some(format!("Profile updated")));
                    }
                    Err(err) => {
                        error.set(Some(format!("Login failed: {err}")));
                    }
                }
            },

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
                            id: "display_name",
                            name: "display_name",
                            r#type: "display_name",
                            placeholder: "Niels",
                            value: "{display_name}",
                            oninput: move |event: Event<FormData>| {
                                display_name.set(event.value());
                            }
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            r#type: "submit",
                            form: "update-form",
                            style: "width: 100%;",
                            "Update"
                        }
                        if let Some(message) = error() {
                            p {style: "color: red; font-size: o.875rem;", "{message}" }
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
