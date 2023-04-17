use yew::prelude::*;

use pwt::prelude::*;
use pwt::widget::{Container, Pane, SplitPane};
//use pwt::widget::nav_menu::{NavMenuItem, NavMenu};
use pwt::widget::nav::{NavMenu, NavMenuItem, NavigationDrawer};

#[function_component]
pub fn NavigationDemo() -> Html {

    let content = use_state(|| html!{});

    let activate_view = |text: &str| -> Callback<()> {
        let content = content.clone();
        let text = text.to_string();
        Callback::from(move |()| {
            //log::info!("ACTIVATE VIEW {}", text);
            content.set(html!{text.clone()});
        })
    };

    let menu = NavMenu::new()
        .with_item(
            NavMenuItem::new("Entry 1")
                .key("id1")
                .icon_class("fa fa-paper-plane")
                .on_activate(activate_view("Entry 1")),
        )
        .with_item(
            NavMenuItem::new("Submenu 1")
                .key("id2")
                .icon_class("fa fa-car")
                .on_activate(activate_view("Submenu 1"))
                .submenu(
                    NavMenu::new()
                        .with_item(
                            NavMenuItem::new("Entry 2")
                                .key("id3")
                                .icon_class("fa fa-motorcycle")
                                .on_activate(activate_view("Entry 2"))
                        )
                        .with_item(
                            NavMenuItem::new("Entry 3")
                                .key("id4")
                                .icon_class("fa fa-bicycle")
                                .on_activate(activate_view("Entry 3")),
                        )
                        .with_item(
                            NavMenuItem::new("Submenu 2")
                                .key("id5")
                                .icon_class("fa fa-ship")
                                .on_activate(activate_view("Submenu 2"))
                                .submenu(
                                    NavMenu::new().with_item(
                                        NavMenuItem::new("Entry 4")
                                            .key("id6")
                                            .icon_class("fa fa-rocket")
                                            .on_activate(activate_view("Entry 4")),
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
                .on_activate(activate_view("UNSELECTEABLE")) // never triggers
                .submenu(
                    NavMenu::new()
                        .with_item(
                            NavMenuItem::new("Entry 5")
                                .key("id7")
                                .icon_class("fa fa-rocket")
                                .on_activate(activate_view("Entry 5"))
                        ),
                ),
        )
        .with_item(
            NavMenuItem::new("Entry 6")
                .key("id8")
                .on_activate(activate_view("Entry 6"))
        );

    let drawer = NavigationDrawer::new(menu)
        .class("pwt-border-left")
        .navigation_container();

    let content = Container::new().padding(2).with_child((&*content).clone());

    SplitPane::new()
            .class("pwt-fit")
            .with_child(Pane::new(drawer).size(None))
            .with_child(Pane::new(content).flex(1))
            .into()
}
