use yew::prelude::*;

use pwt::prelude::*;
use pwt::state::NavigationContainer;
use pwt::widget::{
    Button, Column, Panel, Row, SelectionViewRenderInfo, TabBarItem, TabPanel, ThemeDensitySelector,
};

use pwt::css::{ColorScheme, Flex, Overflow};
static COLOR_SCHEMES: &[ColorScheme] = &[
    ColorScheme::Neutral,
    ColorScheme::Surface,
    ColorScheme::NeutralAlt,
    ColorScheme::Primary,
    ColorScheme::Secondary,
    ColorScheme::Tertiary,
    ColorScheme::Error,
    ColorScheme::Warning,
];

fn color_palette(color: &str, title: &str) -> Html {
    let percentage = [99, 95, 90, 80, 70, 60, 50, 40, 30, 20, 10, 5];
    let colors: Html = percentage
        .iter()
        .map(|p| {
            let class = format!(
                "pwt-bg-color-{}-{} pwt-color-{}-{} pwt-p-2",
                color,
                p,
                color,
                if *p <= 50 { 99 } else { 10 },
            );

            html! {
                <div {class}>{format!("{}-{}", color, p)}</div>
            }
        })
        .collect();

    let class = format!("pwt-bg-color-{} pwt-color-on-{} pwt-p-4", color, color,);

    let variant_class = format!(
        "pwt-bg-color-{}-container pwt-color-on-{}-container pwt-p-4",
        color, color,
    );

    html! {
        <div>
            <div {class}>{title}</div>
            <div class={variant_class}>{"Container variant"}</div>
            {colors}
        </div>
    }
}

fn color_panel(_: &SelectionViewRenderInfo) -> Html {
    Column::new()
        .class(Flex::Fill)
        .class(Overflow::Auto)
        .padding(2)
        .gap(4)
        .with_child(color_palette("primary", "Primary Color"))
        .with_child(color_palette("secondary", "Secondary Color"))
        .with_child(color_palette("tertiary", "Tertiary Color"))
        .with_child(color_palette("neutral", "Neutral Color"))
        .with_child(color_palette("neutral-alt", "Neutral Variant Color"))
        .with_child(color_palette("success", "Success Color"))
        .with_child(color_palette("error", "Error Color"))
        .with_child(color_palette("warning", "Warning Color"))
        .into()
}

fn box_panel(_: &SelectionViewRenderInfo) -> Html {
    let mut panel = Column::new()
        .class(Flex::Fill)
        .class(Overflow::Auto)
        .padding(2)
        .gap(4)
        .with_child(ThemeDensitySelector::new());

    for scheme in COLOR_SCHEMES {
        let mut buttons = Row::new().padding(2).gap(2);

        for scheme in COLOR_SCHEMES {
            buttons.add_child(Button::new(format!("{:?}", scheme)).class(*scheme))
        }

        panel.add_child(
            Panel::new()
                .border(true)
                .class("pwt-size-fit-content")
                .class(*scheme)
                .title(format!("Color scheme '{:?}'", scheme))
                .with_child(buttons),
        );
    }

    panel.into()
}

fn fonts_panel(_: &SelectionViewRenderInfo) -> Html {
    let fonts = ["Display", "Headline", "Title", "Body", "Label"];
    let sizes = ["Large", "Medium", "Small"];

    let mut panel = Column::new()
        .class(Flex::Fill)
        .class(Overflow::Auto)
        .padding(2)
        .gap(1);

    for font in fonts {
        for size in sizes {
            let class = format!("pwt-font-{}-{}", font.to_lowercase(), size.to_lowercase());
            panel.add_child(html! {
                <span {class}>{format!("{} {}", font, size)}</span>
            });
        }
    }

    panel.into()
}

fn text_panel(_: &SelectionViewRenderInfo) -> Html {
    let text = "The contrast of this colored text should be OK.";
    Column::new()
        .class(Flex::Fill)
        .class(Overflow::Auto)
        .padding(2)
        .gap(1)
        .with_child(html! {<span class="pwt-color-primary">{format!("{text} (Primary)")}</span>})
        .with_child(
            html! {<span class="pwt-color-secondary">{format!("{text} (Secondary)")}</span>},
        )
        .with_child(html! {<span class="pwt-color-tertiary">{format!("{text} (Tertiary)")}</span>})
        .with_child(html! {<span class="pwt-color-success">{format!("{text} (Success)")}</span>})
        .with_child(html! {<span class="pwt-color-warning">{format!("{text} (Warming)")}</span>})
        .with_child(html! {<span class="pwt-color-error">{format!("{text} (Error)")}</span>})
        .into()
}

#[function_component]
pub fn ThemeDemo() -> Html {
    let panel = TabPanel::new()
        .router(true)
        .class(Flex::Fill)
        .class(Overflow::Auto)
        .title("Theme settings preview")
        .with_item_builder(TabBarItem::new().key("fonts").label("Fonts"), fonts_panel)
        .with_item_builder(TabBarItem::new().key("colors").label("Colors"), color_panel)
        .with_item_builder(TabBarItem::new().key("boxes").label("Boxes"), box_panel)
        .with_item_builder(TabBarItem::new().key("text").label("Text"), text_panel);

    NavigationContainer::new().with_child(panel).into()
}
