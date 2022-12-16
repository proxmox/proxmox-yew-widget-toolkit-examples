use pwt::prelude::*;
use yew::prelude::*;

use pwt::widget::{ActionIcon, Button, Column, Panel, Row};

#[function_component]
pub fn SevenGuisCounter() -> Html {

    let count = use_state(|| 0);
    let label = html!{<input readonly=true value={count.to_string()}/>};
    let button = Button::new("Count")
        .onclick(move |_| { count.set(*count + 1); })
        .into();

    Panel::new()
        .title("Counter")
        .with_child(
            Row::new()
                .class("pwt-align-items-center")
                .padding(2)
                .gap(2)
                .children([label, button])
        )
        .into()
}
