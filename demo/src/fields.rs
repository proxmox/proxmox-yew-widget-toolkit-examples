use pwt::prelude::*;
use pwt::widget::form::{
    Checkbox, Combobox, DisplayField, Field, Form, FormContext, InputType, Number, TextArea,
    Tristate, TristateBoolean,
};
use pwt::widget::{AlertDialog, Container, InputPanel, Trigger};

fn demo_input_panel(alert: UseStateHandle<Option<String>>) -> Html {
    let mut long_combo = Combobox::new();
    for i in 0..50 {
        long_combo.add_item(format!("Item {}", i))
    }

    let form_ctx = FormContext::new();
    let panel = InputPanel::new()
        .width(500)
        .with_field(
            "Textfield (primary)",
            Field::new().class("pwt-scheme-primary"),
        )
        .with_field(
            "Textfield (with placeholder and trigger)",
            Field::new()
            .name("search")
                .with_trigger(Trigger::new("fa fa-search").tip("Search").onclick({
                    let form_ctx = form_ctx.clone();
                    move |_| {
                        if (*alert).is_none() {
                            let text =  form_ctx.read().get_field_text("search");
                            alert.set(Some(format!("Search for: '{text}'")));
                        } else {
                            alert.set(None);
                        }
                    }}), true)
                .placeholder("Search"),
        )
        .with_field("Display", DisplayField::new("Some Value"))
        .with_field("Display with long text", DisplayField::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed tempor nisi odio, sit amet efficitur purus pharetra ac."))
        .with_field("Password", Field::new().input_type(InputType::Password))
        .with_field("Range field", Field::new().range(0.0, 100.0, 1.0))
        .with_field("Numberfield", Number::<i64>::new())
        .with_field(
            "Numberfield with limits",
            Number::new().min(0.0).max(150.0).step(2.0),
        )
        .with_field("Disabled Textfield", Field::new().disabled(true))
        .with_field("Checkbox", Checkbox::new().default(true))
        .with_field(
            "Disabled Checkbox",
            Checkbox::new().disabled(true).default(true),
        )
        .with_field("Switch", Checkbox::new().switch(true))
        .with_field(
            "TristateBoolean",
            TristateBoolean::new().default(Tristate::Yes),
        )
        .with_field(
            "Radio Group Option1",
            Checkbox::radio().name("g1").value("opt1"),
        )
        .with_field(
            "Radio Group Option2",
            Checkbox::radio().name("g1").value("opt2").default(true),
        )
        .with_field(
            "Radio Group Option3",
            Checkbox::radio().name("g1").value("opt3"),
        )
        .with_field(
            "Combobox",
            Combobox::new()
                .default("Item 1")
                .with_item("Item 1")
                .with_item("Item 2"),
        )
        .with_field(
            "Combobox (editable)",
            Combobox::new()
                .editable(true)
                .default("Item 1")
                .with_item("Item 1")
                .with_item("Item 2"),
        )
        .with_field(
            "Combobox (with icon)",
            Combobox::new()
                .placeholder("Select a user")
                .with_trigger("fa fa-user", false)
                .with_item("Admin")
                .with_item("User"),
        )
        .with_field(
            "Combobox (disabled)",
            Combobox::new()
                .disabled(true)
                .placeholder("Select a user")
                .with_trigger("fa fa-user", false)
                .with_item("Admin")
                .with_item("User"),
        )
        .with_field("Long Combobox", long_combo)
        .with_field(
            "TextArea",
            TextArea::new().placeholder("Type some text here."),
        );

    Form::new().form_context(form_ctx).with_child(panel).into()
}

#[function_component]
pub fn FieldsDemo() -> Html {
    let alert = use_state(|| None);
    Container::new()
        .padding(2)
        .with_child(
            // We place the InpuPanel inside a Form so that we have a
            // FormContext (The FormContext is required for RadioButtons).
            Form::new().with_child(demo_input_panel(alert.clone())),
        )
        .with_optional_child(
            (*alert)
                .clone()
                .map(|text| AlertDialog::new(text).on_close(move |_| alert.set(None))),
        )
        .into()
}
