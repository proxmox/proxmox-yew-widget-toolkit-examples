use yew::prelude::*;

use pwt::prelude::*;

use pwt::widget::{Column, Panel};
use pwt::widget::canvas::{Canvas, Circle, Polygon, Rect, SvgLength};


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
        /*
        .with_child({
            let onclick = Callback::from(|_event: MouseEvent| {
                log::info!("CLICKED");
            });

            Circle::new()
                .position(40.0, 50.0)
                .radius(40.0)
                .onclick(onclick)
        })
     */
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
