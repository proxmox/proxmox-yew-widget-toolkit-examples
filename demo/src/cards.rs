use pwt::prelude::*;
use pwt::widget::{Card, Column};

fn card_with_text(text: &str) -> Card {
    Card::new()
        .class("pwt-d-flex pwt-justify-content-center pwt-align-items-center")
        .with_child(text)
}

#[function_component]
pub fn CardsDemo() -> Html {

    let mut column = Column::new()
        .class("pwt-fit")
        .padding(2)
        .gap(4)
        .with_child(card_with_text("Default Card"));

    for i in 0..6 {
        column.add_child(
            card_with_text(&format!("Elevantionm level {i}"))
                .class(format!("pwt-elevation{i}"))
        )
    }

    column.into()
}
