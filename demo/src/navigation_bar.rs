use pwt::prelude::*;
use pwt::touch::NavigationBar;
use pwt::widget::{Column, TabBarItem};
use pwt::state::NavigationContainer;

#[function_component]
pub fn NavigationBarDemo() -> Html {
    let rail = NavigationBar::new(vec![
        TabBarItem::new()
            .key("mail")
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
    .router(true);

    NavigationContainer::new()
        .with_child(
            Column::new()
                .class("pwt-fit")
                .with_flex_spacer()
                .with_child(rail),
        )
        .into()
}
