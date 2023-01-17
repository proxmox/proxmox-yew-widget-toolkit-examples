use yew::prelude::*;

use pwt::props::WidgetBuilder;
use pwt::widget::nav_menu::{MenuItem, NavigationMenu};

#[function_component]
pub fn NavigationDemo() -> Html {
    NavigationMenu::new()
        .with_item(MenuItem::new("id1", "Entry 1", "fa fa-paper-plane", |_| {
            html! { "Entry 1" }
        }))
        .with_item(
            MenuItem::new("id2", "Submenu 1", "fa fa-car", |_| html! {"Submenu 1"})
                .submenu()
                .with_item(MenuItem::new(
                    "id3",
                    "Entry 2",
                    "fa fa-motorcycle",
                    |_| html! { "Entry 2"},
                ))
                .with_item(MenuItem::new(
                    "id4",
                    "Entry 3",
                    "fa fa-bicycle",
                    |_| html! { "Entry 3" },
                ))
                .with_item(
                    MenuItem::new("id5", "Submenu 2", "fa fa-ship", |_| html! { "Submenu 2"})
                        .submenu()
                        .with_item(MenuItem::new(
                            "id6",
                            "Entry 4",
                            "fa fa-rocket",
                            |_| html! { "Entry 4" },
                        )),
                ),
        )
        .with_item(
            MenuItem::new(
                "unselectable",
                "Submenu 3 (unselectable)",
                "fa fa-ship",
                |_| html! { "UNSELECTEABLE"},
            )
            .submenu()
            .unselectable()
            .with_item(MenuItem::new(
                "unselectable-1",
                "Entry 4",
                "fa fa-rocket",
                |_| html! { "Entry 4" },
            )),
        )
        .with_item(MenuItem::new("id7", "Entry 5", "", |_| html! { "Entry 5" }))
        .class("pwt-border-left")
        .navigation_container()
        .into()
}
