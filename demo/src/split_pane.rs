use pwt::prelude::*;

use pwt::widget::{Pane, SplitPane};

#[function_component]
pub fn SplitPaneDemo() -> Html {

    SplitPane::new()
        .class("pwt-fit")
        .vertical(true)
        .with_child(Pane::new("Region1").min_size(20))
        .with_child(Pane::new("Region2").flex(2))
        .with_child(Pane::new("Region3").flex(2).min_fraction(0.2))
        .into()
}

#[function_component]
pub fn SplitPaneDemo3() -> Html {

    SplitPane::new()
        .class("pwt-fit")
        .with_child(Pane::new("Region1").fraction(0.1))
        .with_child(Pane::new("Region2").fraction(0.4))
        .with_child(Pane::new("Region3").fraction(0.5))
        .into()
}

#[function_component]
pub fn SplitPaneDemo1() -> Html {

    SplitPane::new()
        .with_child(html!{<div class="pwt-p-2">{"This is Region1"}</div>})
        .with_child(html!{<div class="pwt-p-2">{"This is Region2 and is longer"}</div>})
        .with_child(
            Pane::new(html!{<div class="pwt-p-2 pwt-overflow-hidden">{"Region3"}</div>})
                .size(900)
                .min_fraction(0.2)
                .max_size(45)
       )
        .with_child(html!{<div class="pwt-p-2">{"Region4"}</div>})
        .into()
}
