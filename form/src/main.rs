use anyhow::bail;

use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{Container, Panel, Row};
use pwt::widget::form2::*;


pub struct MyForm {}

impl Component for MyForm {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Form::new(|form_ctx| {
            Container::new()
                .padding(2)
                .border(false)
                .class("pwt-form-grid-col2")
                .with_child(html!{<span style="grid-column-start: 1; grid-column-end: 2">{"Field1"}</span>})
                .with_child(html!{
                    <div style="grid-column-start: 2; grid-column-end: 3">{
                        Field::new("field1")
                            .required(true)
                            .on_change({
                                let form_ctx = form_ctx.clone();
                                move |v| {
                                    form_ctx.validate_field("field2");
                                    log::info!("Field1 changed to '{v}'");
                                }
                            })
                    }</div>
                })
                .with_child(html!{<span style="grid-column-start: 1; grid-column-end: 2">{"Field2"}</span>})
                .with_child(html!{
                    <div style="grid-column-start: 2; grid-column-end: 3">{
                        Field::new("field2")
                            .required(true)
                            .on_change(|v| {
                                log::info!("Field2 changed to '{v}'");
                            })
                            .validate({
                                let form_ctx = form_ctx.clone();
                                move |v| {
                                    log::info!("VALIDATE Field2");
                                    let field1 = form_ctx.get_field_value("field1");
                                    if &field1 != v {
                                        bail!("Field content is different");
                                    }
                                    Ok(())
                                }
                            })
                    }</div>
                })
                .with_child(html!{<span style="grid-column-start: 1; grid-column-end: 2">{"Checkbox1"}</span>})
                .with_child(html!{
                    <div style="grid-column-start: 2; grid-column-end: 3">{
                        Checkbox::new("checkbox1").default(true)
                    }</div>
                })
               .with_child(html!{
                    <div style="grid-column-start: 2; grid-column-end: 3">{
                        Row::new()
                            .gap(2)
                            .with_child(Reset::new())
                            .with_child(
                                Submit::new()
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

        //let input_panel = InputPanel::new(form_state)
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
