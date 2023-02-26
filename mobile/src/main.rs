use pwt::prelude::*;
use yew::prelude::*;

use pwt::widget::{Button, Column, PageView, Panel, Toolbar};

pub enum Msg {
    Page0,
    Page1,
    PageChange(usize),
}

pub struct App {
    page: usize,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { page: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Page0 => {
                self.page = 0;
            }
            Msg::Page1 => {
                self.page = 1;
            }
            Msg::PageChange(page) => {
                self.page = page;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page0 = Button::new("VIEW PAGE1").onclick(ctx.link().callback(|_| Msg::Page1));
        let page1 = Button::new("VIEW PAGE0").onclick(ctx.link().callback(|_| Msg::Page0));

        PageView::new()
            .view_page(self.page)
            .on_page_change(ctx.link().callback(Msg::PageChange))
            .with_child(page0).with_child(page1).into()
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
