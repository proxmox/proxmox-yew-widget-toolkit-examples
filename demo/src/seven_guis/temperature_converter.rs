use pwt::prelude::*;
use yew::prelude::*;

use pwt::widget::{InputPanel, Panel};
use pwt::widget::form::Field;

pub struct TemperatureConverter {
    celsius: Option<f64>,
    fahrenheit: Option<f64>,
}

pub enum Msg {
    Celsius(String),
    Fahrenheit(String),
}

impl Component for TemperatureConverter {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            celsius: None,
            fahrenheit: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Celsius(c) => {
                self.celsius = c.parse::<f64>().ok();
                self.fahrenheit = self.celsius.map(|c| (c*9.0/5.0) + 32.0);
            }
            Msg::Fahrenheit(f) => {
                self.fahrenheit = f.parse::<f64>().ok();
                self.celsius = self.fahrenheit.map(|f| (f - 32.0)*5.0/9.0);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let celsius = self.celsius.map(|c| c.to_string()).unwrap_or(String::new());
        let fahrenheit = self.fahrenheit.map(|f| f.to_string()).unwrap_or(String::new());

        let body = InputPanel::new()
            .padding(2)
            .border(false)
            .with_field(
                "Celsius",
                Field::new()
                    .name("celsius")
                    .value(celsius)
                    .number(-273.15, None, 1.0)
                    .on_change(ctx.link().callback(Msg::Celsius))
            )
            .with_right_field(
                "Fahrenheit",
                Field::new()
                    .name("fahrenheit")
                    .value(fahrenheit)
                    .number(-459.67, None, 1.0)
                    .on_change(ctx.link().callback(Msg::Fahrenheit))
            );

        Panel::new()
            .attribute("style", "width: 400px;")
            .title("Temperature Converter")
            .with_child(body)
            .into()
    }
}
