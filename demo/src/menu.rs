use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{Column, Menu, MenuItem, MenuButton, Button, Container, Panel, Toolbar};

fn panel_with_menu_button() -> Html {

    let toolbar = Toolbar::new()
        .class("pwt-border-bottom")
        .with_child(
            MenuButton::new("MyMenu").menu(
                Menu::new()
                    .with_item(MenuItem::new("MyMenu Item1"))
                    .with_item(MenuItem::new("MyMenu Item2"))
                    .with_item(MenuItem::new("Disabled Item").icon_class("fa fa-trash-o").disabled(true))
                    .with_item(MenuItem::new("MyMenu Item3"))
            )
        );

    let text = "Menu buttons are useful inside toolbars.";

    Panel::new()
        .title("Menu Button Example")
        .with_child(toolbar)
        .with_child(html!{<div class="pwt-p-2">{text}</div>})
        .into()

}

fn panel_with_menubar() -> Html {

    let longmenu = MenuItem::new("Long Submenu")
        .menu(
            Menu::new()
                .items((0..50).map(|i| MenuItem::new(format!("Item{i}")).into()))
       );

    let submenu3 = MenuItem::new("Submenu3")
        .menu(
            Menu::new()
                .with_item(MenuItem::new("Submenu3 disabled Item1").disabled(true))
                .with_item(MenuItem::new("Submenu3 disabled Item2").disabled(true))
        );

    let submenu2 = MenuItem::new("Submenu2")
        .menu(
            Menu::new()
                .with_item(MenuItem::new("Submenu2 Item1"))
                .with_separator()
                .with_item(MenuItem::new("Submenu2 Item2"))
                .with_separator()
                .with_item(longmenu.clone())
                .with_item(MenuItem::new("Submenu2 Item3"))
        );

    let submenu1 = MenuItem::new("Submenu1")
        .icon_class("fa fa-coffee")
        .menu(
            Menu::new()
                .with_item(MenuItem::new("Submenu1 Item1"))
                .with_item(submenu2.clone())
                .with_item(<Container as Into<Html>>::into(
                    Container::new()
                        .padding((0, 2))
                        .with_child(Button::new("Button2").tabindex(-1).class("pwt-menu-item-indent"))
                ))
                .with_item(submenu3)
                .with_item(MenuItem::new("Submenu1 Item2"))
        );

    let menubar = Menu::new_menubar()
        .with_item(submenu1)
        .with_item(submenu2)
        .with_separator()
        .with_item(longmenu)
        .with_separator()
        .with_item(MenuItem::new("No Submenu with icon").icon_class("fa fa-fire-extinguisher"))
        .with_item(MenuItem::new("MenuItem4"))
        .with_item(MenuItem::new("MenuItem5"));

    let text = "This panel contains a menubar.";

    Panel::new()
        .title("Menubar Example")
        .with_child(Container::new().with_child(menubar).class("pwt-border-bottom"))
        .with_child(html!{<div class="pwt-p-2">{text}</div>})
        .into()

}

#[function_component]
pub fn MenuDemo() -> Html {
    Column::new()
        .class("pwt-fit")
        .padding(2)
        .gap(2)
        .with_child(panel_with_menu_button())
        .with_child(panel_with_menubar())
        .into()
}
