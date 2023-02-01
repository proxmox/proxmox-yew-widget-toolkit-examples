use std::rc::Rc;

use pwt::prelude::*;
use pwt::widget::{Button, Column, Container, Panel, Row};
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
    view_booked: Option<(String, Option<String>)>,
}

impl FlightBooker {

    fn view_booking(&self, ctx: &Context<Self>, booking: &(String, Option<String>)) -> Html {
        let msg = match booking {
            (start_date, None) => {
                format!("You have booked a one-way flight on {start_date}")
            }
            (start_date, Some(return_date)) => {
                format!("You have booked a return flight on {start_date}, returning on {return_date}")
            }
        };
        Container::new()
            .padding(2)
            .with_child(
                Column::new()
                    .gap(2)
                    .with_child(msg)
                    .with_child(
                        Button::new("Continue")
                            .onclick(ctx.link().callback(|_| Msg::Continue))
                    )
            )
            .into()
    }

    fn view_form(&self, ctx: &Context<Self>) -> Html {
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

        let book = Button::new("Book")
            .disabled(!self.enable_book)
            .onclick(ctx.link().callback(|_| Msg::Book));

        Column::new()
            .padding(2)
            .gap(2)
            .with_child(combo)
            .with_child(flight_date)
            .with_child(return_date)
            .with_child(book)
            .into()
    }
}

pub enum Msg {
    Book,
    Continue,
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
            view_booked: None,
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
            Msg::Continue => {
                self.view_booked = None;
            }
            Msg::Book => {
                if let Some(start_date) = self.start_date.take() {
                    self.view_booked = Some((start_date, self.return_date.take()));
                }
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
        let body: Html = match &self.view_booked {
            Some(data) => self.view_booking(ctx, data),
            None => self.view_form(ctx),
        };

        Row::new().with_child(
            Panel::new()
                .title("Flight Booker")
                .with_child(body)
        ).into()
    }
}
