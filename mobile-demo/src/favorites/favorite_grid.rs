use yew_router::prelude::*;

use pwt::css::{FlexFit, FontStyle, TextAlign};
use pwt::state::SharedStateObserver;
use pwt::touch::prelude::*;
use pwt::touch::{GestureDetector, MaterialAppScopeExt};
use pwt::widget::{ActionIcon, Button, Card, Column, Container, Fa};

use super::favorite_sheet::FavoriteSheet;
use crate::activatable::activatable;
use crate::data::find_destination;
use crate::explore::{picture_hero, title_hero};
use crate::notice::notice;
use crate::route::Route;
use crate::{app_state, observe_app_state, AppState};

/// The favorite destinations as cards with pictures.
#[derive(Properties, PartialEq, Default)]
pub struct FavoriteGrid {}

impl FavoriteGrid {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(FavoriteGrid => FavoriteGridComp);

enum Msg {
    StateChanged,
    Open(&'static str),
    ShowActions(&'static str),
    Explore,
}

struct FavoriteGridComp {
    _observer: SharedStateObserver<AppState>,
}

impl Component for FavoriteGridComp {
    type Message = Msg;
    type Properties = FavoriteGrid;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _observer: observe_app_state(ctx.link(), || Msg::StateChanged),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();
        match msg {
            Msg::StateChanged => return true,
            Msg::Open(id) => {
                if let Some(navigator) = link.navigator() {
                    navigator.push(&Route::FavoriteDestination { id: id.into() });
                }
            }
            Msg::ShowActions(id) => {
                if let Some(page_controller) = link.page_controller() {
                    page_controller.show_modal_bottom_sheet(FavoriteSheet::new(id));
                }
            }
            Msg::Explore => {
                if let Some(navigator) = link.navigator() {
                    navigator.push(&Route::Explore);
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let shared = app_state();
        let state = shared.read();

        if state.favorites.is_empty() {
            return notice(
                Fa::new("heart-o"),
                "Tap the heart of a destination to keep it here.",
            )
            .with_child(
                Button::new("Explore destinations").onclick(link.callback(|_| Msg::Explore)),
            )
            .into();
        }

        let mut grid = Container::new()
            .padding(2)
            .style("display", "grid")
            .style(
                "grid-template-columns",
                "repeat(auto-fill, minmax(160px, 1fr))",
            )
            .style("gap", "var(--pwt-spacer-2)");

        for destination in state.favorites.iter().filter_map(|id| find_destination(id)) {
            let id = destination.id;

            let content = Container::new()
                // keep vertical scrolling, the browser cancels a long press when it scrolls
                .style("touch-action", "pan-y")
                .style("user-select", "none")
                .with_child(picture_hero(destination, "travel-wide"))
                .with_child(
                    Column::new()
                        .padding(2)
                        .gap(1)
                        .with_child(title_hero(destination, FontStyle::TitleSmall))
                        .with_child(
                            Container::new()
                                .class(FontStyle::BodySmall)
                                .with_child(destination.place()),
                        ),
                );
            let content = activatable(
                content,
                format!("Show {}", destination.name),
                link.callback(move |_| Msg::Open(id)),
            );

            // Next to the gesture detector, not inside it, so a tap on it does not open the
            // destination as well.
            let actions = ActionIcon::new("fa fa-lg fa-ellipsis-v")
                .class("travel-card-actions")
                .aria_label(format!("More actions for {}", destination.name))
                .on_activate(link.callback(move |_| Msg::ShowActions(id)));

            grid.add_child(
                Card::new()
                    .class("pwt-interactive")
                    .padding(0)
                    .style("overflow", "hidden")
                    .style("position", "relative")
                    .with_child(
                        GestureDetector::new(content)
                            .on_tap(link.callback(move |_| Msg::Open(id)))
                            .on_long_press(link.callback(move |_| Msg::ShowActions(id))),
                    )
                    .with_child(actions),
            );
        }

        Column::new()
            .class(FlexFit)
            .style("overflow", "auto")
            .with_child(grid)
            .with_child(
                Container::new()
                    .padding(2)
                    .class(FontStyle::BodySmall)
                    .class(TextAlign::Center)
                    .with_child("Long press a picture for more actions."),
            )
            .into()
    }
}
