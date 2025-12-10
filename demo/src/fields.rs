use std::rc::Rc;

use pwt::prelude::*;
use pwt::state::Store;
use pwt::widget::data_table::{DataTableColumn, DataTableHeader};
use pwt::widget::form::{
    Checkbox, Combobox, DisplayField, Field, Form, FormContext, InputType, Number, RadioButton,
    TextArea, Tristate, TristateBoolean,
};
use pwt::widget::{AlertDialog, InputPanel, SearchDropdown, Trigger};
use yew::virtual_dom::Key;

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
        .with_field("Display", DisplayField::new().value("Some Value"))
        .with_field("Display with long text", DisplayField::new().value("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed tempor nisi odio, sit amet efficitur purus pharetra ac."))
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
            RadioButton::new("opt1").name("g1"),
        )
        .with_field(
            "Radio Group Option2",
            RadioButton::new("opt2").name("g1").default(true).box_label("The default option"),
        )
        .with_field(
            "Radio Group Option3",
            RadioButton::new("opt3").name("g1"),
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
        .with_field("SearchDropdown", search_dropdown())
        .with_field(
            "TextArea",
            TextArea::new().placeholder("Type some text here."),
        );

    Form::new().form_context(form_ctx).with_child(panel).into()
}

#[derive(Clone, PartialEq)]
struct UserEntry {
    id: u32,
    first_name: &'static str,
    last_name: &'static str,
    country: &'static str,
}
static USER_LIST: &[UserEntry] = &[
    UserEntry {
        id: 1,
        first_name: "Max",
        last_name: "Mustermann",
        country: "Austria",
    },
    UserEntry {
        id: 2,
        first_name: "Dieter",
        last_name: "Mustermann",
        country: "Austria",
    },
    UserEntry {
        id: 3,
        first_name: "Helene",
        last_name: "Musterfrau",
        country: "Germany",
    },
];

fn user_search_columns() -> Rc<Vec<DataTableHeader<UserEntry>>> {
    Rc::new(vec![
        DataTableColumn::new(tr!("ID"))
            .flex(2)
            .show_menu(false)
            .width("40px")
            .justify("flex-start")
            .render(|item: &UserEntry| html!(item.id))
            .sorter(|a: &UserEntry, b: &UserEntry| a.id.cmp(&b.id))
            .sort_order(true)
            .into(),
        DataTableColumn::new(tr!("First Name"))
            .flex(2)
            .show_menu(false)
            .justify("flex-start")
            .render(|item: &UserEntry| html!(item.first_name))
            .sorter(|a: &UserEntry, b: &UserEntry| a.first_name.cmp(b.first_name))
            .sort_order(true)
            .into(),
        DataTableColumn::new(tr!("Last Name"))
            .flex(2)
            .show_menu(false)
            .justify("flex-start")
            .render(|item: &UserEntry| html!(item.last_name))
            .sorter(|a: &UserEntry, b: &UserEntry| a.last_name.cmp(b.last_name))
            .sort_order(true)
            .into(),
        DataTableColumn::new(tr!("Country"))
            .flex(2)
            .show_menu(false)
            .justify("flex-start")
            .render(|item: &UserEntry| html!(item.country))
            .sorter(|a: &UserEntry, b: &UserEntry| a.country.cmp(b.country))
            .sort_order(true)
            .into(),
    ])
}

fn search_dropdown() -> SearchDropdown<Store<UserEntry>> {
    SearchDropdown::table(user_search_columns(), move |filter: String| async move {
        let mut data = Vec::new();
        let filter = filter.to_lowercase();
        for user in USER_LIST.iter() {
            if user.first_name.to_lowercase().contains(&filter)
                || user.last_name.to_lowercase().contains(&filter)
                || user.country.to_lowercase().contains(&filter)
            {
                data.push(user.clone());
            }
        }
        let store = Store::with_extract_key(|item: &UserEntry| Key::from(item.id));
        store.set_data(data);
        Ok(store)
    })
    .on_select(|id| {
        log::info!("SearchDropdown select {id}");
    })
}

#[function_component]
pub fn FieldsDemo() -> Html {
    let alert = use_state(|| None);

    // We place the InpuPanel inside a Form so that we have a
    // FormContext (The FormContext is required for RadioButtons).
    Form::new()
        .class("pwt-content-spacer-padding")
        .class("pwt-size-fit-content")
        .class(pwt::css::FlexFit)
        .border(false)
        .with_child(demo_input_panel(alert.clone()))
        .with_optional_child(
            (*alert)
                .clone()
                .map(|text| AlertDialog::new(text).on_close(move |_| alert.set(None))),
        )
        .into()
}
