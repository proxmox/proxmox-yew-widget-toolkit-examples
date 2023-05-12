use pwt::prelude::*;
use pwt::touch::NavigationRail;
use pwt::widget::{Row, TabBarItem};
#[function_component]
pub fn NavigationRailDemo() -> Html {
    let rail = NavigationRail::new(vec![
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
    ]);

    Row::new().class("pwt-fit").with_child(rail).into()
}
