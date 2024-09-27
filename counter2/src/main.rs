use pwt::prelude::*;

use pwt::widget::{Button, Column, Panel, Toolbar};
use yew::{function_component, Html};

#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0i64);

    let toolbar = Toolbar::new()
        .border_bottom(true)
        .with_child(Button::new("Increment").onclick({
            let counter = counter.clone();
            Callback::from(move |_| counter.set(*counter + 1))
        }))
        .with_child(Button::new("Decrement").onclick({
            let counter = counter.clone();
            Callback::from(move |_| counter.set(*counter - 1))
        }))
        .with_flex_spacer()
        .with_child(Button::new("Reset").onclick({
            let counter = counter.clone();
            Callback::from(move |_| counter.set(0))
        }));

    let content = Column::new()
        .padding(2)
        .with_child("The current counter value is: ")
        .with_child(*counter);

    Panel::new()
        .margin(4)
        .title("A simple Counter inside a Panel")
        .with_child(toolbar)
        .with_child(content)
        .into()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
