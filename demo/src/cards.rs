use pwt::prelude::*;
use pwt::widget::{Card, Column, Row, Fa};
use pwt::touch::{Fab, FabMenu, FabMenuAlign, FabMenuDirection};

fn card_with_text(text: &str) -> Card {
    Card::new()
        .class("pwt-d-flex pwt-justify-content-center pwt-align-items-center pwt-p-4")
        .with_child(text)
}

#[function_component]
pub fn CardsDemo() -> Html {

    let mut column = Column::new()
        .class("pwt-fit")
        .padding(2)
        .gap(4)
        .with_child(
            Row::new()
                .gap(4)
                .class("pwt-align-items-center")
                .with_child(Fab::new("fa fa-pencil").large())
                .with_child(Fab::new("fa fa-pencil"))
                .with_child(Fab::new("fa fa-pencil").small().class("pwt-scheme-primary-container"))
                .with_child(
                    FabMenu::new()
                        .main_button_class("pwt-scheme-primary")
                        .direction(FabMenuDirection::Down)
                        .align(FabMenuAlign::Start)
                        .with_child(Fab::new("fa fa-pencil").text("Edit"))
                        .with_child(Fab::new("fa fa-trash").text("Delete"))
                )
        )
        .with_child(card_with_text("Default Card"));

    for i in 0..6 {
        column.add_child(
            card_with_text(&format!("Elevantion level {i}, Interactive"))
                .class(format!("pwt-elevation{i}"))
                .class("pwt-interactive")
        )
    }

    column.into()
}
