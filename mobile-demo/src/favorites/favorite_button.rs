use pwt::state::SharedStateObserver;
use pwt::touch::prelude::*;
use pwt::widget::ActionIcon;

use super::set_favorite;
use crate::data::find_destination;
use crate::{app_state, observe_app_state, AppState};

/// Adds a destination to the favorites or removes it.
#[derive(Properties, PartialEq)]
pub struct FavoriteButton {
    /// The id of the destination.
    id: &'static str,
}

impl FavoriteButton {
    pub fn new(id: &'static str) -> Self {
        Self { id }
    }
}

into_vnode!(FavoriteButton => FavoriteButtonComp);

enum Msg {
    StateChanged,
    Toggle,
}

struct FavoriteButtonComp {
    _observer: SharedStateObserver<AppState>,
}

impl Component for FavoriteButtonComp {
    type Message = Msg;
    type Properties = FavoriteButton;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _observer: observe_app_state(ctx.link(), || Msg::StateChanged),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged => true,
            Msg::Toggle => {
                if let (Some(destination), Some(snackbar)) = (
                    find_destination(ctx.props().id),
                    ctx.link().snackbar_controller(),
                ) {
                    let favorite = app_state().read().favorites.contains(destination.id);
                    set_favorite(&snackbar, destination, !favorite);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let favorite = app_state().read().favorites.contains(ctx.props().id);

        ActionIcon::new(if favorite {
            "fa fa-lg fa-heart"
        } else {
            "fa fa-lg fa-heart-o"
        })
        .class(favorite.then_some("pwt-color-error"))
        .aria_label(if favorite {
            "Remove from favorites"
        } else {
            "Add to favorites"
        })
        .on_activate(ctx.link().callback(|_| Msg::Toggle))
        .into()
    }
}
