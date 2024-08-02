use pwt::prelude::*;
use pwt::state::NavigationContainer;

use pwt::touch::{NavigationBar, PageView};
use pwt::widget::{Button, Column, Row};
use pwt::widget::{Container, TabBarItem};
use yew::virtual_dom::Key;
use yew_router::HashRouter;

pub enum Msg {
    Page0,
    Page1,
    PageChange(usize),
    NavigationBarSelect(Option<Key>),
}

pub struct App {
    page: usize,
    active_item: Option<Key>,
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
            .padding(2)
            .margin_bottom(2)
            .with_child(
                Container::new()
                    .tag("span")
                    .padding_start(1)
                    .class("pwt-font-headline-small")
                    .with_child(text),
            )
        //.with_flex_spacer()
        //.with_child(button_group)
    }

    fn page0(&self, ctx: &Context<Self>) -> Html {
        let content = Button::new("VIEW PAGE1").onclick(ctx.link().callback(|_| Msg::Page1));

        Column::new()
            .with_child(self.banner("Page0"))
            .with_child(
                Container::new()
                    .tag("p")
                    .padding(2)
                    .with_child("You can also SWIPE LEFT to view Page 1."),
            )
            .with_child(content)
            .into()
    }

    fn page1(&self, ctx: &Context<Self>) -> Html {
        let content = Button::new("VIEW PAGE0").onclick(ctx.link().callback(|_| Msg::Page0));

        Column::new()
            .with_child(self.banner("Page1"))
            .with_child(
                Container::new()
                    .tag("p")
                    .padding(2)
                    .with_child("You can also SWIPE LEFT to view Page 0."),
            )
            .with_child(content)
            .into()
    }
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            page: 0,
            active_item: None,
        }
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
            Msg::NavigationBarSelect(active_item) => {
                self.active_item = active_item;
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

        let items = vec![
            TabBarItem::new()
                .key("delete")
                .icon_class("fa fa-trash-o")
                .active_icon_class("fa fa-trash")
                .label("Delete"),
            TabBarItem::new()
                .key("settings")
                .icon_class("fa fa-gear")
                .label("Settings"),
            TabBarItem::new()
                .key("users")
                .icon_class("fa fa-user-o")
                .active_icon_class("fa fa-user")
                .label("Users"),
        ];

        let navigation_bar = NavigationBar::new(items)
            .router(true)
            .on_select(ctx.link().callback(Msg::NavigationBarSelect));

        NavigationContainer::new()
            .with_child(
                Column::new()
                    .class("pwt-viewport")
                    .with_child(html! {
                        <div class="pwt-flex-fill">{page_view}</div>
                    })
                    .with_child(navigation_bar),
            )
            .into()
    }
}

#[function_component]
fn RoutedApp() -> Html {
    html! { <HashRouter><App/></HashRouter> }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<RoutedApp>::new().render();
}
