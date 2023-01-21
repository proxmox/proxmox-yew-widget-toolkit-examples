use yew::prelude::*;

use pwt::prelude::*;
use pwt::widget::{Button, Container, Column, Row, TabPanel, TabPanelRenderInfo};

use pwt::css::ColorScheme;
static COLOR_SCHEMES: &[ColorScheme] = &[
    ColorScheme::Neutral,
    ColorScheme::NeutralAlt,
    ColorScheme::Primary,
    ColorScheme::Secondary,
    ColorScheme::Error,
];

fn color_scheme(color: &str, title: &str) -> Html {
    let percentage = [99, 95, 90, 80, 70, 60, 50, 40, 30, 20, 10];
    let colors: Html = percentage.iter().map(|p| {
        let class = format!(
            "pwt-bg-color-{}-{} pwt-color-{}-{} pwt-p-2",
            color,
            p,
            color,
            if *p <= 50 { 99 } else { 10 },
        );

        html!{
            <div {class}>{format!("{}-{}", color, p)}</div>
        }
    }).collect();

    let class = format!(
        "pwt-bg-color-{} pwt-color-on-{} pwt-p-4",
        color,
        color,
    );

    let variant_class = format!(
        "pwt-bg-color-{}-container pwt-color-on-{}-container pwt-p-4",
        color,
        color,
    );

    html!{
        <div>
            <div {class}>{title}</div>
            <div class={variant_class}>{"Container variant"}</div>
            {colors}
        </div>
    }
}

fn color_panel(_: &TabPanelRenderInfo) -> Html {

    Column::new()
        .padding(2)
        .gap(4)
        //.class("pwt-flex-fill")
        .with_child(color_scheme("primary", "Primary Color"))
        .with_child(color_scheme("secondary", "Secondary Color"))
        .with_child(color_scheme("error", "Error Color"))
        .with_child(color_scheme("neutral", "Neutral Color"))
        .into()
}

fn box_panel(_: &TabPanelRenderInfo) -> Html {

    let mut panel = Column::new()
        .padding(2)
        .gap(4);

    for scheme in COLOR_SCHEMES {
        panel.add_child(
            Container::new()
                .class(*scheme)
                .border(true)
                .padding(2)
                .with_child(format!("Color scheme '{:?}'", scheme))
        );

        let mut buttons = Row::new().gap(2);

        for scheme in COLOR_SCHEMES {
            buttons.add_child(
                Button::new(format!("{:?}", scheme))
                    .class(*scheme)
            )
        }
        panel.add_child(
            Container::new()
                .class(*scheme)
                .border(true)
                .padding(2)
                .with_child(buttons)
        );
    }

    panel.into()
}

#[function_component]
pub fn ColorsDemo() -> Html {
    TabPanel::new()
        .class("pwt-fit")
        .title("Color Scheme preview")
        .with_item_builder("boxes", "Boxes", None::<&str>, box_panel)
        .with_item_builder("colors", "Colors", None::<&str>, color_panel)
        .into()
}
