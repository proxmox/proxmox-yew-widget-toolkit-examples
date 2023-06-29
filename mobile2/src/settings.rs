use pwt::touch::prelude::*;

use pwt::touch::Scaffold;
use pwt::widget::{Column, Card};

use yew_router::prelude::*;

use crate::navigation_bar;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum SubSettings {
    #[at("/settings/network")]
    Network,

    #[at("/settings/account")]
    Account,

    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

impl SubSettings {
    pub fn switch(route: SubSettings) -> Html {
        match route {
            SubSettings::Account => html! {<AccountSettings/>},
            SubSettings::Network => html! {<NetworkSettings/>},
            SubSettings::NotFound => html! {<super::NotFound/>},
        }
    }
}

#[function_component]
pub fn Settings() -> Html {
    fn list_tile(titel: &str, body: &str) -> Card {
        Card::new()
            .class("pwt-interactive")
            .class("pwt-shape-none")
            .class("pwt-card-flat")
            .class("pwt-scheme-neutral")
            .class("pwt-border-bottom")
            .with_child(html! {
                <>
                    <h6>{titel}</h6>
                    <p>{body}</p>
                </>
            })
    }

    let navigator = use_navigator().unwrap();

    let body = Column::new()
        .with_child(
            Card::new()
                .class("pwt-shape-none")
                .with_child(html! {<h6>{"Settings"}</h6>}),
        )
        .with_child(
            list_tile(
                "Account",
                "Opens a new page to configure the account settings.",
            )
            .onclick({
                let navigator = navigator.clone();
                move |_| navigator.push(&SubSettings::Account)
            }),
        )
        .with_child(
            list_tile("Network", "Opens a new page to configure the network.").onclick({
                let navigator = navigator.clone();
                move |_| navigator.push(&SubSettings::Network)
            }),
        );

    Scaffold::with_title("Settings")
        .body(body)
        .navigation_bar(navigation_bar())
        .into()
}

#[function_component]
fn AccountSettings() -> Html {
    Scaffold::with_title("Account settings")
        .body(Card::new().margin(2).with_child("Account Settings"))
        .into()
}

#[function_component]
fn NetworkSettings() -> Html {
    Scaffold::with_title("Network settings")
        .body(Card::new().margin(2).with_child("Network Settings"))
        .into()
}
