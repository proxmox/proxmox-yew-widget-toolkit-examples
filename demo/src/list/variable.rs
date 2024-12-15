use std::cell::RefCell;
use std::rc::Rc;

use pwt::prelude::*;

use pwt::widget::{Button, Column, Container, List, ListTile, Toolbar};

pub struct VariableListDemo {
    random_seed: u32,
    heights: Rc<RefCell<Vec<(u32, bool)>>>,
}

pub enum Msg {
    Randomize,
    Toggle(usize),
}

fn get_random_height(random_seed: u32) -> Vec<(u32, bool)> {
    let mut heights = Vec::new();

    let mut random = random_seed;

    for _ in 0..1000 {
        random = (((random as u64) * 48271) % 0x7fffffff) as u32;
        let height = 50 + (random % 250);
        heights.push((height, false));
    }

    heights
}

impl VariableListDemo {
    fn toolbar(&self, ctx: &Context<Self>) -> Toolbar {
        Toolbar::new()
            .border_bottom(true)
            .with_child(Button::new("Randomize").onclick(ctx.link().callback(|_| Msg::Randomize)))
    }
}

impl Component for VariableListDemo {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let random_seed = 1;
        let heights = Rc::new(RefCell::new(get_random_height(random_seed)));

        Self {
            random_seed,
            heights,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle(pos) => {
                let mut heights = self.heights.borrow_mut();
                let record = heights.get_mut(pos).unwrap();
                *record = (record.0, !record.1);
                log::info!("TOGGLE {} {}", pos, record.1);
                true
            }
            Msg::Randomize => {
                self.random_seed += 100;
                self.heights = Rc::new(RefCell::new(get_random_height(self.random_seed)));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let heights = Rc::clone(&self.heights);
        let item_count = heights.borrow().len() as u64;
        let link = ctx.link().clone();

        let content = List::new(item_count, move |pos: u64| {
            let (height, collapsed) = heights.borrow()[pos as usize];

            let height = if collapsed { 40.0 } else { height as f64 };

            ListTile::new()
                .height(height)
                .with_child(html! {<div>{format!("Item {pos}")}</div>})
                .onclick(link.callback(move |_| Msg::Toggle(pos as usize)))
        })
        .min_row_height(40)
        .separator(true)
        // reset list if we re-initialize all tile heights
        .key(format!("test-list-{}", self.random_seed))
        .class("pwt-flex-fit");

        Column::new()
            .class(pwt::css::FlexFit)
            .with_child(
                Container::new()
                    .padding(2)
                    .border_bottom(true)
                    .with_child(html! {<h5>{"Virtual scrolling with dynamic sized items"}</h5>})
                    .with_child(
                        html! {"Virtual scrolling can handle dynamic element size changes. You can CLICK on the items to collapse/expand them."},
                    ),
            )
            .with_child(self.toolbar(ctx))
            .with_child(content)
            .into()
    }
}
