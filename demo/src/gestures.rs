use std::collections::VecDeque;

use pwt::prelude::*;
use pwt::widget::{Column, GestureDetector, Panel};

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

        Column::new()
            .class("pwt-fit")
            .with_child(
                Panel::new()
                    .class("pwt-flex-fill")
                    .title("Gesture Detector")
                    .with_child(
                        GestureDetector::new()
                            .on_tap(ctx.link().callback(|_| Msg::Log("TAP".into())))
                            .on_long_tap(ctx.link().callback(|_| Msg::Log("LONG TAP".into()))),
                    ),
            )
            .with_child(log_panel)
            .into()
    }
}
