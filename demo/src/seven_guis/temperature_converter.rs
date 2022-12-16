use pwt::prelude::*;
use yew::prelude::*;

use pwt::widget::{Button, Column, InputPanel, Panel, Row};
use pwt::widget::form::{Form, Field};

#[function_component]
pub fn TemperatureConverter() -> Html {

    let form = Form::new(|form_ctx| {
        InputPanel::new()
            .padding(2)
            .border(false)
            .with_field(
                "Celsius",
                Field::new()
                    .name("celsius")
                    .number(-273.15, None, 1.0)
                    .on_change({
                        let form_ctx = form_ctx.clone();
                        move |celsius: String| {
                            if let Ok(celsius) = celsius.parse::<f64>() {
                                let fh = (celsius*5.0/9.0) + 32.0;
                                log::info!("CELSIUS {celsius} FH {fh}");
                            }
                        }
                    })
            )
            .with_right_field(
                 "Fahrenheit",
                 Field::new()
                    .name("fahrenheit")
                    .number(-459.67, None, 1.0)
                    .on_change({
                        let form_ctx = form_ctx.clone();
                        move |fh: String| {
                            if let Ok(fh) = fh.parse::<f64>() {
                                let celsius = (fh - 32.0)*5.0/9.0;
                                log::info!("FH {fh} CELSIUS {celsius}");
                           }
                        }
                    })
              )
            .into()
    });

    Panel::new()
        .title("Temperature Converter")
        .with_child(form)
        .into()
}
