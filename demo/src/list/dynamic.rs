use pwt::prelude::*;

use pwt::widget::form::Checkbox;
use pwt::widget::{Button, Column, Container, List, ListTile, Toolbar};

use gloo_timers::callback::Interval;

pub struct DynamicListDemo {
    timer: Option<Interval>,
    time_s: f64,
    phase: f64,
    frequency: f64,

    random_seed: u32,
}

pub enum Msg {
    Timer,
    Dynamic(bool),
    IncreasePhase,
    Frequency(f64),
    Randomize,
}

impl DynamicListDemo {
    fn toolbar(&self, ctx: &Context<Self>) -> Toolbar {
        let mut toolbar = Toolbar::new()
            .with_child(
                Checkbox::new()
                    .box_label("Dynamic")
                    .checked(self.timer.is_some())
                    .on_change(ctx.link().callback(Msg::Dynamic)),
            )
            .border_bottom(true);

        if self.timer.is_some() {
            toolbar.add_child(
                Button::new("Increase Phase").onclick(ctx.link().callback(|_| Msg::IncreasePhase)),
            );
            let frequency = self.frequency;
            toolbar.add_child(
                Button::new("Faster").onclick(
                    ctx.link()
                        .callback(move |_| Msg::Frequency(frequency * 1.1)),
                ),
            );
            let frequency = self.frequency;
            toolbar.add_child(
                Button::new("Slower").onclick(
                    ctx.link()
                        .callback(move |_| Msg::Frequency((frequency / 1.1).max(0.0))),
                ),
            );
        } else {
            toolbar.add_child(
                Button::new("Randomize").onclick(ctx.link().callback(|_| Msg::Randomize)),
            );
        }

        toolbar
    }
}

impl Component for DynamicListDemo {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::Dynamic(true));
        Self {
            timer: None,
            time_s: 0f64,
            phase: 0f64,
            frequency: 0.5,

            random_seed: 1,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        match msg {
            Msg::Randomize => {
                self.random_seed += 100;
                true
            }
            Msg::Frequency(frequency) => {
                self.frequency = frequency;
                true
            }
            Msg::IncreasePhase => {
                self.phase += 0.1;
                true
            }
            Msg::Timer => {
                self.time_s += 0.100;
                true
            }
            Msg::Dynamic(dynamic) => {
                if dynamic {
                    self.timer = Some(Interval::new(1_00, move || link.send_message(Msg::Timer)));
                } else {
                    self.timer = None;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let time_s = self.time_s;
        let phase = self.phase;
        let frequency = self.frequency;

        let dynamic = self.timer.is_some();
        let random_seed = self.random_seed;

        let content = List::new(10000, move |pos: u64| {
            let height = if dynamic {
                75.0 + (time_s * 6.28 * frequency + (pos as f64) * phase).sin() * 25.0
            } else {
                // pseudo random, which produces always the same value for same position.
                // Note: this is not random at all
                let random = ((random_seed as u64 + (pos * 378632)) * 48271 % 0x7fffffff) as u32;
                50.0 + (random % 250) as f64
            };

            ListTile::new()
                .interactive(false)
                .height(height)
                .border(true)
                .with_child(html! {<div>{format!("Item {pos}")}</div>})
        })
        .min_row_height(40)
        // prefetching rows with dynamic size just disturbs scrolling, so disbale it
        .prefetch_count(0)
        .class("pwt-flex-fit");

        Column::new()
            .class(pwt::css::FlexFit)
            .with_child(
                Container::new()
                    .padding(2)
                    .border_bottom(true)
                    .with_child(html! {<h5>{"Virtual scrolling with dynamic sized items"}</h5>})
                    .with_child(
                        html! {"Virtual scrolling can handle dynamic element size changes."},
                    ),
            )
            .with_child(self.toolbar(ctx))
            .with_child(content)
            .into()
    }
}
