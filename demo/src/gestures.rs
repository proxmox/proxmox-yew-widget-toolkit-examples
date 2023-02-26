use std::collections::VecDeque;

use pwt::prelude::*;
use pwt::widget::{Button, Column, GestureDetector, GestureDragEvent, GestureSwipeEvent, Panel};

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

        let content = Button::new("Button inside GestureDetector")
            .onclick(ctx.link().callback(|_| Msg::Log("BUTTON CLICK".into())));

        let detector = GestureDetector::new(content)
            .on_drag_start(ctx.link().callback(|_| Msg::Log(format!("DRAG START"))))
            .on_drag_end(ctx.link().callback(|_| Msg::Log(format!("DRAG END"))))
            .on_drag_update(ctx.link().callback(|event: GestureDragEvent| {
                Msg::Log(format!(
                    "DRAG UPDATE {} {}",
                    event.offset_x(),
                    event.offset_y(),
                ))
            }))
            .on_swipe(ctx.link().callback(|event: GestureSwipeEvent| {
                Msg::Log(format!(
                    "SWIPE direction {}",
                    event.direction,
                ))
            }))
            .on_tap(ctx.link().callback(|_| Msg::Log("TAP".into())))
            .on_long_press(ctx.link().callback(|_| Msg::Log("LONG PRESS".into())));

        Column::new()
            .class("pwt-fit")
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
