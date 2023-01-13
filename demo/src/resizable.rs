use pwt::prelude::*;

use pwt::widget::{Column, Resizable, Row};

#[function_component]
pub fn ResizableDemo() -> Html {

    Row::new()
        .class("pwt-fit")
        .with_child(Resizable::new(html!{<div class="pwt-p-2">{"Region1"}</div>}))
        .with_child(
            Column::new()
                .attribute("style", "min-width:30px")
                .class("pwt-flex-fill pwt-overflow-auto")
                .with_child(
                    Resizable::new(html!{<div class="pwt-p-2">{"Region2"}</div>})
                        .vertical(true)
                )
                .with_child(html!{<div class="pwt-p-2 pwt-flex-fill">{"Region3"}</div>})
        )
        .into()
}
