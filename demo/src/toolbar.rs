use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{Column, Button, Panel, Toolbar};
use pwt::widget::menu::{Menu, MenuButton, MenuItem};
use pwt::widget::form::Checkbox;

use crate::LoremIpsum;

#[function_component]
pub fn ToolbarDemo() -> Html {

    let menu_button = MenuButton::new("MyMenu").menu(
        Menu::new()
            .with_item(MenuItem::new("Item1"))
            .with_item(MenuItem::new("Item2"))
    );

    let toolbar = Toolbar::new()
        .class("pwt-border-bottom")
        .with_child(Button::new("Button1"))
        .with_child(Button::new("Button2"))
        .with_child(Button::new("Button3").disabled(true))
        .with_child(Checkbox::new())
        .with_child(menu_button)
        ;

    let panel = Panel::new()
        .title("A panel with a toolbar")
        .with_child(toolbar)
        .with_child(html!{<LoremIpsum/>});

    Column::new()
        .class("pwt-fit")
        .padding(2)
        .with_child(panel)
        .into()
}
