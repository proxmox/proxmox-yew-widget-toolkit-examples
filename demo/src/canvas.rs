use yew::prelude::*;

use pwt::prelude::*;

use pwt::widget::{Column, Panel};
use pwt::widget::canvas::{
    Canvas, Circle, Group, Line, Path, Polyline, Polygon, Rect, Text, TSpan,
    SvgLength,
};


fn test_canvas() -> Canvas {
    Canvas::new()
        .class("pwt-w-100 pwt-p-0")
        .height(150.0)
        .with_child(
            Rect::new()
                .position(10, 10)
                .class("pwt-fill-primary")
                .width(80)
                .height(80)
                .stroke_width(5)
                .stroke("currentColor")
                .attribute("stroke-dasharray", "20 10")
                .rx(20)
        )
        .with_child(
            Circle::new()
                .position(150, 50)
                .radius(40)
                .stroke_width(5)
                .class("pwt-fill-primary")
                .class("pwt-stroke-secondary")
        )
        .with_child(
            Polygon::new()
                .stroke("black")
                .class("pwt-fill-tertiary")
                .points(&[(50.,0.), (21.,90.), (98.,35.),  (2.,35.), (79.,90.)])
                .attribute("transform", "translate(200,5)")
        )
        .with_child(
            Polyline::new()
                .stroke("black")
                .fill("none")
                .points(&[(50.,0.), (21.,90.), (98.,35.),  (2.,35.), (79.,90.)])
                .attribute("transform", "translate(300,5)")
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
                .attribute("transform", "translate(400,5)")
        )
        .with_child(
            Group::new()
                .stroke("grey")
                .stroke_width(10)
                .attribute("transform", "translate(500,5)")
                .attribute("stroke-linecap", "round")
                .with_child(Line::new(10.0, 10.0, 90.0, 90.0))
                .with_child(Line::new(90.0, 10.0, 10.0, 90.0))
        )
        .with_child(
            Text::new("You are ")
                .position(10, 120)
                .class("pwt-font-title-large")
                .with_child(TSpan::new("not").class("pwt-fill-error"))
                .with_child(" a banana!")
                .with_child(TSpan::new("This is the second line.").x(10).dy(SvgLength::Em(1.0)))
        )
}

#[function_component]
pub fn CanvasDemo() -> Html {

    Column::new()
        .class("pwt-fit")
        .padding(2)
        .gap(2)
        .with_child(
            Panel::new()
                .title("Simple Canvas")
                .with_child(test_canvas())
        )
        .into()
}
