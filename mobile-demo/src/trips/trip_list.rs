use yew::html::Scope;

use pwt::css::{AlignItems, ColorScheme, FlexFit, FontStyle, TextAlign};
use pwt::state::SharedStateObserver;
use pwt::touch::prelude::*;
use pwt::touch::{Slidable, SlidableAction, SlidableActionMouseEvent};
use pwt::widget::form::PlainDate;
use pwt::widget::{Badge, Column, Container, Image, Row};

use crate::data::find_destination;
use crate::messages::show_with_action;
use crate::{app_state, observe_app_state, AppState, Trip};

/// The planned and the visited trips.
#[derive(Properties, PartialEq, Default)]
pub struct TripList {}

impl TripList {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(TripList => TripListComp);

const MONTHS: &[&str] = &[
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

fn format_date(date: &PlainDate) -> String {
    let month = MONTHS
        .get(date.month() as usize)
        .copied()
        .unwrap_or_default();
    format!("{} {month} {}", date.day(), date.year())
}

enum Msg {
    StateChanged,
    SetVisited(u32, bool),
    Delete(u32),
}

struct TripListComp {
    _observer: SharedStateObserver<AppState>,
}

impl Component for TripListComp {
    type Message = Msg;
    type Properties = TripList;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _observer: observe_app_state(ctx.link(), || Msg::StateChanged),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let state = app_state();
        match msg {
            Msg::StateChanged => return true,
            Msg::SetVisited(id, visited) => state.write().set_visited(id, visited),
            Msg::Delete(id) => {
                let removed = state.write().remove_trips(|trip| trip.id == id);
                let Some(destination) = removed
                    .first()
                    .and_then(|trip| find_destination(trip.destination))
                else {
                    return false;
                };
                if let Some(snackbar) = ctx.link().snackbar_controller() {
                    let message = format!("Trip to {} deleted", destination.name);
                    show_with_action(&snackbar, message, "Undo", move || {
                        app_state().write().restore_trips(removed.clone())
                    });
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let shared = app_state();
        let state = shared.read();
        let (mut visited, mut upcoming): (Vec<&Trip>, Vec<&Trip>) =
            state.trips.iter().partition(|trip| trip.visited);
        upcoming.sort_by_key(|trip| trip.start);
        // the latest trip first
        visited.sort_by_key(|trip| std::cmp::Reverse(trip.start));

        let mut list = Column::new()
            .class(FlexFit)
            .style("overflow", "auto")
            .padding_bottom(4);

        if state.trips.is_empty() {
            list.add_child(
                Container::new()
                    .padding(4)
                    .class(TextAlign::Center)
                    .with_child("No trips yet. Use the + button to plan one."),
            );
        }

        for (title, trips) in [("Upcoming", upcoming), ("Visited", visited)] {
            if trips.is_empty() {
                continue;
            }
            list.add_child(
                Container::new()
                    .padding_x(2)
                    .padding_top(2)
                    .padding_bottom(1)
                    .class(FontStyle::TitleSmall)
                    .class("pwt-color-primary")
                    .with_child(title),
            );
            for trip in trips {
                list.add_child(trip_tile(ctx.link(), trip));
            }
        }

        list.add_child(
            Container::new()
                .padding(2)
                .class(FontStyle::BodySmall)
                .class(TextAlign::Center)
                .with_child("Swipe a trip sideways to mark it visited or to delete it."),
        );

        list.into()
    }
}

fn trip_tile(link: &Scope<TripListComp>, trip: &Trip) -> Html {
    let Some(destination) = find_destination(trip.destination) else {
        return Html::default();
    };

    let content = Row::new()
        .padding(2)
        .gap(2)
        .class(AlignItems::Center)
        .class("pwt-bg-color-surface")
        .with_child(
            Image::new(destination.picture(0))
                .class("travel-picture travel-thumb")
                .attribute("alt", destination.name),
        )
        .with_child(
            Column::new()
                .class("pwt-flex-fill")
                .gap(1)
                .with_child(
                    Container::new()
                        .class(FontStyle::TitleMedium)
                        .with_child(destination.name),
                )
                .with_child(
                    Container::new()
                        .class(FontStyle::BodySmall)
                        .with_child(format!(
                            "{} - {} days - {} {}",
                            format_date(&trip.start),
                            destination.days,
                            trip.travelers,
                            if trip.travelers == 1 {
                                "traveler"
                            } else {
                                "travelers"
                            },
                        )),
                ),
        )
        .with_optional_child(trip.visited.then(|| {
            Badge::new("Visited")
                .icon("check")
                .color_scheme(ColorScheme::SuccessContainer)
        }));

    let (id, visited) = (trip.id, trip.visited);

    let visited_action = SlidableAction::new(if visited { "Upcoming" } else { "Visited" })
        .icon_class(if visited { "fa fa-undo" } else { "fa fa-check" })
        .on_activate(
            link.callback(move |_: SlidableActionMouseEvent| Msg::SetVisited(id, !visited)),
        );

    // Slidables with actions do not report a dismiss, so the action removes the trip itself.
    let delete_action = SlidableAction::new("Delete")
        .icon_class("fa fa-trash")
        .on_activate(link.callback(move |_: SlidableActionMouseEvent| Msg::Delete(id)));

    Slidable::new(content)
        .key(trip.id.to_string())
        .left_actions(visited_action)
        .right_actions(delete_action)
        .into()
}
