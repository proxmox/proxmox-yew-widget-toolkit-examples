use pwt::prelude::*;

use pwt::dom::align::{align_to, align_to_xy, AlignOptions, GrowDirection, Point};
use pwt::dom::IntoHtmlElement;
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

    let button_points = vec![
        ("Top", Point::Top),
        ("TopEnd", Point::TopEnd),
        ("End", Point::End),
        ("BottomEnd", Point::BottomEnd),
        ("Bottom", Point::Bottom),
        ("BottomStart", Point::BottomStart),
        ("Start", Point::Start),
        ("TopStart", Point::TopStart),
        ("Center", Point::Center),
    ];

    let mut column = Column::new().gap(2);

    for (text, point) in button_points {
        let handler = {
            let button_ref = button_ref.clone();
            let text_ref = text_ref.clone();
            move |_| align(button_ref.clone(), text_ref.clone(), point)
        };

        column.add_child(Button::new(text).onclick(handler));
    }

    {
        let button_ref = button_ref.clone();
        let text_ref = text_ref.clone();
        gloo_timers::callback::Timeout::new(100, move || {
            align(button_ref.clone(), text_ref.clone(), Point::Center);
        })
        .forget();
    }

    {
        let text_ref = text_ref.clone();
        column.add_child(Button::new("To 0,0").onclick(move |_| {
            let _ = align_to_xy(text_ref.clone(), (0.0, 0.0), Point::TopStart);
        }));
    }
    {
        let text_ref = text_ref.clone();
        column.add_child(Button::new("To 500,500").onclick(move |_| {
            let _ = align_to_xy(text_ref.clone(), (500.0, 500.0), Point::TopStart);
        }));
    }
    {
        let text_ref = text_ref.clone();
        column.add_child(Button::new("To -20,-20").onclick(move |_| {
            let _ = align_to_xy(text_ref.clone(), (-20.0, -20.0), Point::TopStart);
        }));
    }

    Row::new()
        .class(pwt::css::FlexFit)
        .class("pwt-content-spacer-padding")
        .gap(2)
        .with_child(column)
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
                .border(true)
                .title("Floating")
                .style("position", "fixed")
                .with_child(Container::new().padding(2).with_child("Content"))
                .node_ref(text_ref.clone()),
        )
        .into()
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
