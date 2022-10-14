use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{dispatch_menu_close_event, Column, Menu, MenuItem, MenuButton, MenuCheckbox, Button, Container, Panel, Toolbar};
use pwt::widget::form::Form;

fn panel_with_menu_button() -> Html {

    let submenu1 = MenuItem::new("Submenu1")
        .menu(
            Menu::new()
                .with_item(MenuItem::new("Submenu1 Item1"))
                .with_item(MenuItem::new("Submenu1 Item2"))
        );

    let toolbar = Toolbar::new()
        .class("pwt-border-bottom")
        .with_child(
            MenuButton::new("MyMenu").menu(
                Menu::new()
                    .with_item(MenuItem::new("MyMenu Item1"))
                    .with_item(
                        MenuItem::new(html!{<a target="_blank" href="http://google.com">{"open google.com"}</a>})
                            .focusable(true)
                    )
                    .with_item(MenuItem::new("Disabled Item").icon_class("fa fa-trash-o").disabled(true))
                    .with_item(submenu1)
                    .with_item(MenuItem::new("MyMenu Item3"))
            )
        );

    let text = "Menu buttons are useful inside toolbars.";

    Panel::new()
        .title("Menu Button Example")
        .with_child(toolbar)
        .with_child(html!{<p class="pwt-p-2">{text}</p>})
        .into()

}

fn panel_with_menubar() -> Html {

    let longmenu = MenuItem::new("Long Submenu")
        .menu(
            Menu::new()
                .items((0..50).map(|i| MenuItem::new(format!("Item{i}")).into()))
        );

    let radiogroup = MenuItem::new("RadioGroup test")
        .menu(
            Menu::new()
                .with_item(MenuCheckbox::new("Value1").group("mygroup").name("opt1"))
                .with_item(MenuCheckbox::new("Value2").group("mygroup").name("opt2"))
                .with_item(MenuCheckbox::new("Value3").group("mygroup").name("opt3"))
                .with_item(MenuCheckbox::new("Value4").group("mygroup").name("opt4"))
        );

    let checkboxes = MenuItem::new("CheckboxTest")
        .menu(
            Menu::new()
                .with_item(radiogroup)
                .with_item(MenuCheckbox::new("Checkbox1").name("check1"))
                .with_item(MenuCheckbox::new("Checkbox2").name("check2"))
                .with_item(MenuCheckbox::new("Checkbox3").name("check3"))
                .with_item(MenuCheckbox::new("Checkbox4").name("check4"))
                .with_item(MenuCheckbox::new("Checkbox5").name("check5"))
        );

    let submenu3 = MenuItem::new("Submenu3")
        .menu(
            Menu::new()
                .with_item(MenuItem::new("Submenu3 disabled Item1").disabled(true))
                .with_item(MenuItem::new("Submenu3 disabled Item2").disabled(true))
                .with_item(MenuCheckbox::new("A Checkbox"))
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
                        .with_child(
                            Button::new("Button2")
                                .tabindex(-1)
                                .class("pwt-menu-item-indent")
                                .onclick(|event: MouseEvent| {
                                    dispatch_menu_close_event(&event);
                                })
                        )
                ))
                .with_item(submenu3)
                .with_item(MenuItem::new("Submenu1 Item2"))
        );

    let menubar = Menu::new_menubar()
        .with_item(submenu1)
        .with_item(checkboxes)
        .with_item(submenu2)
        .with_separator()
        .with_item(longmenu)
        .with_separator()
        .with_item(MenuItem::new("No Submenu with icon").icon_class("fa fa-fire-extinguisher"))
        .with_item(MenuItem::new("MenuItem4"))
        .with_item(MenuItem::new("MenuItem5"));

    let text = "This panel contains a menubar. The menubar is inside a
    Form which provides the context to store checkbox/radiobox
    values.";

    Panel::new()
        .title("Menubar Example")
        .with_child(
             Form::new(move |_form_ctx| {
                 Container::new()
                     .with_child(menubar.clone())
                     .class("pwt-border-bottom")
                     .into()
             })
        )
        .with_child(html!{<p class="pwt-p-2">{text}</p>})
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
