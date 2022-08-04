mod buttons;
use buttons::ButtonsDemo;

mod tab_panel;
use tab_panel::TabPanelDemo;

use yew::prelude::*;
use yew_router::HashRouter;

use pwt::prelude::*;

use pwt::component::{NavigationMenu, MenuItem};
use pwt::widget::{Panel};

#[function_component]
fn MainMenu() -> Html {
    NavigationMenu::new()
        .with_item(
            MenuItem::new(
                "tab_panel",
                "Tab Panel",
                None::<&str>,
                |_| { html!{<TabPanelDemo/>} },
            )
        )
       .with_item(
            MenuItem::new(
                "button_panel",
                "Buttons",
                None::<&str>,
                |_| { html!{<ButtonsDemo/>} },
            )
        )
        .navigation_container()
        .into()
}

#[function_component]
fn DemoApp() -> Html {
    Panel::new()
        .class("pwt-viewport")
        .title("Proxmox Yew Widget Toolkit Demo Application")
        .with_child(html!{<MainMenu/>})
        .into()
}

#[function_component]
fn RoutedApp() -> Html {
    html!{ <HashRouter><DemoApp/></HashRouter> }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<RoutedApp>::new().render();
}
