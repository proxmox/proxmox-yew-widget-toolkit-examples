use yew::prelude::*;

use pwt::prelude::*;
use pwt::widget::Container;
use pwt::widget::nav_menu::{NavMenuItem, NavMenu};

#[function_component]
pub fn NavigationDemo() -> Html {

    fn panel_content(text: &str) -> Html {
        Container::new()
            .padding(2)
            .with_child(text)
            .into()
    }

    NavMenu::new()
        .with_item(NavMenuItem::new("id1", "Entry 1", "fa fa-paper-plane", |_| {
            panel_content("Entry 1")
        }))
        .with_item(
            NavMenuItem::new("id2", "Submenu 1", "fa fa-car", |_| panel_content("Submenu 1"))
                .submenu()
                .with_item(NavMenuItem::new(
                    "id3",
                    "Entry 2",
                    "fa fa-motorcycle",
                    |_| panel_content("Entry 2"),
                ))
                .with_item(NavMenuItem::new(
                    "id4",
                    "Entry 3",
                    "fa fa-bicycle",
                    |_| panel_content("Entry 3"),
                ))
                .with_item(
                    NavMenuItem::new("id5", "Submenu 2", "fa fa-ship", |_| panel_content("Submenu 2"))
                        .submenu()
                        .with_item(NavMenuItem::new(
                            "id6",
                            "Entry 4",
                            "fa fa-rocket",
                            |_| panel_content("Entry 4"),
                        )),
                ),
        )
        .with_item(
            NavMenuItem::new(
                "unselectable",
                "Submenu 3 (unselectable)",
                "fa fa-ship",
                |_| panel_content("UNSELECTEABLE"),
            )
            .submenu()
            .unselectable()
            .with_item(NavMenuItem::new(
                "unselectable-1",
                "Entry 4",
                "fa fa-rocket",
                |_| panel_content("Entry 4"),
            )),
        )
        .with_item(NavMenuItem::new("id7", "Entry 5", "", |_| panel_content("Entry 5")))
        .class("pwt-border-left")
        .navigation_container()
        .into()
}
