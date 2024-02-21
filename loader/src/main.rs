use gloo_console as console;
use pwt::prelude::*;
use yew::prelude::*;

use serde_json::Value;

use pwt::state::Loader;
use pwt::widget::{Column, Panel};

pub enum Msg {
    DataChange,
}

pub struct App {
    loader: Loader<Value>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let loader: Loader<Value> = Loader::new()
            .on_change(ctx.link().callback(|_| Msg::DataChange))
            .loader("/data/test_data.js");

        loader.load();

        Self { loader }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DataChange => {
                console::log!("got DataChange message");
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Use loader.render to correcty show load mask and load errors
        let content = self.loader.render(|data| {
            Column::new()
                .padding(2)
                .with_child("Load was sucessfull. Got the following data: ")
                .with_child(serde_json::to_string_pretty(data.as_ref()).unwrap())
        });

        Panel::new()
            .margin(4)
            .title("This panel loads data form the server.")
            .with_tool(self.loader.reload_button()) // add a reload button
            .with_child(content) // display data
            .into()
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
