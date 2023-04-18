use pwt::state::Selection;
use yew::prelude::*;
use yew::virtual_dom::Key;

use pwt::prelude::*;
use pwt::widget::{Container, Pane, SplitPane};
//use pwt::widget::nav_menu::{NavMenuItem, NavMenu};
use pwt::widget::nav::{NavMenu, NavMenuItem, NavigationDrawer};

#[function_component]
pub fn NavigationDemo() -> Html {

    let content = use_state(|| html!{});

    let menu = NavMenu::new()
       .with_item(
            NavMenuItem::new("Entry 1")
                .key("id1")
                .icon_class("fa fa-paper-plane")
        )
        .with_item(
            NavMenuItem::new("Submenu 1")
                .key("id2")
                .icon_class("fa fa-car")
                .submenu(
                    NavMenu::new()
                        .with_item(
                            NavMenuItem::new("Entry 2")
                                .key("id3")
                                .icon_class("fa fa-motorcycle")
                        )
                        .with_item(
                            NavMenuItem::new("Entry 3")
                                .key("id4")
                                .icon_class("fa fa-bicycle")
                        )
                        .with_item(
                            NavMenuItem::new("Submenu 2")
                                .key("id5")
                                .icon_class("fa fa-ship")
                                .submenu(
                                    NavMenu::new().with_item(
                                        NavMenuItem::new("Entry 4")
                                            .key("id6")
                                            .icon_class("fa fa-rocket")
                                    ),
                                ),
                        ),
                ),
        )
        .with_item(
            NavMenuItem::new("Submenu 3 (unselectable)")
                .key("unselectable")
                .selectable(false)
                .icon_class("fa fa-ship")
                .submenu(
                    NavMenu::new()
                        .with_item(
                            NavMenuItem::new("Entry 5")
                                .key("id7")
                                .icon_class("fa fa-rocket")
                        ),
                ),
        )
        .with_item(
            NavMenuItem::new("Entry 6")
                .key("id8")
        );

    let drawer = NavigationDrawer::new(menu)
        .class("pwt-border-left")
        .on_select({
            let content = content.clone();
            move |key: Option<Key>| {
                content.set(html!{format!("Selected Key is {:?}", key)});
            }
        })
        .navigation_container();


    let content = Container::new().padding(2).with_child((&*content).clone());

    SplitPane::new()
            .class("pwt-fit")
            .with_child(Pane::new(drawer).size(None))
            .with_child(Pane::new(content).flex(1))
            .into()
}
