use pwt::prelude::*;
use pwt::widget::{Button, Panel, Row};

#[function_component]
pub fn Counter() -> Html {
    let count = use_state(|| 0);
    let label = html! {<input class="pwt-input" readonly=true value={count.to_string()}/>};
    let button = Button::new("Count")
        .onclick(move |_| {
            count.set(*count + 1);
        })
        .into();

    Row::new()
        .with_child(
            Panel::new().border(true).title("Counter").with_child(
                Row::new()
                    .class("pwt-align-items-center")
                    .padding(2)
                    .gap(2)
                    .children([label, button]),
            ),
        )
        .into()
}
