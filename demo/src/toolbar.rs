use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{Column, Button, MiniScroll, Panel, Toolbar};
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
        .with_child(Button::new("Button4"))
        .with_child(Button::new("Button5"))
        .with_child(Button::new("Button6"))
        .with_child(Button::new("Button7"))
        .with_child(menu_button)
        ;

    let panel = Panel::new()
        .title("A panel with a toolbar")
        .with_child(MiniScroll::new(toolbar))
        .with_child(html!{<LoremIpsum/>});

    Column::new()
        .class("pwt-size-fit-content")
        .padding(2)
        .with_child(panel)
        .into()
}
