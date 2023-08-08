use pwt::css::{Flex, Overflow};
use yew::prelude::*;

use pwt::prelude::*;

use pwt::state::NavigationContainer;
use pwt::widget::{Button, Container, MiniScrollMode, TabBarItem, TabPanel};

#[function_component]
pub fn TabPanelDemo() -> Html {
    fn render_tab(text: &str) -> Html {
        Container::new().padding(2).with_child(text).into()
    }

    let mut panel = TabPanel::new()
        .class(Flex::Fill)
        .class(Overflow::Auto)
        .router(true)
        .scroll_mode(MiniScrollMode::Native)
        .title("This is a simple Tab Panel")
        .tool(Button::new("?").class("circle").disabled(true))
        .tool(Button::new("!").class("circle").disabled(false));

    for i in 0..6 {
        panel.add_item(
            TabBarItem::new()
                .key(format!("tab{i}"))
                .label(format!("Tab {i}"))
                .icon_class("fa fa-hdd-o")
                .disabled(i == 4),
            render_tab(&format!("This is Tab {i}")),
        );
    }

    NavigationContainer::new().with_child(panel).into()
}
