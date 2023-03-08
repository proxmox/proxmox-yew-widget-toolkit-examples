use yew::prelude::*;

use pwt::prelude::*;
use pwt::widget::{Button, Column, Meter, MiniScroll, Panel, Progress, Row, MiniScrollMode};

fn mini_scoll_demo1() -> Html {

    let mut content = Row::new()
        .padding(2)
        .gap(2);

    for i in 0..10 {
        content.add_child(Button::new(format!("Button {i}")));
    }


    Panel::new()
        .title("MiniScroll mode 'MiniScrollMode::Native' (default)")
        .with_child(MiniScroll::new(content))
        .into()
}

fn mini_scoll_demo2() -> Html {

    let mut content = Row::new()
        .padding(2)
        .gap(2);

    for i in 0..10 {
        content.add_child(Button::new(format!("Button {i}")));
    }


    Panel::new()
        .title("MiniScroll mode 'MiniScrollMode::Arrow'")
        .with_child(MiniScroll::new(content).scroll_mode(MiniScrollMode::Arrow))
        .into()
}

#[function_component]
pub fn MiscellaneousDemo() -> Html {

    let meter = Meter::new()
        .min(200.0)
        .max(500.0)
        .low(350.0)
        .high(450.0)
        .optimum(200.0);

    Column::new()
        .class("pwt-fit")
        .padding(2)
        .gap(2)
        .with_child("Progress indicator (ongoing progress)")
        .with_child(Progress::new())
        .with_child("Progress indicator (value)")
        .with_child(Progress::new().max(60.0).value(50.0))
        .with_child("Meter (min 200, max 500, low 350, height 450, optimum 200)")
        .with_child(meter.clone().value(220.0))
        .with_child(meter.clone().value(320.0))
        .with_child(meter.clone().value(420.0))
        .with_child(meter.clone().value(460.0))
        .with_child(mini_scoll_demo1())
        .with_child(mini_scoll_demo2())
        .into()
}
