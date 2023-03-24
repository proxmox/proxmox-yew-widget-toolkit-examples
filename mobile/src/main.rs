use pwt::prelude::*;
use yew::prelude::*;

use pwt::touch::PageView;
use pwt::widget::{Button, Column, Panel, Row, Toolbar};

pub enum Msg {
    Page0,
    Page1,
    PageChange(usize),
}

pub struct App {
    page: usize,
}

impl App {

    fn banner(&self, text: &str) -> Row {
        Row::new()
            .attribute("role", "banner")
            .attribute("aria-label", "PWT Mobile Demo")
            .class("pwt-navbar")
            .class("pwt-justify-content-space-between pwt-align-items-center")
            .class("pwt-border-bottom")
            .class("pwt-shadow1")
            .padding(4)
            .margin_bottom(2)
            .with_child(html!{
                <span class="pwt-ps-1 pwt-font-headline-small">{text}</span>
            })
            //.with_flex_spacer()
            //.with_child(button_group)
    }

    fn page0(&self, ctx: &Context<Self>) -> Html {
        let content = Button::new("VIEW PAGE1").onclick(ctx.link().callback(|_| Msg::Page1));

        Column::new()
            .with_child(self.banner("Page0"))
            .with_child(html!{<p class="pwt-p-2">{"You can also SWIPE LEFT to view Page 1."}</p>})
            .with_child(content)
            .into()
    }

    fn page1(&self, ctx: &Context<Self>) -> Html {
        let content = Button::new("VIEW PAGE0").onclick(ctx.link().callback(|_| Msg::Page0));

        Column::new()
            .with_child(self.banner("Page1"))
            .with_child(html!{<p class="pwt-p-2">{"You can also SWIPE RIGHT to view Page 0."}</p>})
            .with_child(content)
            .into()
    }

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
        let page0 = self.page0(ctx);
        let page1 = self.page1(ctx);

        let page_view = PageView::new()
            .view_page(self.page)
            .on_page_change(ctx.link().callback(Msg::PageChange))
            .with_child(page0)
            .with_child(page1);

        Column::new()
            .class("pwt-viewport")
            .with_child(html!{
                <div class="pwt-flex-fill">{page_view}</div>
            })
            .into()

    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
