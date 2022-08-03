use yew::prelude::*;
use yew_router::HashRouter;

use pwt::prelude::*;

use pwt::component::{NavigationMenu, MenuItem};
use pwt::widget::{Button, Container, Column, Panel, Row, TabPanel};

#[function_component]
fn ButtonsDemo() -> Html {

    fn render_buttons(disabled: bool) -> Html {
        Row::new()
            .gap(2)
            .with_child(Button::new("Normal").disabled(disabled))
            .with_child(Button::new("Primary").disabled(disabled).class("primary"))
            .with_child(Button::new("Secondary").disabled(disabled).class("secondary"))
            .with_child(Button::new("Neutral").disabled(disabled).class("neutral"))
            .into()
    }

    Column::new()
        .padding(2)
        .gap(2)
        .class("pwt-align-items-start")
        .with_child("Enabled Buttons")
        .with_child(render_buttons(false))
        .with_child("Disabled Buttons")
        .with_child(render_buttons(true))
        .with_child("Button with Icon")
        .with_child(Button::new("Delete").icon_class("fa fa-trash-o"))
        .with_child("Round Button with Icon")
        .with_child(Button::new("Delete").icon_class("fa fa-trash-o").class("rounded"))
        .with_child("Round Icon Button")
        .with_child(Button::new_icon("fa fa-trash-o").class("rounded"))
        .into()
}

#[function_component]
fn TabPanelDemo() -> Html {

    fn render_tab(text: &str) -> Html {
        Container::new()
            .padding(2)
            .with_child(text)
            .into()
    }

    TabPanel::new()
        .class("pwt-fit")
        .with_item(
            "tab1",
            "Tab 1",
            None::<&str>,
            render_tab("This is Tab 1"),
        )
       .with_item(
            "tab2",
            "Tab 2",
            None::<&str>,
           render_tab("This is Tab 2"),
        )
        .with_item(
            "tab3",
            "Tab 3",
            None::<&str>,
            render_tab("This is Tab 3"),
        )
        .navigation_container()
        .into()
}

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
