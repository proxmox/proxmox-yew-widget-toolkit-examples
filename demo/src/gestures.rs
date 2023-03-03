use std::collections::VecDeque;

use pwt::prelude::*;
use pwt::touch::{
    GestureDetector, GestureDragEvent, GestureSwipeEvent, Slidable, SlidableAction,
    SlidableActionMouseEvent,
};
use pwt::widget::{Button, Column, Container, Panel, Row};

#[derive(Properties, Clone, PartialEq)]
pub struct GestureDetectorDemoProps {}

pub enum Msg {
    Log(String),
}

pub struct GestureDetectorDemo {
    logs: VecDeque<String>,
}

impl Component for GestureDetectorDemo {
    type Message = Msg;
    type Properties = GestureDetectorDemoProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let mut logs = VecDeque::new();
        logs.push_back("This is initial log message.".into());
        Self { logs }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Log(msg) => {
                if self.logs.len() > 5 {
                    self.logs.pop_front();
                }
                self.logs.push_back(msg);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let log_panel = Panel::new().title("Gesture detection log").with_child({
            self.logs
                .iter()
                .map(|t| html! {<div>{t}</div>})
                .collect::<Html>()
        });

        let content = Container::new()
            .class("pwt-fit")
            .attribute("style", "touch-action:none;")
            .with_child(
                Button::new("Button inside GestureDetector")
                    .onclick(ctx.link().callback(|_| Msg::Log("BUTTON CLICK".into()))),
            );

        let detector = GestureDetector::new(content)
            .on_drag_start(ctx.link().callback(|_| Msg::Log(format!("DRAG START"))))
            .on_drag_end(ctx.link().callback(|_| Msg::Log(format!("DRAG END"))))
            .on_drag_update(ctx.link().callback(|event: GestureDragEvent| {
                Msg::Log(format!("DRAG UPDATE {} {}", event.x(), event.y(),))
            }))
            .on_swipe(ctx.link().callback(|event: GestureSwipeEvent| {
                Msg::Log(format!("SWIPE direction {}", event.direction,))
            }))
            .on_tap(ctx.link().callback(|_| Msg::Log("TAP".into())))
            .on_long_press(ctx.link().callback(|_| Msg::Log("LONG PRESS".into())));

        let slidable1 = Slidable::new(
            Container::new()
                .padding(4)
                .with_child("Slide to left or right to dismiss"),
        )
        .class("pwt-w-100")
        .on_dismiss(|_| { /* just enable dismiss */ });

        let slidable2 = Slidable::new(
            Container::new()
                .padding(4)
                .with_child("Slide to reveal buttons"),
        )
        .left_actions(
            SlidableAction::new("Bookmark")
                .icon_class("fa fa-star-o")
                .on_click(|event: SlidableActionMouseEvent| {
                    event.keep_open(true);
                }),
        )
        .right_actions(
            SlidableAction::new("Delete")
                .icon_class("fa fa-trash")
                .on_click(|event: SlidableActionMouseEvent| {
                    event.dismiss(true);
                }),
        )
        .class("pwt-w-100");

        Column::new()
            .class("pwt-fit")
            .with_child(Row::new().with_child(slidable1))
            .with_child(Row::new().with_child(slidable2))
            .with_child(
                Panel::new()
                    .class("pwt-flex-fill")
                    .title("Gesture Detector")
                    .with_child(detector),
            )
            .with_child(log_panel)
            .into()
    }
}
