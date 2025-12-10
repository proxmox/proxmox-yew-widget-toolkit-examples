use yew::prelude::*;

use pwt::css::ColorScheme;
use pwt::prelude::*;
use pwt::touch::{Fab, FabMenu, FabMenuAlign, FabMenuDirection, FabMenuEntry};
use pwt::widget::{ActionIcon, Button, Column, FileButton, Row, SegmentedButton};

#[function_component]
pub fn ButtonsDemo() -> Html {
    fn render_buttons(disabled: bool, pressed: bool) -> Html {
        Row::new()
            .class("pwt-align-items-center")
            .gap(2)
            .with_child(Button::new("Normal").disabled(disabled).pressed(pressed))
            .with_child(
                Button::new("Primary")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class(ColorScheme::Primary),
            )
            .with_child(
                Button::new("PrimaryContainer")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class(ColorScheme::PrimaryContainer),
            )
            .with_child(
                Button::new("Secondary")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class(ColorScheme::Secondary),
            )
            .with_child(
                Button::new("SecondaryContainer")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class(ColorScheme::SecondaryContainer),
            )
            .with_child(
                Button::new("Tertiary")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class(ColorScheme::Tertiary),
            )
            .with_child(
                Button::new("TertiaryContainer")
                    .disabled(disabled)
                    .pressed(pressed)
                    .class(ColorScheme::TertiaryContainer),
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
        let mut row = Row::new().gap(2).class("pwt-align-items-center");
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
        .class("pwt-content-spacer-padding")
        .class(pwt::css::FlexFit)
        .class("pwt-align-items-start")
        .gap(2)
        .with_child("Favorite Action Buttons (FAB) types.")
        .with_child(
            Row::new()
                .gap(4)
                .class("pwt-align-items-center")
                .with_child(Fab::new("fa fa-pencil").size(pwt::touch::FabSize::Large))
                .with_child(Fab::new("fa fa-pencil").class("pwt-scheme-secondary"))
                .with_child(
                    Fab::new("fa fa-pencil")
                        .size(pwt::touch::FabSize::Small)
                        .class("pwt-scheme-tertiary"),
                )
                .with_child(
                    FabMenu::new()
                        .main_button_class("pwt-scheme-primary")
                        .direction(FabMenuDirection::Down)
                        .align(FabMenuAlign::Start)
                        .with_child(FabMenuEntry::new("Edit", "fa fa-pencil", |_| {}))
                        .with_child(FabMenuEntry::new("Delete", "fa fa-trash", |_| {})),
                ),
        )
        .with_child("Button Types")
        .with_child(
            Row::new()
                .class("pwt-align-items-center")
                .gap(2)
                .with_child(
                    Fab::new("fa fa-trash")
                        .size(pwt::touch::FabSize::Small)
                        .text("FAB"),
                )
                .with_child(Button::new("Filled").class(ColorScheme::Primary))
                .with_child(Button::new("Tonal").class(ColorScheme::PrimaryContainer))
                .with_child(Button::new("Elevated").class("pwt-button-elevated"))
                .with_child(
                    Button::new("Outline")
                        .class(ColorScheme::Tertiary)
                        .class("pwt-button-outline"),
                )
                .with_child(Button::new("Text").class("pwt-button-text")),
        )
        .with_child("Enabled Filled Buttons")
        .with_child(render_buttons(false, false))
        .with_child("Disabled Filled Buttons")
        .with_child(render_buttons(true, false))
        .with_child("Pressed Filled Buttons")
        .with_child(render_buttons(false, true))
        .with_child("Button with Icon (elevated)")
        .with_child(
            Row::new()
                .class("pwt-align-items-center")
                .gap(2)
                .with_child(
                    Button::new("Delete")
                        .class("pwt-button-elevated")
                        .icon_class("fa fa-trash-o"),
                )
                .with_child(
                    Button::new("Delete")
                        .class("pwt-button-elevated")
                        .disabled(true)
                        .icon_class("fa fa-trash-o"),
                ),
        )
        .with_child("Round Button with Icon")
        .with_child(
            Button::new("Delete")
                .icon_class("fa fa-trash-o")
                .onclick(|_| log::info!("CLICK"))
                .class("rounded"),
        )
        .with_child("File Buttons")
        .with_child(
            Row::new()
                .gap(2)
                .with_child(
                    FileButton::new("Downlaod")
                        .icon_class("fa fa-download")
                        .accept("image/*")
                        .multiple(true),
                )
                .with_child(FileButton::new_icon("fa fa-download")),
        )
        .with_child("Round Icon Button")
        .with_child(Button::new_icon("fa fa-trash-o").class("circle"))
        .with_child("Action Icons (Icon Buttons without decoration)")
        .with_child(
            Row::new()
                .gap(2)
                .class("pwt-align-items-center")
                .with_child(
                    ActionIcon::new("fa fa-trash-o")
                        .tabindex(0)
                        .aria_label("remove"),
                )
                .with_child(
                    ActionIcon::new("fa fa-refresh")
                        .tabindex(0)
                        .aria_label("refresh"),
                )
                .with_child(
                    ActionIcon::new("fa fa-italic")
                        .tabindex(0)
                        .aria_label("italic"),
                )
                .with_child(ActionIcon::new("fa fa-bold").tabindex(0).aria_label("bold"))
                .with_child(
                    ActionIcon::new("fa fa-align-justify")
                        .tabindex(0)
                        .aria_label("align-justify"),
                ),
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
                .class("pwt-align-items-center")
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
        .with_child("Elevated Segmented Button")
        .with_child(
            Row::new()
                .class("pwt-align-items-center")
                .gap(2)
                .with_child(
                    SegmentedButton::new()
                        .class(ColorScheme::Primary)
                        .class("pwt-button-elevated")
                        .with_button(Button::new("First"))
                        .with_button(Button::new("Second"))
                        .with_button(Button::new("Third")),
                )
                .with_child(
                    SegmentedButton::new()
                        .class(ColorScheme::Secondary)
                        .class("rounded pwt-button-elevated")
                        .with_button(Button::new("First"))
                        .with_button(Button::new("Second"))
                        .with_button(Button::new("Third")),
                )
                .with_child(
                    SegmentedButton::new()
                        .class("pwt-button-elevated")
                        .with_button(Button::new("First").class(ColorScheme::Tertiary))
                        .with_button(Button::new("Second"))
                        .with_button(Button::new("Third").class("rounded")),
                ),
        )
        .with_child("Outlined Segmented Button")
        .with_child(
            Row::new()
                .class("pwt-align-items-center")
                .gap(2)
                .with_child(
                    SegmentedButton::new()
                        .class("pwt-button-outline")
                        .class(ColorScheme::Primary)
                        .with_button(Button::new("First"))
                        .with_button(Button::new("Second"))
                        .with_button(Button::new("Third")),
                )
                .with_child(
                    SegmentedButton::new()
                        .class(ColorScheme::Secondary)
                        .class("rounded pwt-button-outline")
                        .with_button(Button::new("First"))
                        .with_button(Button::new("Second"))
                        .with_button(Button::new("Third")),
                )
                .with_child(
                    SegmentedButton::new()
                        .class("pwt-button-outline")
                        .with_button(Button::new("First").class(ColorScheme::Tertiary))
                        .with_button(Button::new("Second"))
                        .with_button(Button::new("Third").class("rounded")),
                ),
        )
        .into()
}
