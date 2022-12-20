use pwt::prelude::*;
use yew::prelude::*;

use pwt::widget::{InputPanel, Panel, Row};
use pwt::state::form::FormContext;
use pwt::widget::form2::{Field, Form};

pub struct TemperatureConverter2 {
    form_ctx: FormContext,
}

pub enum Msg {
    Celsius(String),
    Fahrenheit(String),
}

impl Component for TemperatureConverter2 {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let form_ctx = FormContext::new()
            .on_change(|form_ctx: FormContext| {
                log::info!("CTX CHANGE C: {:?}", form_ctx.read().get_field_value("celsius"));
                log::info!("CTX CHANGE F: {:?}", form_ctx.read().get_field_value("fahrenheit"));
            });

        Self { form_ctx }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Celsius(c) => {
                log::info!("UPDATE C '{c}'");
                if let Ok(celsius) = c.parse::<f64>() {
                    let fahrenheit = (celsius*9.0/5.0) + 32.0;
                    self.form_ctx.write()
                        .set_field_value("fahrenheit", format!("{}", fahrenheit).into());
                }
            }
            Msg::Fahrenheit(f) => {
               log::info!("UPDATE F '{f}'");
                if let Ok(fahrenheit) = f.parse::<f64>() {
                    let celsius = (fahrenheit - 32.0)*5.0/9.0;
                    self.form_ctx.write()
                        .set_field_value("celsius", format!("{}", celsius).into());
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let body = Form::new()
            .form_context(self.form_ctx.clone())
            .with_child(
                InputPanel::new()
                    .padding(2)
                    .border(false)
                    .with_field(
                        "Celsius",
                        Field::new()
                            .name("celsius")
                            .number(-273.15, None, 1.0)
                            .on_change(link.callback(Msg::Celsius))
                    )
                    .with_right_field(
                        "Fahrenheit",
                        Field::new()
                            .name("fahrenheit")
                            .number(-459.67, None, 1.0)
                            .on_change(link.callback(Msg::Fahrenheit))
                    )
            );

        Row::new().with_child(
            Panel::new()
                .title("Temperature Converter (FormContext)")
                .with_child(body)
        ).into()
    }
}
