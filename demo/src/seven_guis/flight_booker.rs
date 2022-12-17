use std::rc::Rc;

use pwt::prelude::*;
use yew::prelude::*;

use pwt::widget::{Button, Column, Panel};
use pwt::widget::form::{Combobox, Field};

thread_local!{
    static FLIGHT_MODE: Rc<Vec<AttrValue>> = Rc::new(vec![
        "one-way flight".into(),
        "return flight".into(),
    ]);
}
#[function_component]
pub fn FlightBooker() -> Html {

    let combo = Combobox::new()
        .default("one-way flight")
        .items(FLIGHT_MODE.with(Rc::clone));

    let flight_date = Field::new()
        .input_type("date");

    let return_date = Field::new()
        .input_type("date");
    
    let book = Button::new("Book");
 
    Panel::new()
        .attribute("style", "width:200px;")
        .title("Flight Booker")
        .with_child(
            Column::new()
                .padding(2)
                .gap(2)
                .with_child(combo)
                .with_child(flight_date)
                .with_child(return_date)
                .with_child(book)
        )
        .into()
}
