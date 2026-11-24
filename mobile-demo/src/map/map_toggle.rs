use pwt::prelude::*;
use pwt::state::SharedStateObserver;
use pwt::widget::ActionIcon;

use crate::{app_state, observe_app_state, AppState};

/// App bar action to switch between the list and the map.
#[derive(Properties, PartialEq, Default)]
pub struct MapToggle {}

impl MapToggle {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(MapToggle => MapToggleComp);

enum Msg {
    StateChanged,
    Toggle,
}

struct MapToggleComp {
    _observer: SharedStateObserver<AppState>,
}

impl Component for MapToggleComp {
    type Message = Msg;
    type Properties = MapToggle;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _observer: observe_app_state(ctx.link(), || Msg::StateChanged),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged => true,
            Msg::Toggle => {
                let shared = app_state();
                let mut state = shared.write();
                state.show_map = !state.show_map;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let show_map = app_state().read().show_map;
        ActionIcon::new(if show_map {
            "fa fa-lg fa-list"
        } else {
            "fa fa-lg fa-map-o"
        })
        .aria_label(if show_map { "Show list" } else { "Show map" })
        .on_activate(ctx.link().callback(|_| Msg::Toggle))
        .into()
    }
}
