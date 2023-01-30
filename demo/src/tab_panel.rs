use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{Button, Container, TabPanel};


#[function_component]
pub fn TabPanelDemo() -> Html {

    fn render_tab(text: &str) -> Html {
        Container::new()
            .padding(2)
            .with_child(text)
            .into()
    }

    TabPanel::new()
        .class("pwt-fit")
        .title("This is a simple Tab Panel")
        .tool(Button::new("?").class("circle").disabled(true))
        .tool(Button::new("!").class("circle").disabled(false))
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
