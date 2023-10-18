use serde_json::Value;

use pwt::prelude::*;

use pwt::widget::form::{
    Boolean, Checkbox, Combobox, Field, Form, FormContext, Number, ResetButton, SubmitButton,
    Tristate, TristateBoolean, InputType,
};
use pwt::widget::{InputPanel, Panel, Row};

pub struct MyForm {
    form_ctx: FormContext,
    data: Option<Value>,
}

pub enum Msg {
    ShowData(Value),
}

impl Component for MyForm {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let form_ctx = FormContext::new().on_change(move |form_ctx: FormContext| {
            let pw = form_ctx.read().get_field_text("password");
            let confirm = form_ctx.read().get_field_text("confirm_password");
            if !confirm.is_empty() {
                let valid = if pw == confirm {
                    Ok(())
                } else {
                    Err(String::from("Passwords do not match!"))
                };
                form_ctx.write().set_field_valid("confirm_password", valid);
            }
            let data = form_ctx.get_submit_data();
            link.send_message(Msg::ShowData(data));
        });
        Self {
            form_ctx,
            data: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowData(data) => {
                self.data = Some(data);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let form_ctx = self.form_ctx.clone();

        let input_panel = InputPanel::new()
            .padding(2)
            .border(false)
            .with_field(
                "Password",
                Field::new()
                    .name("password")
                    .required(true)
                    .input_type(InputType::Password),
            )
            .with_field(
                "Confirm Password",
                Field::new()
                    .name("confirm_password")
                    .required(true)
                    .submit(false)
                    .input_type(InputType::Password),
            )
            .with_field(
                "Number1 (text)",
                Field::new()
                    .name("number1")
                    .number(3.0, 10.0, 1.0)
                    .default("4"),
            )
            .with_field("Number2 (f64)", Number::<f64>::new().name("number2"))
            .with_field("Number3 (u64)", Number::<u64>::new().name("number3"))
            .with_field("Number4 (i8)", Number::<i8>::new().name("number4"))
            .with_field("Checkbox1", Checkbox::new().name("checkbox1").default(true))
            .with_field(
                "Checkbox2",
                Checkbox::new().name("checkbox2").switch(true).default(true),
            )
            .with_field("Boolean1", Boolean::new().name("boolean1").default(true))
            .with_field(
                "Boolean2",
                Boolean::new().name("boolean2").switch(true).default(true),
            )
            .with_field(
                "TristateBoolean",
                TristateBoolean::new()
                    .name("tristate1")
                    .submit_empty(true)
                    .default(Tristate::Yes),
            )
            .with_field(
                "Combobox1",
                Combobox::new()
                    .name("combobox1")
                    .submit_empty(true)
                    .placeholder("Please select something.")
                    //.editable(true)
                    .with_item("Option 1")
                    .with_item("Option 2")
                    .with_item("Option 3"),
            )
            .with_child(html! {
                <div style="grid-column-start: 2; grid-column-end: 3">{
                    Row::new()
                        .gap(2)
                        .with_child(ResetButton::new())
                        .with_child(
                            SubmitButton::new()
                                .on_submit({
                                    let form_ctx = form_ctx.clone();
                                    move |_|  {
                                        let data = form_ctx.get_submit_data();
                                        log::info!("Submit {:?}", data);
                                    }
                                })
                        )
                }</div>
            });

        let form = Form::new().form_context(form_ctx).with_child(input_panel);

        let data_view = self.data.clone().map(|data| {
            let text = serde_json::to_string_pretty(&data).unwrap();
            Panel::new()
                .title("Submit Data")
                .with_child(html! {<div class="pwt-p-2" style="white-space: pre;">{text}</div>})
        });

        Panel::new()
            .title("A Simple Form")
            .with_child(form)
            .with_optional_child(data_view)
            .into()
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<MyForm>::new().render();
}
