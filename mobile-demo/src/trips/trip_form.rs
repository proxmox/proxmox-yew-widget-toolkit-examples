use std::rc::Rc;

use yew_router::prelude::*;

use pwt::css::{ColorScheme, JustifyContent};
use pwt::touch::prelude::*;
use pwt::widget::form::{Combobox, DateField, Number, PlainDate};
use pwt::widget::{Button, Column, InputPanel, Row};

use crate::app_state;
use crate::data::{find_destination, DESTINATIONS};
use crate::messages::show_with_action;
use crate::route::Route;

/// Collects the details of a trip and adds it.
#[derive(Properties, PartialEq)]
pub struct TripForm {
    /// The destination of the trip. Without one, the form lets the user choose it.
    #[prop_or_default]
    destination: Option<&'static str>,
    /// Called when the user is done, by adding the trip or by canceling.
    on_done: Callback<()>,
}

impl TripForm {
    pub fn new(on_done: impl Into<Callback<()>>) -> Self {
        yew::props!(Self {
            on_done: on_done.into()
        })
    }

    /// Plan the trip to this destination instead of letting the user choose one.
    pub fn destination(mut self, id: &'static str) -> Self {
        self.destination = Some(id);
        self
    }
}

into_vnode!(TripForm => TripFormComp);

const DATE_FORMAT: &str = "Y-m-d";

/// The travelers of a new trip, unless the user says otherwise.
pub const DEFAULT_TRAVELERS: u32 = 2;

/// The start of a new trip, unless the user says otherwise.
pub fn default_start() -> PlainDate {
    PlainDate::today().add_days(14)
}

enum Msg {
    Destination(String),
    Start(Option<PlainDate>),
    Travelers(Option<u32>),
    Add,
    Cancel,
}

struct TripFormComp {
    destination: Option<&'static str>,
    start: Option<PlainDate>,
    travelers: Option<u32>,
}

impl Component for TripFormComp {
    type Message = Msg;
    type Properties = TripForm;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            destination: ctx.props().destination,
            start: Some(default_start()),
            travelers: Some(DEFAULT_TRAVELERS),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Destination(name) => {
                self.destination = DESTINATIONS.iter().find(|d| d.name == name).map(|d| d.id)
            }
            // the date field also reports dates before its minimum, it only marks them invalid
            Msg::Start(start) => self.start = start.filter(|date| *date >= PlainDate::today()),
            Msg::Travelers(travelers) => self.travelers = travelers,
            Msg::Cancel => ctx.props().on_done.emit(()),
            Msg::Add => {
                let (Some(id), Some(start), Some(travelers)) =
                    (self.destination, self.start, self.travelers)
                else {
                    return false;
                };
                let Some(destination) = find_destination(id) else {
                    return false;
                };
                app_state().write().add_trip(id, start, travelers);
                ctx.props().on_done.emit(());
                if let (Some(snackbar), Some(navigator)) =
                    (ctx.link().snackbar_controller(), ctx.link().navigator())
                {
                    let message = format!("Trip to {} planned", destination.name);
                    show_with_action(&snackbar, message, "Show", move || {
                        navigator.push(&Route::Trips)
                    });
                }
                return false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        // the input panel links each label to its field, for screen readers
        let mut fields = InputPanel::new().mobile(true);

        if ctx.props().destination.is_none() {
            let names: Rc<Vec<AttrValue>> = Rc::new(
                DESTINATIONS
                    .iter()
                    .map(|d| AttrValue::from(d.name))
                    .collect(),
            );
            let selected = self.destination.and_then(find_destination).map(|d| d.name);
            fields.add_field(
                "Destination",
                Combobox::new()
                    .items(names)
                    .placeholder("Where do you want to go?")
                    .value(selected)
                    .on_change(link.callback(Msg::Destination)),
            );
        }

        fields.add_field(
            "Start date",
            DateField::new()
                .format(DATE_FORMAT)
                .default(default_start().format(DATE_FORMAT))
                .min_value(PlainDate::today())
                .on_change(link.callback(Msg::Start)),
        );

        fields.add_field(
            "Travelers",
            Number::<u32>::new()
                .min(1)
                .max(9)
                .default(DEFAULT_TRAVELERS)
                .on_change(link.callback(|value: Option<Result<u32, String>>| {
                    Msg::Travelers(value.and_then(|value| value.ok()))
                })),
        );

        let valid = self.destination.is_some() && self.start.is_some() && self.travelers.is_some();

        let buttons = Row::new()
            .gap(2)
            .padding_top(2)
            .class(JustifyContent::End)
            .with_child(Button::new("Cancel").onclick(link.callback(|_| Msg::Cancel)))
            .with_child(
                Button::new("Add trip")
                    .class(ColorScheme::Primary)
                    .disabled(!valid)
                    .onclick(link.callback(|_| Msg::Add)),
            );

        Column::new()
            .min_width(280)
            .with_child(fields)
            .with_child(buttons)
            .into()
    }
}
