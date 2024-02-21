use pwt::touch::NavigationBar;
use pwt::widget::TabBarItem;

pub fn navigation_bar() -> NavigationBar {
    let items = vec![
        TabBarItem::new()
            .key("")
            .icon_class("fa fa-home")
            .label("Home"),
        TabBarItem::new()
            .key("settings")
            .icon_class("fa fa-gear")
            .label("Settings"),
        TabBarItem::new()
            .key("favorite")
            .icon_class("fa fa-star")
            .label("Favorite"),
        TabBarItem::new()
            .key("profile")
            .icon_class("fa fa-user-o")
            .active_icon_class("fa fa-user")
            .label("Profile"),
    ];

    NavigationBar::new(items).router(true)
}
