use pwt::{prelude::*, widget::SegmentedButton};
use yew::prelude::*;

use pwt::widget::{Button, Column, Row};

#[function_component]
pub fn ButtonsDemo() -> Html {
    fn render_buttons(disabled: bool, pressed: bool) -> Html {
        Row::new()
            .gap(2)
            .with_child(Button::new("Normal").disabled(disabled).pressed(pressed))
            .with_child(
                Button::new("Primary")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class("primary"),
            )
            .with_child(
                Button::new("Secondary")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class("secondary"),
            )
            .with_child(
                Button::new("Neutral")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class("neutral"),
            )
            .with_child(
                Button::new("Error")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class("error"),
            )
            .into()
    }

    fn render_segmented_buttons(icons: bool) -> Html {
        let mut row = Row::new().gap(2);
        for class in &["", "primary", "secondary", "neutral", "error"] {
            let mut btn = if icons {
                SegmentedButton::new()
                    .with_button(Button::new("First").icon_class("fa fa-paper-plane"))
                    .with_button(Button::new("Second").icon_class("fa fa-fire-extinguisher"))
                    .with_button(Button::new("Third").icon_class("fa fa-coffee"))
            } else {
                SegmentedButton::new()
                    .with_button(Button::new("First"))
                    .with_button(Button::new("Second"))
                    .with_button(Button::new("Third"))
            };

            if *class != "" {
                btn.add_class(*class);
            }

            row.add_child(btn)
        }
        row.into()
    }

    Column::new()
        .padding(2)
        .gap(2)
        .class("pwt-align-items-start")
        .with_child("Enabled Buttons")
        .with_child(render_buttons(false, false))
        .with_child("Disabled Buttons")
        .with_child(render_buttons(true, false))
        .with_child("Pressed Buttons")
        .with_child(render_buttons(false, true))
        .with_child("Button with Icon")
        .with_child(Button::new("Delete").icon_class("fa fa-trash-o"))
        .with_child("Round Button with Icon")
        .with_child(
            Button::new("Delete")
                .icon_class("fa fa-trash-o")
                .class("rounded"),
        )
        .with_child("Round Icon Button")
        .with_child(Button::new_icon("fa fa-trash-o").class("rounded"))
        .with_child("Segmented Button")
        .with_child(render_segmented_buttons(false))
        .with_child("Segmented Button with Icon")
        .with_child(render_segmented_buttons(true))
        .with_child("Segmented Button with mixed colors")
        .with_child(
            SegmentedButton::new()
                .class("neutral")
                .with_button(Button::new("First"))
                .with_button(Button::new("Second").class("primary"))
                .with_button(Button::new("Third")),
        )
        .with_child("Rounded Segmented Button")
        .with_child(
            Row::new()
                .gap(2)
                .with_child(
                    SegmentedButton::new()
                        .class("rounded")
                        .with_button(Button::new("First"))
                        .with_button(Button::new("Second"))
                        .with_button(Button::new("Third")),
                )
                .with_child(
                    SegmentedButton::new()
                        .with_button(Button::new("First").class("rounded"))
                        .with_button(Button::new("Second"))
                        .with_button(Button::new("Third")),
                )
                .with_child(
                    SegmentedButton::new()
                        .with_button(Button::new("First"))
                        .with_button(Button::new("Second"))
                        .with_button(Button::new("Third").class("rounded")),
                ),
        )
        .into()
}
