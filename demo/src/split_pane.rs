use pwt::prelude::*;

use pwt::widget::{Pane, SplitPane};
use super::LoremIpsum;

#[function_component]
pub fn SplitPaneDemoNested() -> Html {

    let nested = SplitPane::new()
        .class("pwt-fit")
        .vertical(true)
        .with_child(html!{<div class="pwt-p-2">{"Nested Region1"}</div>})
        .with_child(html!{<div class="pwt-p-2">{"Nested Region2"}</div>})
        .with_child(html!{<div class="pwt-p-2">{"Nested Region3"}</div>});

    SplitPane::new()
        .class("pwt-fit")
        .with_child(html!{<div class="pwt-p-2">{"This is Region1"}</div>})
        .with_child(nested)
        .with_child(html!{<div class="pwt-p-2">{"This is Region3"}</div>})
        .into()
}

#[function_component]
pub fn SplitPaneDemoHorizontal() -> Html {
    SplitPane::new()
        .class("pwt-fit")
        .with_child(Pane::new(html!{<LoremIpsum/>}))
        .with_child(Pane::new(html!{<LoremIpsum/>}))
        .with_child(Pane::new(html!{<LoremIpsum/>}))
        .into()
}

#[function_component]
pub fn SplitPaneDemoVertical() -> Html {
    SplitPane::new()
        .class("pwt-fit")
        .vertical(true)
        .with_child(Pane::new(html!{<LoremIpsum/>}))
        .with_child(Pane::new(html!{<LoremIpsum/>}))
        .with_child(Pane::new(html!{<LoremIpsum/>}))
        .into()
}

#[function_component]
pub fn SplitPaneDemoSizing() -> Html {
    SplitPane::new()
        .class("pwt-fit")
        .with_child(
            Pane::new(html!{<div class="pwt-p-2">{"Region1: 100 pixel"}</div>})
                .size(100)
        )
        .with_child(
            Pane::new(html!{<div class="pwt-p-2">{"Region2: 20 percent"}</div>})
                .fraction(0.2)
        )
        .with_child(
            Pane::new(html!{<div class="pwt-p-2">{"Region3: None (auto)"}</div>})
                .size(None)
        )
        .with_child(
            Pane::new(html!{<div class="pwt-p-2">{"Region4: flex 1 (default)"}</div>})
                .flex(1)
        )
        .with_child(
            Pane::new(html!{<div class="pwt-p-2">{"Region5: flex 2"}</div>})
                .flex(2)
        )
        .into()
}

#[function_component]
pub fn SplitPaneDemoMinMax() -> Html {
    SplitPane::new()
        .class("pwt-fit")
        .with_child(
            Pane::new(html!{<div class="pwt-p-2">{"Region1: min 100 pixel, max 80%"}</div>})
                .max_fraction(0.8)
                .min_size(100)
        )
        .with_child(html!{<div class="pwt-p-2">{"Region2"}</div>})
        .into()
}
