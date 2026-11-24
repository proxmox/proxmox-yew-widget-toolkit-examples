use pwt::touch::prelude::*;
use pwt::touch::{FabMenu, FabMenuEntry, MaterialAppScopeExt, SnackBar};

use super::new_trip_page::NewTripPage;
use super::trip_form::{default_start, DEFAULT_TRAVELERS};
use crate::app_state;
use crate::data::random_destination;
use crate::messages::show_with_action;

/// A FAB menu to plan a trip, add a random one, or remove the visited trips.
#[derive(Properties, PartialEq, Default)]
pub struct TripsFab {}

impl TripsFab {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(TripsFab => TripsFabComp);

enum Msg {
    Plan,
    Surprise,
    ClearVisited,
}

struct TripsFabComp;

impl Component for TripsFabComp {
    type Message = Msg;
    type Properties = TripsFab;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();
        let state = app_state();
        match msg {
            Msg::Plan => {
                if let Some(page_controller) = link.page_controller() {
                    page_controller.push_page(NewTripPage::new());
                }
            }
            Msg::Surprise => {
                let destination = random_destination();
                state
                    .write()
                    .add_trip(destination.id, default_start(), DEFAULT_TRAVELERS);
                link.show_snackbar(
                    SnackBar::new().message(format!("Pack your bags for {}!", destination.name)),
                );
            }
            Msg::ClearVisited => {
                if !state.read().trips.iter().any(|trip| trip.visited) {
                    link.show_snackbar(SnackBar::new().message("There are no visited trips"));
                    return false;
                }
                let removed = state.write().remove_trips(|trip| trip.visited);
                if let Some(snackbar) = link.snackbar_controller() {
                    let message = match removed.len() {
                        1 => "1 visited trip removed".to_string(),
                        n => format!("{n} visited trips removed"),
                    };
                    show_with_action(&snackbar, message, "Undo", move || {
                        app_state().write().restore_trips(removed.clone())
                    });
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        FabMenu::new()
            .main_icon_class("fa fa-plus")
            .with_child(FabMenuEntry::new(
                "Plan a trip",
                "fa fa-calendar-plus-o",
                link.callback(|_| Msg::Plan),
            ))
            .with_child(FabMenuEntry::new(
                "Surprise trip",
                "fa fa-random",
                link.callback(|_| Msg::Surprise),
            ))
            .with_child(FabMenuEntry::new(
                "Clear visited",
                "fa fa-eraser",
                link.callback(|_| Msg::ClearVisited),
            ))
            .into()
    }
}
