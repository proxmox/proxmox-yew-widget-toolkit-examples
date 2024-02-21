use pwt::css::Flex;
use pwt::prelude::*;
use pwt::touch::ApplicationBar;
use pwt::widget::{ActionIcon, Column};

#[function_component]
pub fn ApplicationBarDemo() -> Html {
    let app_bar = ApplicationBar::new()
        .leading(ActionIcon::new("fa fa-lg fa-arrow-left"))
        .title("The Title")
        .with_action(ActionIcon::new("fa fa-lg fa-bars"));

    Column::new()
        .class(Flex::Fill)
        .with_child(app_bar)
        .with_flex_spacer()
        .into()
}
