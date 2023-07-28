use pwt::prelude::*;
use yew::prelude::*;

use pwt::widget::form::Field;
use pwt::widget::{Button, Column, Row, Tooltip};

#[function_component]
pub fn TooltipDemo() -> Html {
    Column::new()
        .padding(2)
        .gap(2)
        .class("pwt-align-items-start")
        .with_child("Button with tooltip")
        .with_child(
            Tooltip::new()
                .tip(html!{"This button has a tooltip"})
                .with_child(Button::new("TooltipButton"))
        )
        .with_child(
            Tooltip::new()
                .class("pwt-mt-4")
                .tip(html!{"Simple text with a tooltip."})
                .with_child("Anything may have a tooltip, i.e this text.")

        )
        .with_child(html!{<div/>})
        .with_child(
            Tooltip::new()
                .class("pwt-mt-4")
                .rich_tip({
                    let html: Html = Column::new()
                        .with_child(html!{<h6>{"Rich Tooltip"}</h6>})
                        .with_child(html!{<p>{"Rich Tooltips may contain arbitrary html content. It is possible to add interactive elements."}</p>})
                        .with_child(Row::new().with_child(Button::new("Action").class("pwt-mt-2 pwt-button-text")))
                        .into();
                    html
                })
                .with_child("Tooltips may have rich html content.")
        )
        .with_child(html!{<div class="pwt-mt-4">{"Inputs use tooltips to display validation results:"}</div>})
        .with_child(html!{
            <form novalidate={true}> // disable broser validation popup
            {
                Row::new()
                    .gap(1)
                    .class("pwt-align-items-center")
                    .with_child("Number between 3 and 10: ")
                    .with_child(
                        Field::new()
                            .number(3.0, 10.0, 1.0)
                            .default("2") // invalid to show tooltip
                            .required(true)
                    )
            }
            </form>
        })
        .into()
}
