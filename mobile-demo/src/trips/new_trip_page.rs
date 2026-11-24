use pwt::touch::prelude::*;
use pwt::touch::{MaterialAppScopeExt, Scaffold};
use pwt::widget::Column;

use super::trip_form::TripForm;

/// A page without a route, pushed by the FAB menu. It is dropped on the next route change.
#[derive(Properties, PartialEq, Default)]
pub struct NewTripPage {}

impl NewTripPage {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(NewTripPage => NewTripPageComp);

struct NewTripPageComp;

impl Component for NewTripPageComp {
    type Message = ();
    type Properties = NewTripPage;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: ()) -> bool {
        if let Some(page_controller) = ctx.link().page_controller() {
            page_controller.pop_page();
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_done = ctx.link().callback(|_| ());
        Scaffold::with_title("Plan a trip")
            .body(
                Column::new()
                    .class("travel-content")
                    .padding(2)
                    .with_child(TripForm::new(on_done)),
            )
            .into()
    }
}
