use pwt::css::{Flex, Overflow};
use pwt::prelude::*;
use pwt::widget::{Card, Column};

fn card_with_text(text: &str) -> Card {
    Card::new()
        .class("pwt-d-flex pwt-justify-content-center pwt-align-items-center pwt-p-4")
        .with_child(text)
}

#[function_component]
pub fn CardsDemo() -> Html {
    let mut column = Column::new()
        .class(Flex::Fill)
        .class(Overflow::Auto)
        .padding(2)
        .gap(4)
        .with_child(card_with_text("Default Card"));

    for i in 0..6 {
        column.add_child(
            card_with_text(&format!("Elevantion level {i}, Interactive"))
                .class(format!("pwt-elevation{i}"))
                .class("pwt-interactive"), //.class("pwt-scheme-primary-container")
                                           //.class("pwt-card-outline")
        )
    }

    column
        .with_child(
            card_with_text("Filled Card (primary)")
                .class("pwt-interactive")
                .class("pwt-scheme-primary"),
        )
        .with_child(
            card_with_text("Outline Card")
                .class("pwt-interactive")
                .class("pwt-card-outline"),
        )
        .with_child(
            card_with_text("Flat Card")
                .class("pwt-card-flat")
                .class("pwt-interactive"),
        )
        .with_child(
            card_with_text("Rectangular Card")
                .class("pwt-interactive")
                .class("pwt-shape-none"),
        )
        .into()
}
