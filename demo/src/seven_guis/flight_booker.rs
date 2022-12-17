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

pub struct FlightBooker {
    with_return_flight: bool,
    enable_book: bool,
    start_date: Option<String>,
    return_date: Option<String>,
}

pub enum Msg {
    Mode(String),
    StartDate(String),
    ReturnDate(String),
}

impl Component for FlightBooker {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            with_return_flight: false,
            start_date: None,
            return_date: None,
            enable_book: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Mode(mode) => {
                self.with_return_flight = mode != "one-way flight";
            }
            Msg::StartDate(date) => {
                self.start_date = Some(date);
            }
            Msg::ReturnDate(date) => {
                self.return_date = Some(date);
            }
        }

        let mut enable_book = true;
        if let Some(date_str) = &self.start_date {
            let start_date = js_sys::Date::parse(date_str);
            if start_date.is_nan() {
                enable_book = false;
            } else {
                if self.with_return_flight {
                    if let Some(date_str) = &self.return_date {
                        let return_date = js_sys::Date::parse(date_str);
                        if return_date.is_nan() {
                            enable_book = false;
                        } else {
                            if return_date < start_date {
                                enable_book = false;
                            }
                        }
                    } else {
                        enable_book = false;
                    }
                }
            }
        } else {
            enable_book = false;
        }

        self.enable_book = enable_book;

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let combo = Combobox::new()
            .default("one-way flight")
            .items(FLIGHT_MODE.with(Rc::clone))
            .on_change(ctx.link().callback(Msg::Mode));

        let flight_date = Field::new()
            .required(true)
            .on_change(ctx.link().callback(Msg::StartDate))
            .input_type("date");

        let return_date = Field::new()
            .required(true)
            .disabled(!self.with_return_flight)
            .on_change(ctx.link().callback(Msg::ReturnDate))
            .input_type("date");

        let book = Button::new("Book").disabled(!self.enable_book);

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
}
