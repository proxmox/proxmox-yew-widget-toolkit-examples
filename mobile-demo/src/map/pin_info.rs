use yew_router::prelude::*;

use pwt::css::{AlignItems, FontStyle};
use pwt::prelude::*;
use pwt::widget::{Column, Container, Fa, List, ListTile};

use crate::data::find_destination;
use crate::explore::{picture_hero, title_hero};
use crate::route::Route;

/// The info box of a pin, close pins share one.
#[derive(Properties, PartialEq)]
pub struct PinInfo {
    /// The ids of the destinations at the pin.
    ids: Vec<&'static str>,
}

impl PinInfo {
    pub fn new(ids: Vec<&'static str>) -> Self {
        Self { ids }
    }
}

into_vnode!(PinInfo => PinInfoComp);

struct PinInfoComp;

impl Component for PinInfoComp {
    /// Open a destination.
    type Message = &'static str;
    type Properties = PinInfo;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, id: &'static str) -> bool {
        if let Some(navigator) = ctx.link().navigator() {
            navigator.push(&Route::ExploreDestination { id: id.into() });
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tiles = ctx
            .props()
            .ids
            .iter()
            .filter_map(|id| find_destination(id))
            .map(|destination| {
                let id = destination.id;
                ListTile::new()
                    .interactive(true)
                    .class(AlignItems::Center)
                    // the picture and the name fly into the destination page, as from the list
                    .with_child(picture_hero(destination, "travel-pin"))
                    .with_child(
                        Column::new()
                            .with_child(title_hero(destination, FontStyle::TitleSmall))
                            .with_child(
                                Container::new()
                                    .class(FontStyle::BodySmall)
                                    .with_child(destination.place()),
                            ),
                    )
                    .with_child(Fa::new("chevron-right"))
                    .on_activate(ctx.link().callback(move |_| id))
            })
            .collect();

        List::from_tiles(tiles)
            .grid_template_columns("auto 1fr auto")
            .column_gap(2)
            .min_width(260)
            .into()
    }
}
