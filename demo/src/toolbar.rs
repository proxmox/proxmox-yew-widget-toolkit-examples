use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{Column, Button, Menu, MenuButton, MenuItem, Panel, Toolbar};
use pwt::widget::form2::Checkbox;

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
        .with_child(Checkbox::new())
        .with_child(menu_button)
        ;

    let panel = Panel::new()
        .title("A panel with a toolbar")
        .with_child(toolbar)
        .with_child(html!{<div class="pwt-p-2"><p>{"This is the panel body."}</p></div>});

    Column::new()
        .class("pwt-fit")
        .padding(2)
        .with_child(panel)
        .into()
}
