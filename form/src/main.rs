use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{InputPanel, Panel, Row};
use pwt::widget::form2::{Field, Form, Checkbox, Combobox, SubmitButton, ResetButton};
use pwt::widget::form2::context::FormContext;


pub struct MyForm {
    form_ctx: FormContext,
}

impl Component for MyForm {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let form_ctx = FormContext::new()
            .on_change(|form_ctx: FormContext| {
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
            });
        Self { form_ctx }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let form_ctx = self.form_ctx.clone();

        log::info!("test");
        let input_panel = InputPanel::new()
            .padding(2)
            .border(false)
            .with_field(
                "Password",
                Field::new()
                    .name("password")
                    .required(true)
                    .input_type("password")
            )
            .with_field(
                "Confirm Password",
                Field::new()
                    .name("confirm_password")
                    .required(true)
                    .submit(false)
                    .input_type("password")
            )
            .with_field(
                "Number1",
                Field::new()
                    .name("number1")
                    .number(3.0, 10.0, 1.0)
                    .default("4")
            )
            .with_field("Checkbox1", Checkbox::new().name("checkbox1").default(true))
            .with_field(
                "Combobox1",
                Combobox::new()
                    .name("combobox1")
                    .submit_empty(true)
                    .placeholder("Please select something.")
                //.editable(true)
                    .with_item("Option 1")
                    .with_item("Option 2")
                    .with_item("Option 3")
            )
            .with_child(html!{
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


        Form::new()
            .form_context(form_ctx)
            .with_child(input_panel)
            .into()
    }
}

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        Panel::new()
            .title("A Simple Form")
            .with_child(html!{<MyForm/>})
            .into()
    }

}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
