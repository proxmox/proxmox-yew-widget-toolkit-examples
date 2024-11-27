use pwt::prelude::*;

use pwt::widget::{List, ListTile, Panel};

#[function_component]
pub fn ListDemo() -> Html {
    let content = List::new(100000, |pos: &u64| {
            let disabled = pos % 2 == 1;
            let text =  format!("This is a item inside a virtually scrolled list. Only the visible records are actually rendered. This is record {}", pos);
            ListTile::new()
                .with_child(html!{<span>{text}</span>})
                .with_child(html!{<span>{if disabled { "disabled" } else { "enabled" }}</span>})
                .interactive(true).disabled(disabled)
                .style("justify-content", "space-between")
                //.class(css::ColorScheme::Primary)
                .into()
        })
        .class("pwt-flex-fit")
        .margin(1)
        .padding(2)
        .separator(|_pos: &u64| html! {<hr/>});

    Panel::new()
        .class(pwt::css::FlexFit)
        .style("box-sizing", "border-box")
        .title("A List using virtual scrolling")
        .with_child(content)
        .into()
}
