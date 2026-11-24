use yew_router::prelude::*;

use pwt::css::{AlignItems, FlexFit, FontStyle};
use pwt::prelude::*;
use pwt::state::SharedStateObserver;
use pwt::widget::{Button, Column, Container, Fa, List, ListTile};

use super::{destination_badges, picture_hero, title_hero};
use crate::data::{Destination, DESTINATIONS};
use crate::favorites::FavoriteButton;
use crate::notice::notice;
use crate::route::Route;
use crate::{app_state, observe_app_state, AppState, Filter};

/// The destinations that match the filter.
#[derive(Properties, PartialEq, Default)]
pub struct ExploreList {}

impl ExploreList {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(ExploreList => ExploreListComp);

enum Msg {
    StateChanged,
    Open(&'static str),
    ResetFilter,
}

struct ExploreListComp {
    _observer: SharedStateObserver<AppState>,
}

impl Component for ExploreListComp {
    type Message = Msg;
    type Properties = ExploreList;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _observer: observe_app_state(ctx.link(), || Msg::StateChanged),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged => true,
            Msg::Open(id) => {
                if let Some(navigator) = ctx.link().navigator() {
                    navigator.push(&Route::ExploreDestination { id: id.into() });
                }
                false
            }
            Msg::ResetFilter => {
                app_state().write().filter = Filter::default();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let filter = app_state().read().filter.clone();
        let destinations: Vec<&'static Destination> =
            DESTINATIONS.iter().filter(|d| filter.matches(d)).collect();

        if destinations.is_empty() {
            return notice(Fa::new("filter"), "No destination matches your filters.")
                .with_child(
                    Button::new("Reset filters").onclick(ctx.link().callback(|_| Msg::ResetFilter)),
                )
                .into();
        }

        let link = ctx.link().clone();
        List::new(destinations.len() as u64, move |pos| {
            let destination = destinations[pos as usize];
            ListTile::new()
                .interactive(true)
                .class(AlignItems::Center)
                .with_child(picture_hero(destination, "travel-thumb"))
                .with_child(
                    Column::new()
                        .gap(1)
                        .with_child(title_hero(destination, FontStyle::TitleMedium))
                        .with_child(
                            Container::new()
                                .class(FontStyle::BodySmall)
                                .with_child(destination.place()),
                        )
                        .with_child(destination_badges(destination)),
                )
                .with_child(FavoriteButton::new(destination.id))
                .on_activate(link.callback(move |_| Msg::Open(destination.id)))
        })
        .grid_template_columns("auto 1fr auto")
        .column_gap(2)
        .class(FlexFit)
        .into()
    }
}
