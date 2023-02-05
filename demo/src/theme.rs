use yew::prelude::*;

use pwt::prelude::*;
use pwt::widget::{
    Button, Column, Row, Panel, TabPanel, TabPanelRenderInfo,
};

use pwt::css::ColorScheme;
static COLOR_SCHEMES: &[ColorScheme] = &[
    ColorScheme::Neutral,
    ColorScheme::Surface,
    ColorScheme::NeutralAlt,
    ColorScheme::Primary,
    ColorScheme::Secondary,
    ColorScheme::Tertiary,
    ColorScheme::Error,
];

fn color_palette(color: &str, title: &str) -> Html {
    let percentage = [99, 95, 90, 80, 70, 60, 50, 40, 30, 20, 10, 5];
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
        .with_child(color_palette("primary", "Primary Color"))
        .with_child(color_palette("secondary", "Secondary Color"))
        .with_child(color_palette("tertiary", "Tertiary Color"))
        .with_child(color_palette("neutral", "Neutral Color"))
        .with_child(color_palette("neutral-alt", "Neutral Variant Color"))
        .with_child(color_palette("error", "Error Color"))
        .into()
}

fn box_panel(_: &TabPanelRenderInfo) -> Html {

    let mut panel = Column::new()
        .padding(2)
        .gap(4);

    for scheme in COLOR_SCHEMES {

        let mut buttons = Row::new()
            .padding(2)
            .gap(2);

        for scheme in COLOR_SCHEMES {
            buttons.add_child(
                Button::new(format!("{:?}", scheme))
                    .class(*scheme)
            )
        }

        panel.add_child(
            Panel::new()
                .class("pwt-size-fit-content")
                .class(*scheme)
                .title(format!("Color scheme '{:?}'", scheme))
                .with_child(buttons)
        );
    }

    panel.into()
}

fn fonts_panel(_: &TabPanelRenderInfo) -> Html {

    let fonts = ["Display", "Headline", "Title", "Body", "Label"];
    let sizes = ["large", "medium", "small"];

    let mut panel = Column::new()
        .padding(2)
        .gap(1);

    for font in fonts {
        for size in sizes {
            let class = format!("pwt-font-{}-{}", font.to_lowercase(), size);
            panel.add_child(html!{
                <span {class}>{format!("{} {}", font, size)}</span>
            });

        }
    }

    panel.into()
}

#[function_component]
pub fn ThemeDemo() -> Html {
    TabPanel::new()
        .class("pwt-fit")
        .title("Theme settings preview")
        .with_item_builder("fonts", "Fonts", None::<&str>, fonts_panel)
        .with_item_builder("colors", "Colors", None::<&str>, color_panel)
        .with_item_builder("boxes", "Boxes", None::<&str>, box_panel)
        .into()
}
