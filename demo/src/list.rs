use pwt::prelude::*;

use pwt::widget::{Column, Container, List, ListTile, Panel};

fn flex_layout_list() -> Panel {
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
   .class("pwt-flex-fit")
   .separator(|_pos| html! {<hr/>});

    Panel::new()
        .class(pwt::css::FlexFit)
        .border(true)
        .title("A List using virtual scrolling (flex-row layout, justify-content: space-between)")
        .with_child(content)
}

fn grid_layout_list() -> Panel {
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
   .class("pwt-flex-fit")
   .separator(|_pos| html! {<hr/>})
   .grid_template_columns("auto 1fr auto");

    Panel::new()
        .class(pwt::css::FlexFit)
        .border(true)
        .title("A List using virtual scrolling (grid layout, grid_template_columns: auto 1fr auto)")
        .with_child(content)
}

#[function_component]
pub fn ListDemo() -> Html {
    Column::new()
        .class(pwt::css::FlexFit)
        .padding(2)
        .gap(3)
        .with_child(flex_layout_list())
        .with_child(grid_layout_list())
        .into()
}
