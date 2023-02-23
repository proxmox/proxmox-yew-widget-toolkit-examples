use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{Button, Container, TabPanel, MiniScrollMode};


#[function_component]
pub fn TabPanelDemo() -> Html {

    fn render_tab(text: &str) -> Html {
        Container::new()
            .padding(2)
            .with_child(text)
            .into()
    }

    let mut panel = TabPanel::new()
        //.class("pwt-size-fit-content")
        .class("pwt-fit")
        .scroll_mode(MiniScrollMode::Arrow)
        .title("This is a simple Tab Panel")
        .tool(Button::new("?").class("circle").disabled(true))
        .tool(Button::new("!").class("circle").disabled(false));

    for i in 0..6 {
        panel.add_item(
            format!("tab{i}"),
            format!("Tab {i}"),
            "fa fa-hdd-o",
            render_tab(&format!("This is Tab {i}")),
        );
    }

    panel.navigation_container()
        .into()
}
