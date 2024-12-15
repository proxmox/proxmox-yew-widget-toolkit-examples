mod dynamic;
use dynamic::DynamicListDemo;

mod variable;
use variable::VariableListDemo;

use pwt::prelude::*;

use pwt::state::NavigationContainer;
use pwt::widget::{Card, Column, Container, List, ListTile, TabBarItem, TabBarStyle, TabPanel};

fn list_simple() -> Html {
    let content = List::new(10000, |pos: u64| {
        let disabled = pos % 2 == 1;
        let text =  format!("This is a item inside a virtually scrolled list. Only the visible records are actually rendered. This is record {}", pos);
        ListTile::new()
            .style("display", "flex") // use flex row layout
            .with_child(html!{<span>{text}</span>})
            .with_child(html!{<span>{if disabled { "disabled" } else { "enabled" }}</span>})
            .interactive(true).disabled(disabled)
            .style("justify-content", "space-between")
            //.class(pwt::css::ColorScheme::Primary)
    })
    .separator(true)
   .class("pwt-flex-fit");

    Column::new()
        .class(pwt::css::FlexFit)
        .with_child(
            Container::new()
                .padding(2)
                .border_bottom(true)
                .with_child(html! {<h5>{"A List using virtual scrolling"}</h5>})
                .with_child(html! {
                    <p>{"Settings: flex-row layout, justify-content: space-between"}</p>
                }),
        )
        .with_child(content)
        .into()
}

fn list_cards() -> Html {
    let content = List::new(1000, |pos: u64| {
        ListTile::new().disabled((pos % 2) == 1).with_child(
            Card::new()
                .class("pwt-interactive")
                .height(60)
                //.class(pwt::css::ColorScheme::Primary)
                .with_child(format!("Item {pos}")),
        )
    })
    .class("pwt-flex-fit");

    Column::new()
        .class(pwt::css::FlexFit)
        .with_child(Container::new().padding(2).border_bottom(true).with_child(
            html! {<h5>{"A List using virtual scrolling using card layout for tiles"}</h5>},
        ))
        .with_child(content)
        .into()
}

fn list_columns() -> Html {
    let content = List::new(10000, |pos: u64| {
        let disabled = pos % 2 == 1;
        let text =  format!("This is a item inside a virtually scrolled list. Only the visible records are actually rendered. This is record {}", pos);
        ListTile::new()
            .with_child(Container::new().with_child(pos.to_string()).padding_end(2))
            .with_child(html!{<span>{text}</span>})
            .with_child(Container::new().with_child(if disabled { "disabled" } else { "enabled" }).padding_start(2))
            .interactive(true).disabled(disabled)
            //.class(pwt::css::ColorScheme::Primary)
    })
    .separator(true)
   .class("pwt-flex-fit")
   .grid_template_columns("auto 1fr auto");

    Column::new()
        .class(pwt::css::FlexFit)
        .with_child(
            Container::new()
                .padding(2)
                .border_bottom(true)
                .with_child(html! {<h5>{"A List using virtual scrolling and grid columns"}</h5>})
                .with_child(html! {"Settings: grid layout, grid_template_columns: auto 1fr auto)"}),
        )
        .with_child(content)
        .into()
}

#[function_component]
pub fn ListDemo() -> Html {
    let tap_panel = TabPanel::new()
        .class(pwt::css::FlexFit)
        .router(true)
        .tab_bar_style(TabBarStyle::MaterialPrimary)
        .with_item_builder(
            TabBarItem::new().key(format!("simple")).label("Simple"),
            |_| list_simple(),
        )
        .with_item_builder(
            TabBarItem::new()
                .key(format!("variable"))
                .label("Variable Size"),
            |_| html! {<VariableListDemo/>},
        )
        .with_item_builder(
            TabBarItem::new().key(format!("column")).label("Columns"),
            |_| list_columns(),
        )
        .with_item_builder(
            TabBarItem::new().key(format!("cards")).label("Cards"),
            |_| list_cards(),
        )
        .with_item_builder(
            TabBarItem::new().key(format!("dynamic")).label("Dynamic"),
            |_| html! {<DynamicListDemo/>},
        );

    NavigationContainer::new().with_child(tap_panel).into()
}
