use pwt::prelude::*;

use pwt::widget::align::{align_to, AlignOptions, GrowDirection, Point};
use pwt::widget::dom::IntoHtmlElement;
use pwt::widget::Panel;
use pwt::{
    props::{ContainerBuilder, EventSubscriber, WidgetBuilder},
    widget::{Button, Column, Container, Row},
};

#[function_component]
pub fn AlignDemo() -> Html {
    let button_ref = NodeRef::default();
    let text_ref = NodeRef::default();
    let button = Container::new()
        .class("pwt-bg-color-surface")
        .padding(2)
        .border(true)
        .with_child("Base")
        .node_ref(button_ref.clone());

    let top_onclick = {
        let button_ref = button_ref.clone();
        let text_ref = text_ref.clone();
        move |_| align(button_ref.clone(), text_ref.clone(), Point::Top)
    };
    let top_end_onclick = {
        let button_ref = button_ref.clone();
        let text_ref = text_ref.clone();
        move |_| align(button_ref.clone(), text_ref.clone(), Point::TopEnd)
    };
    let end_onclick = {
        let button_ref = button_ref.clone();
        let text_ref = text_ref.clone();
        move |_| align(button_ref.clone(), text_ref.clone(), Point::End)
    };
    let bottom_end_onclick = {
        let button_ref = button_ref.clone();
        let text_ref = text_ref.clone();
        move |_| align(button_ref.clone(), text_ref.clone(), Point::BottomEnd)
    };
    let bottom_onclick = {
        let button_ref = button_ref.clone();
        let text_ref = text_ref.clone();
        move |_| align(button_ref.clone(), text_ref.clone(), Point::Bottom)
    };
    let bottom_start_onclick = {
        let button_ref = button_ref.clone();
        let text_ref = text_ref.clone();
        move |_| align(button_ref.clone(), text_ref.clone(), Point::BottomStart)
    };
    let start_onclick = {
        let button_ref = button_ref.clone();
        let text_ref = text_ref.clone();
        move |_| align(button_ref.clone(), text_ref.clone(), Point::Start)
    };
    let top_start_onclick = {
        let button_ref = button_ref.clone();
        let text_ref = text_ref.clone();
        move |_| align(button_ref.clone(), text_ref.clone(), Point::TopStart)
    };
    let center_onclick = {
        let button_ref = button_ref.clone();
        let text_ref = text_ref.clone();
        move |_| align(button_ref.clone(), text_ref.clone(), Point::Center)
    };

    {
        let button_ref = button_ref.clone();
        let text_ref = text_ref.clone();
        gloo_timers::callback::Timeout::new(100, move || {
            align(button_ref.clone(), text_ref.clone(), Point::Center);
        })
        .forget();
    }

    let row = Row::new()
        .class("pwt-fit")
        .padding(2)
        .gap(2)
        .with_child(
            Column::new()
                .gap(2)
                .with_child(Button::new("Top").onclick(top_onclick))
                .with_child(Button::new("TopEnd").onclick(top_end_onclick))
                .with_child(Button::new("End").onclick(end_onclick))
                .with_child(Button::new("BottomEnd").onclick(bottom_end_onclick))
                .with_child(Button::new("Bottom").onclick(bottom_onclick))
                .with_child(Button::new("BottomStart").onclick(bottom_start_onclick))
                .with_child(Button::new("Start").onclick(start_onclick))
                .with_child(Button::new("TopStart").onclick(top_start_onclick))
                .with_child(Button::new("Center").onclick(center_onclick)),
        )
        .with_flex_spacer()
        .with_child(
            Column::new()
                .with_flex_spacer()
                .with_child(button)
                .with_flex_spacer(),
        )
        .with_flex_spacer()
        .with_child(
            Panel::new()
                .title("Floating")
                .with_child(html!({ "Content" }))
                .node_ref(text_ref.clone())
                .attribute("style", "width: 100px; height: 100px"),
        );
    row.into()
}

fn align(base: NodeRef, target: NodeRef, side: Point) {
    let target_point = match side {
        Point::TopStart => Point::BottomEnd,
        Point::Top => Point::Bottom,
        Point::TopEnd => Point::BottomStart,
        Point::Start => Point::End,
        Point::Center => Point::Center,
        Point::End => Point::Start,
        Point::BottomStart => Point::TopEnd,
        Point::BottomEnd => Point::TopStart,
        Point::Bottom => Point::Top,
    };
    let options = AlignOptions::new(side, target_point, GrowDirection::None);
    if let Err(err) = align_to(base, target.clone(), Some(options)) {
        log::error!("could not align: {}", err.to_string());
    }
    if let Some(el) = target.into_html_element() {
        let style = el.style();
        gloo_timers::callback::Timeout::new(250, move || {
            let _ = style.set_property("transition", "250ms");
        })
        .forget();
    }
}
