use pwt::{
    prelude::*,
    widget::{Container, InputPanel},
};
use yew::prelude::*;

use pwt::widget::form::{Checkbox, Combobox, Field, Form, FormContext};

fn demo_input_panel(_form_ctx: &FormContext) -> Html {
    InputPanel::new()
        .with_field("Textfield", Field::new())
        .with_field("Password", Field::new().input_type("password"))
        .with_field("Range field", Field::new().input_type("range"))
        .with_field(
            "Numberfield",
            Field::new().input_type("number"),
        )
        .with_field(
            "Numberfield with limits",
            Field::new().number(0.0, 150.0, 2.0),
        )
        .with_field("Checkbox", Checkbox::new())
        .with_field("Radio Group Option1", Checkbox::new().group("g1").name("opt1"))
        .with_field("Radio Group Option2", Checkbox::new().group("g1").name("opt2"))
        .with_field("Radio Group Option3", Checkbox::new().group("g1").name("opt3"))
        .with_field(
            "Combobox",
            Combobox::new()
                .with_item("Item 1")
                .with_item("Item 2"),
        )
        .into()
}

#[function_component]
pub fn FieldsDemo() -> Html {

    Container::new()
        .padding(2)
    // We place the InpuPanel inside a Form so that we have a
    // FormContext (The FormContext is required for RadioButtons).
        .with_child(Form::new(demo_input_panel))
        .into()
}
