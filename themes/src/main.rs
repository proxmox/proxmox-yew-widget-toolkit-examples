use pwt::prelude::*;
use pwt::widget::{Column, Panel, ThemeLoader, ThemeModeSelector, ThemeNameSelector, Toolbar};

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let toolbar = Toolbar::new()
            .border_bottom(true)
            .with_flex_spacer()
            .with_child(ThemeNameSelector::new())
            .with_child(ThemeModeSelector::new());

        let content = Column::new()
            .padding(2)
            .with_child("Please press the theme selector button to change the theme.");

        let panel = Panel::new()
            .margin(4)
            .title("A simple Panel")
            .with_child(toolbar)
            .with_child(content);

        ThemeLoader::new(panel).into()
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    pwt::state::set_available_themes(&["Crisp", "Desktop", "Material"]);
    yew::Renderer::<App>::new().render();
}
