//! The routes of the app and the pages they show.

use yew_router::prelude::*;

use pwt::css::AlignItems;
use pwt::prelude::*;
use pwt::touch::Scaffold;
use pwt::widget::{Button, Column, Container};

use crate::destination::DestinationPage;
use crate::home::HomePage;
use crate::settings::AboutPage;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/explore")]
    Explore,

    #[at("/explore/:id")]
    ExploreDestination { id: String },

    #[at("/favorites")]
    Favorites,

    #[at("/favorites/:id")]
    FavoriteDestination { id: String },

    #[at("/trips")]
    Trips,

    #[at("/settings")]
    Settings,

    #[at("/settings/about")]
    About,

    #[not_found]
    #[at("/404")]
    NotFound,
}

// All tabs share one home page, so switching tabs does not animate the whole page, and the
// home page stays below the detail pages as the source of their hero animations.
pub fn switch(path: &str) -> Vec<Html> {
    match Route::recognize(path).unwrap_or(Route::NotFound) {
        Route::Home | Route::Explore | Route::Favorites | Route::Trips | Route::Settings => {
            vec![HomePage::new().into()]
        }
        Route::ExploreDestination { id } | Route::FavoriteDestination { id } => {
            vec![HomePage::new().into(), DestinationPage::new(id).into()]
        }
        Route::About => vec![HomePage::new().into(), AboutPage::new().into()],
        Route::NotFound => vec![NotFound::new().into()],
    }
}

/// Shown for an unknown route, offers the way back to the destinations.
#[derive(Properties, PartialEq, Default)]
struct NotFound {}

impl NotFound {
    fn new() -> Self {
        Self {}
    }
}

into_vnode!(NotFound => NotFoundComp);

struct NotFoundComp;

impl Component for NotFoundComp {
    /// Go back to the destinations.
    type Message = ();
    type Properties = NotFound;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: ()) -> bool {
        if let Some(navigator) = ctx.link().navigator() {
            navigator.push(&Route::Explore);
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        Scaffold::with_title("Page not found")
            .body(
                Column::new()
                    .padding(2)
                    .gap(2)
                    .class(AlignItems::Start)
                    .with_child(
                        Container::from_tag("p").with_child("There is nothing to explore here."),
                    )
                    .with_child(
                        Button::new("Back to the destinations")
                            .onclick(ctx.link().callback(|_| ())),
                    ),
            )
            .into()
    }
}
