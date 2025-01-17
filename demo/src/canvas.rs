use yew::prelude::*;

use pwt::prelude::*;

use pwt::widget::canvas::{
    Animate, AnimateTransform, Canvas, Circle, Ellipse, Group, Hyperlink, Line, Path, Polygon,
    Polyline, Rect, Reference, SvgLength, TSpan, Text,
};
use pwt::widget::{Column, Panel};

fn canvas_animation() -> Canvas {
    Canvas::new()
        .padding(2)
        .with_child(
            Polygon::new()
                .fill("green")
                .points(&[(60., 30.), (90., 90.), (30., 90.)])
                .animate(
                    AnimateTransform::rotate()
                        .attribute("from", "0 60 70")
                        .attribute("to", "360 60 70")
                        .attribute("dur", "5s")
                        .repeat_count(f32::INFINITY),
                )
                .animate(
                    AnimateTransform::scale()
                        .attribute("values", "1 1; 2 1; 1 1;")
                        .attribute("dur", "5s")
                        .additive(true)
                        .repeat_count(f32::INFINITY),
                ),
        )
        .with_child(
            Circle::new().position(200, 50).r(10).animate(
                Animate::new("r")
                    .attribute("values", "10; 50; 10;")
                    .attribute("dur", "2s")
                    .repeat_count(f32::INFINITY),
            ),
        )
}

fn canvas_reference() -> Canvas {
    Canvas::new()
        .padding(2)
        .with_child(Polygon::new().id("star").stroke("black").points(&[
            (50., 0.),
            (21., 90.),
            (98., 35.),
            (2., 35.),
            (79., 90.),
        ]))
        .with_child(Reference::new("#star").x(120).class("pwt-fill-primary"))
        .with_child(
            Reference::new("#star")
                .attribute("transform", "translate(240 0) rotate(45 50 50)")
                .fill("green"),
        )
}

fn canvas_text() -> Canvas {
    Canvas::new().padding(2).with_child(
        Text::new("You are ")
            .position(0, SvgLength::Em(1.0))
            .class("pwt-font-title-large")
            .with_child(TSpan::new("not").class("pwt-fill-error"))
            .with_child(" a banana!")
            .with_child(
                TSpan::new("This is the second line.")
                    .x(0)
                    .dy(SvgLength::Em(1.0)),
            )
            .with_child(
                TSpan::new("A link to ")
                    .x(0)
                    .dy(SvgLength::Em(1.0))
                    .with_child(
                        Hyperlink::new()
                            .href("https://www.proxmox.com")
                            .with_child(TSpan::new("www.proxmox.com")),
                    ),
            ),
    )
}

fn simple_canvas() -> Canvas {
    Canvas::new()
        .with_child(
            Rect::new()
                .position(10, 10)
                .class("pwt-fill-primary")
                .width(80)
                .height(80)
                .stroke_width(5)
                .stroke("currentColor")
                .attribute("stroke-dasharray", "20 10")
                .rx(20),
        )
        .with_child(
            Circle::new()
                .position(150, 50)
                .r(40)
                .stroke_width(5)
                .class("pwt-fill-primary")
                .class("pwt-stroke-secondary"),
        )
        .with_child(
            Polygon::new()
                .stroke("black")
                .class("pwt-fill-tertiary")
                .points(&[(50., 0.), (21., 90.), (98., 35.), (2., 35.), (79., 90.)])
                .attribute("transform", "translate(200,5)"),
        )
        .with_child(
            Polyline::new()
                .stroke("black")
                .fill("none")
                .points(&[(50., 0.), (21., 90.), (98., 35.), (2., 35.), (79., 90.)])
                .attribute("transform", "translate(300,5)"),
        )
        .with_child(
            Path::new()
                .fill("red")
                .d(concat!(
                    "M 10,30 ",
                    "A 20,20 0,0,1 50,30 ",
                    "A 20,20 0,0,1 90,30 ",
                    "Q 90,60 50,90 ",
                    "Q 10,60 10,30 z",
                ))
                .attribute("transform", "translate(400,5)"),
        )
        .with_child(
            Group::new()
                .stroke("grey")
                .stroke_width(10)
                .attribute("transform", "translate(500,5)")
                .attribute("stroke-linecap", "round")
                .with_child(Line::new(10.0, 10.0, 90.0, 90.0))
                .with_child(Line::new(90.0, 10.0, 10.0, 90.0)),
        )
        .with_child(
            Ellipse::new()
                .position(650, 50)
                .rx(40)
                .ry(30)
                .class("pwt-fill-primary"),
        )
}

#[function_component]
pub fn CanvasDemo() -> Html {
    Column::new()
        .class(pwt::css::FlexFit)
        .class("pwt-content-spacer")
        .with_child(
            Panel::new()
                .border(true)
                .title("Simple canvas shapes")
                .with_child(simple_canvas()),
        )
        .with_child(
            Panel::new()
                .border(true)
                .title("Canvas Text")
                .with_child(canvas_text()),
        )
        .with_child(
            Panel::new()
                .border(true)
                .title("Use object references.")
                .with_child(canvas_reference()),
        )
        .with_child(
            Panel::new()
                .border(true)
                .title("SVG animation")
                .with_child(canvas_animation()),
        )
        .into()
}
