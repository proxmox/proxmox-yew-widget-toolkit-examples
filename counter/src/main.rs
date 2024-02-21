use pwt::prelude::*;

use pwt::widget::{Button, Column, Panel, Toolbar};

pub enum Msg {
    Increment,
    Decrement,
    Reset,
}

pub struct App {
    value: i64, // counter value
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                log::info!("plus one");
            }
            Msg::Decrement => {
                self.value -= 1;
                log::info!("minus one");
            }
            Msg::Reset => {
                self.value = 0;
                log::info!("reset to zero");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let toolbar = Toolbar::new()
            .border_bottom(true)
            .with_child(Button::new("Increment").onclick(ctx.link().callback(|_| Msg::Increment)))
            .with_child(Button::new("Decrement").onclick(ctx.link().callback(|_| Msg::Decrement)))
            .with_flex_spacer()
            .with_child(Button::new("Reset").onclick(ctx.link().callback(|_| Msg::Reset)));

        let content = Column::new()
            .padding(2)
            .with_child("The current counter value is: ")
            .with_child(self.value);

        Panel::new()
            .margin(4)
            .title("A simple Counter inside a Panel")
            .with_child(toolbar)
            .with_child(content)
            .into()
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
