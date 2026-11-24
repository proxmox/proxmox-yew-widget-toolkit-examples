use yew_router::prelude::*;

use pwt::prelude::*;
use pwt::touch::Fab;

use crate::data::random_destination;
use crate::route::Route;

/// Opens a random destination.
#[derive(Properties, PartialEq, Default)]
pub struct SurpriseFab {}

impl SurpriseFab {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(SurpriseFab => SurpriseFabComp);

struct SurpriseFabComp;

impl Component for SurpriseFabComp {
    type Message = ();
    type Properties = SurpriseFab;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: ()) -> bool {
        if let Some(navigator) = ctx.link().navigator() {
            navigator.push(&Route::ExploreDestination {
                id: random_destination().id.into(),
            });
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        Fab::new("fa fa-random")
            .text("Surprise me")
            .on_activate(ctx.link().callback(|_| ()))
            .into()
    }
}
