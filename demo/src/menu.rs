use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{Column, Menu, MenuItem, Button, Container};

#[function_component]
pub fn MenuDemo() -> Html {

    let submenu2 = MenuItem::new("Submenu2")
        .menu(
            Menu::new()
                .with_item(MenuItem::new("Submenu2 Item1"))
                .with_item(MenuItem::new("Submenu2 Item2"))
                .with_item(MenuItem::new("Submenu2 Item3"))
        );
    
     let submenu1 = MenuItem::new("Submenu1")
        .icon_class("fa fa-coffee")
        .menu(
            Menu::new()
                .with_item(MenuItem::new("Submenu1 Item1"))
                .with_item(submenu2)
                .with_item(<Container as Into<Html>>::into(
                    Container::new()
                        .padding((0, 2))
                        .with_child(Button::new("Button2").tabindex(-1).class("pwt-menu-item-indent"))
                ))
                .with_item(MenuItem::new("Submenu1 Item2"))
        );
   
    Column::new()
        .padding(2)
        .gap(2)
        .with_child("Simple Menu:")
        .with_child(
            Menu::new()
                .with_item(MenuItem::new("Disabled Item").icon_class("fa fa-trash-o").disabled(true))
                .with_item(MenuItem::new("Normal item with icon").icon_class("fa fa-fire-extinguisher"))
                .with_item(submenu1)
                .with_separator()
                .with_item(MenuItem::new("MenuItem4"))
                .with_item(<Container as Into<Html>>::into(
                    Container::new()
                        .padding((0, 2))
                        .with_child(Button::new("A Button").tabindex(-1).class("pwt-menu-item-indent"))
                ))
                .with_item(MenuItem::new("MenuItem5"))
        )
        .into()
}
