use pwt::css::{JustifyContent, Flex};
use pwt::prelude::*;
use pwt::touch::{Fab, NavigationRail};
use pwt::widget::{Row, TabBarItem};
use pwt::state::NavigationContainer;

#[function_component]
pub fn NavigationRailDemo() -> Html {
    let rail = NavigationRail::new(vec![
        TabBarItem::new()
            .key("delete")
            .icon_class("fa fa-envelope-o")
            .active_icon_class("fa fa-envelope")
            .label("Mail"),
        TabBarItem::new()
            .key("settings")
            .icon_class("fa fa-gear")
            .label("Settings"),
        TabBarItem::new()
            .key("users")
            .icon_class("fa fa-user-o")
            .active_icon_class("fa fa-user")
            .label("Users"),
    ])
    .router(true)
    .leading(Fab::new("fa fa-pencil"))
    .group_alignment(JustifyContent::Center);

    NavigationContainer::new()
        .with_child(
            Row::new()
                .class(Flex::Fill)
                .border_start(true)
                .with_child(rail)
            )
        .into()
}
