use yew::prelude::*;

use pwt::prelude::*;
use pwt::widget::{ActionIcon, Button, Column, Row, SegmentedButton};
use pwt::css::ColorScheme;

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
                    .class(ColorScheme::Primary),
            )
            .with_child(
                Button::new("Secondary")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class(ColorScheme::Secondary),
            )
            .with_child(
                Button::new("Tertiray")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class(ColorScheme::Tertiary),
            )
            .with_child(
                Button::new("Neutral")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class(ColorScheme::Neutral),
            )
            .with_child(
                Button::new("Surface")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class(ColorScheme::Surface),
            )
            .with_child(
                Button::new("NeutralAlt")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class(ColorScheme::NeutralAlt),
            )
            .with_child(
                Button::new("Error")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class(ColorScheme::Error),
            )
            .into()
    }

    fn render_segmented_buttons(icons: bool) -> Html {
        let mut row = Row::new().gap(2);
        for class in &[
            None,
            Some(ColorScheme::Primary),
            Some(ColorScheme::Secondary),
            Some(ColorScheme::Tertiary),
        ] {
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

            if let Some(class) = *class {
                btn.add_class(class);
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
        .with_child("Button with Icon (raised)")
        .with_child(Button::new("Delete").class("raised").icon_class("fa fa-trash-o"))
        .with_child("Round Button with Icon")
        .with_child(
            Button::new("Delete")
                .icon_class("fa fa-trash-o")
                .class("rounded"),
        )
        .with_child("Round Icon Button")
        .with_child(Button::new_icon("fa fa-trash-o").class("rounded"))

        .with_child("Action Icons (Icon Buttons without decoration)")
        .with_child(
            Row::new()
                .gap(2)
                .with_child(ActionIcon::new("fa fa-trash-o").tabindex(0).aria_label("remove"))
                .with_child(ActionIcon::new("fa fa-refresh").tabindex(0).aria_label("refresh"))
                .with_child(ActionIcon::new("fa fa-italic").tabindex(0).aria_label("italic"))
                .with_child(ActionIcon::new("fa fa-bold").tabindex(0).aria_label("bold"))
                .with_child(ActionIcon::new("fa fa-align-justify").tabindex(0).aria_label("align-justify"))
        )

        .with_child("Segmented Button")
        .with_child(render_segmented_buttons(false))
        .with_child("Segmented Button with Icon")
        .with_child(render_segmented_buttons(true))
        .with_child("Segmented Button with mixed colors")
        .with_child(
            SegmentedButton::new()
                .class(ColorScheme::Neutral)
                .with_button(Button::new("First"))
                .with_button(Button::new("Second").class(ColorScheme::Primary))
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
        .with_child("Rounded Segmented Button (raised)")
        .with_child(
            SegmentedButton::new()
                .class("rounded raised")
                .with_button(Button::new("First"))
                .with_button(Button::new("Second"))
                .with_button(Button::new("Third")),
        )
        .into()
}
