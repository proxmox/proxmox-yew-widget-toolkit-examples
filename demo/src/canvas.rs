use yew::prelude::*;

use pwt::prelude::*;

use pwt::widget::{Column, Panel};
use pwt::widget::canvas::{Canvas, Circle, SvgLength};


fn test_canvas() -> Canvas {
    Canvas::new()
        .class("pwt-w-100 pwt-p-0")
        .height(150.0)
        .with_child(
            Circle::new()
                .position(SvgLength::Fraction(0.5), SvgLength::Fraction(0.5))
                .radius(50)
                .stroke_width(15)
                .class("pwt-fill-primary")
                .class("pwt-stroke-secondary")
        )
        .with_child({
            let onclick = Callback::from(|_event: MouseEvent| {
                log::info!("CLICKED");
            });

            Circle::new()
                .position(40.0, 50.0)
                .radius(40.0)
                .onclick(onclick)
        })
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
