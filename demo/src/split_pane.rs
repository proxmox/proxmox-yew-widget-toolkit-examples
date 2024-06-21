use pwt::prelude::*;
use pwt::{css::Flex, widget::Container};

use super::LoremIpsum;
use pwt::widget::{Pane, SplitPane};

#[function_component]
pub fn SplitPaneDemoNested() -> Html {
    let nested = SplitPane::new()
        .vertical(true)
        .with_child(Container::new().padding(2).with_child("Nested Region1"))
        .with_child(Container::new().padding(2).with_child("Nested Region2"))
        .with_child(Container::new().padding(2).with_child("Nested Region3"));

    SplitPane::new()
        .class(Flex::Fill)
        .with_child(Container::new().padding(2).with_child("This is Region1"))
        .with_child(nested)
        .with_child(Container::new().padding(2).with_child("This is Region3"))
        .into()
}

#[function_component]
pub fn SplitPaneDemoHorizontal() -> Html {
    SplitPane::new()
        .class(Flex::Fill)
        .with_child(Pane::new(html! {<LoremIpsum/>}))
        .with_child(Pane::new(html! {<LoremIpsum/>}))
        .with_child(Pane::new(html! {<LoremIpsum/>}))
        .into()
}

#[function_component]
pub fn SplitPaneDemoVertical() -> Html {
    SplitPane::new()
        .class(Flex::Fill)
        .vertical(true)
        .with_child(Pane::new(html! {<LoremIpsum/>}))
        .with_child(Pane::new(html! {<LoremIpsum/>}))
        .with_child(Pane::new(html! {<LoremIpsum/>}))
        .into()
}

#[function_component]
pub fn SplitPaneDemoSizing() -> Html {
    SplitPane::new()
        .class(Flex::Fill)
        .with_child(
            Pane::new(Container::new().padding(2).with_child("Region1: 100 pixel")).size(100),
        )
        .with_child(
            Pane::new(
                Container::new()
                    .padding(2)
                    .with_child("Region2: 20 percent"),
            )
            .fraction(0.2),
        )
        .with_child(
            Pane::new(
                Container::new()
                    .padding(2)
                    .with_child("Region3: None (auto)"),
            )
            .size(None),
        )
        .with_child(
            Pane::new(
                Container::new()
                    .padding(2)
                    .with_child("Region4: flex 1 (default)"),
            )
            .flex(1),
        )
        .with_child(Pane::new(Container::new().padding(2).with_child("Region5: flex 2")).flex(2))
        .into()
}

#[function_component]
pub fn SplitPaneDemoMinMax() -> Html {
    SplitPane::new()
        .class(Flex::Fill)
        .with_child(
            Pane::new(
                Container::new()
                    .padding(2)
                    .with_child("Region1: min 100 pixel, max 80%"),
            )
            .max_fraction(0.8)
            .min_size(100),
        )
        .with_child(Container::new().padding(2).with_child("Region2"))
        .into()
}
