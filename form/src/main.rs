use anyhow::bail;

use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{InputPanel, Panel, Row};
use pwt::widget::form::*;


pub struct MyForm {}

impl Component for MyForm {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Form::new(|form_ctx| {
            InputPanel::new()
                .padding(2)
                .border(false)
                .with_field(
                    "Password",
                    Field::new()
                        .name("password")
                        .required(true)
                        .input_type("password")
                        .on_change({
                            let form_ctx = form_ctx.clone();
                            move |_value| form_ctx.validate_field("confirm_password")
                        })
                )
                .with_field(
                    "Confirm Password",
                    Field::new()
                        .name("confirm_password")
                        .required(true)
                        .submit(false)
                        .input_type("password")
                        .validate({
                            let form_ctx = form_ctx.clone();
                            move |value| {
                                let field1 = form_ctx.get_field_value("password");
                                if &field1 != value {
                                    bail!("Passwords do not match!");
                                }
                                Ok(())
                            }
                        })
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
                    Combobox::new("combobox1")
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
                })
                .into()
        }).into()
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
