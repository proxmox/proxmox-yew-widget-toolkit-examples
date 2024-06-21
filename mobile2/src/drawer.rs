use yew_router::prelude::Navigator;

use pwt::prelude::*;
use pwt::touch::PageController;
use pwt::widget::nav::{Menu, MenuItem, NavigationDrawer};
use pwt::widget::{Container, Row, ThemeModeSelector};

use super::Route;

pub fn create_drawer(page_controller: PageController, navigator: Navigator) -> NavigationDrawer {
    let menu = Menu::new()
        .with_item(
            MenuItem::new("Home")
                .key("id1")
                .icon_class("fa fa-paper-plane")
                .on_activate({
                    let navigator = navigator.clone();
                    move |_| navigator.push(&Route::Home)
                }),
        )
        .with_item(
            MenuItem::new("Profile")
                .key("id2")
                .icon_class("fa fa-paper-plane")
                .on_activate({
                    let navigator = navigator.clone();
                    move |_| {
                        navigator.push(&Route::Profile);
                    }
                }),
        )
        .with_item(
            MenuItem::new("Close Drawer")
                .key("id3")
                .icon_class("fa fa-paper-plane")
                .on_activate({
                    let page_controller = page_controller.clone();
                    move |_| {
                        page_controller.close_side_dialog();
                    }
                }),
        );

    let header = Row::new()
        .class("pwt-align-items-center")
        .with_child(
            Container::new()
                .padding(2)
                .class("pwt-font-size-title-large")
                .with_child("Menu Header"),
        )
        .with_child(ThemeModeSelector::new());

    NavigationDrawer::new(menu)
        .class("pwt-flex-fill")
        .header(header)
}
