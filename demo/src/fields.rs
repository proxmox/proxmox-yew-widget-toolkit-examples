use pwt::prelude::*;
use pwt::widget::{Container, InputPanel};
use pwt::widget::form::{Checkbox, Combobox, Field, Form};

fn demo_input_panel() -> Html {
    let mut long_combo = Combobox::new();
    for i in 0..50 {
        long_combo.add_item(format!("Item {}", i))
    }
    InputPanel::new()
        .with_field("Textfield (primary)", Field::new().class("pwt-scheme-primary"))
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
        .with_field("Checkbox", Checkbox::new().default(true))
        .with_field("Switch", Checkbox::new().switch(true))
        .with_field("Radio Group Option1", Checkbox::radio().name("g1").value("opt1"))
        .with_field("Radio Group Option2", Checkbox::radio().name("g1").value("opt2").default(true))
        .with_field("Radio Group Option3", Checkbox::radio().name("g1").value("opt3"))
        .with_field(
            "Combobox",
            Combobox::new()
                .default("Item 1")
                .with_item("Item 1")
                .with_item("Item 2"),
        )
        .with_field("Long Combobox", long_combo)
        .into()
}

#[function_component]
pub fn FieldsDemo() -> Html {

    Container::new()
        .padding(2)
        .with_child(
            // We place the InpuPanel inside a Form so that we have a
            // FormContext (The FormContext is required for RadioButtons).
            Form::new()
                .with_child(demo_input_panel())
        )
        .into()
}
