use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{Column, Button, Row};

#[function_component]
pub fn ButtonsDemo() -> Html {

    fn render_buttons(disabled: bool) -> Html {
        Row::new()
            .gap(2)
            .with_child(Button::new("Normal").disabled(disabled))
            .with_child(Button::new("Primary").disabled(disabled).class("primary"))
            .with_child(Button::new("Secondary").disabled(disabled).class("secondary"))
            .with_child(Button::new("Neutral").disabled(disabled).class("neutral"))
            .into()
    }

    Column::new()
        .padding(2)
        .gap(2)
        .class("pwt-align-items-start")
        .with_child("Enabled Buttons")
        .with_child(render_buttons(false))
        .with_child("Disabled Buttons")
        .with_child(render_buttons(true))
        .with_child("Button with Icon")
        .with_child(Button::new("Delete").icon_class("fa fa-trash-o"))
        .with_child("Round Button with Icon")
        .with_child(Button::new("Delete").icon_class("fa fa-trash-o").class("rounded"))
        .with_child("Round Icon Button")
        .with_child(Button::new_icon("fa fa-trash-o").class("rounded"))
        .into()
}
