use pwt::{
    prelude::*,
    widget::{Container, InputPanel},
};
use yew::prelude::*;

use pwt::widget::form::{Checkbox, Combobox, Field};

#[function_component]
pub fn FieldsDemo() -> Html {
    Container::new()
        .padding(2)
        .with_child(
            InputPanel::new()
                .with_field("Textfield", Field::new("textfield"))
                .with_field("Password", Field::new("textfield").input_type("password"))
                .with_field("Range field", Field::new("textfield").input_type("range"))
                .with_field(
                    "Numberfield",
                    Field::new("numberfield").input_type("number"),
                )
                .with_field(
                    "Numberfield with limits",
                    Field::new("numberfield").number(0.0, 150.0, 2.0),
                )
                .with_field("Checkbox", Checkbox::new())
                .with_field(
                    "Combobox",
                    Combobox::new("combobox")
                        .with_item("Item 1")
                        .with_item("Item 2"),
                ),
        )
        .into()
}
