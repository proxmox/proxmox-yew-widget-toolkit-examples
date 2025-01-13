use pwt::prelude::*;
use pwt::widget::form::Field;
use pwt::widget::{InputPanel, Panel, Row};

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
                self.fahrenheit = self.celsius.map(|c| (c * 9.0 / 5.0) + 32.0);
            }
            Msg::Fahrenheit(f) => {
                self.fahrenheit = f.parse::<f64>().ok();
                self.celsius = self.fahrenheit.map(|f| (f - 32.0) * 5.0 / 9.0);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let celsius = self.celsius.map(|c| c.to_string()).unwrap_or_default();
        let fahrenheit = self.fahrenheit.map(|f| f.to_string()).unwrap_or_default();

        let body = InputPanel::new()
            .padding(2)
            .border(false)
            .with_field(
                "Celsius",
                Field::new()
                    .value(celsius)
                    .number(-273.15, None, 1.0)
                    .on_input(ctx.link().callback(Msg::Celsius)),
            )
            .with_right_field(
                "Fahrenheit",
                Field::new()
                    .value(fahrenheit)
                    .number(-459.67, None, 1.0)
                    .on_input(ctx.link().callback(Msg::Fahrenheit)),
            );

        Row::new()
            .with_child(
                Panel::new()
                    .border(true)
                    .title("Temperature Converter (Controlled)")
                    .with_child(body),
            )
            .into()
    }
}
