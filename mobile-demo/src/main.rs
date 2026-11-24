//! A travel explorer that shows off the touch widgets of the toolkit.

use std::collections::BTreeSet;

use yew::html::Scope;

use pwt::prelude::*;
use pwt::state::{LanguageInfo, SharedState, SharedStateObserver};
use pwt::touch::{MaterialApp, PageAnimationStyle};
use pwt::widget::form::PlainDate;

/// Components follow the builder pattern: their properties are the builder, created with `new()`,
/// and turn into a node like any widget.
macro_rules! into_vnode {
    ($props:ident => $comp:ident) => {
        impl From<$props> for yew::virtual_dom::VNode {
            fn from(props: $props) -> Self {
                yew::virtual_dom::VComp::new::<$comp>(std::rc::Rc::new(props), None).into()
            }
        }
    };
}

mod activatable;
mod data;
mod destination;
mod explore;
mod favorites;
mod gallery;
mod home;
mod map;
mod messages;
mod notice;
mod route;
mod settings;
mod trips;

use data::{Category, Destination};
use route::switch;

/// A planned or a visited trip.
#[derive(Clone, Debug, PartialEq)]
pub struct Trip {
    pub id: u32,
    /// The id of the destination.
    pub destination: &'static str,
    pub start: PlainDate,
    pub travelers: u32,
    pub visited: bool,
}

/// Which destinations the explore tab shows.
#[derive(Clone, Debug, PartialEq)]
pub struct Filter {
    /// The categories to show, all by default.
    pub categories: BTreeSet<Category>,
    /// The longest recommended stay to show, any by default.
    pub max_days: Option<u32>,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            categories: Category::ALL.iter().copied().collect(),
            max_days: None,
        }
    }
}

impl Filter {
    /// Whether the user changed the filter from the default, which shows all destinations.
    pub fn is_active(&self) -> bool {
        *self != Filter::default()
    }

    pub fn matches(&self, destination: &Destination) -> bool {
        self.categories.contains(&destination.category)
            && self.max_days.is_none_or(|max| destination.days <= max)
    }
}

/// App wide state. Pages change it directly and listen to it if they show it, see [app_state]
/// and [observe_app_state].
pub struct AppState {
    /// The ids of the favorite destinations.
    pub favorites: BTreeSet<&'static str>,
    /// The trips in no particular order, the trip list sorts them by date.
    pub trips: Vec<Trip>,
    pub filter: Filter,
    /// Show the destinations on a map instead of a list.
    pub show_map: bool,
    pub animation: AnimationSettings,
    next_trip_id: u32,
}

impl Default for AppState {
    fn default() -> Self {
        let today = PlainDate::today();
        Self {
            favorites: ["northern-lights", "island-lagoon"].into_iter().collect(),
            trips: vec![
                Trip {
                    id: 1,
                    destination: "old-town",
                    start: today.add_days(-60),
                    travelers: 2,
                    visited: true,
                },
                Trip {
                    id: 2,
                    destination: "alpine-lake",
                    start: today.add_days(30),
                    travelers: 4,
                    visited: false,
                },
            ],
            filter: Filter::default(),
            show_map: false,
            animation: AnimationSettings::default(),
            next_trip_id: 3,
        }
    }
}

impl AppState {
    pub fn set_favorite(&mut self, id: &'static str, favorite: bool) {
        if favorite {
            self.favorites.insert(id);
        } else {
            self.favorites.remove(id);
        }
    }

    pub fn add_trip(&mut self, destination: &'static str, start: PlainDate, travelers: u32) {
        self.trips.push(Trip {
            id: self.next_trip_id,
            destination,
            start,
            travelers,
            visited: false,
        });
        self.next_trip_id += 1;
    }

    /// Remove trips, see [AppState::restore_trips] to undo it.
    pub fn remove_trips(&mut self, filter: impl Fn(&Trip) -> bool) -> Vec<Trip> {
        let (removed, kept) = std::mem::take(&mut self.trips)
            .into_iter()
            .partition(|trip| filter(trip));
        self.trips = kept;
        removed
    }

    /// Put removed trips back, unless they are back already.
    pub fn restore_trips(&mut self, trips: Vec<Trip>) {
        for trip in trips {
            if self.trips.iter().all(|t| t.id != trip.id) {
                self.trips.push(trip);
            }
        }
    }

    pub fn set_visited(&mut self, id: u32, visited: bool) {
        if let Some(trip) = self.trips.iter_mut().find(|trip| trip.id == id) {
            trip.visited = visited;
        }
    }
}

/// Page animation settings, the [App] component applies them to the material app.
#[derive(Clone, Copy, PartialEq)]
pub struct AnimationSettings {
    pub style: Option<PageAnimationStyle>,
    pub slow_motion: bool,
}

impl Default for AnimationSettings {
    fn default() -> Self {
        Self {
            style: Some(PageAnimationStyle::FadeFromRight),
            slow_motion: false,
        }
    }
}

thread_local! {
    static APP_STATE: SharedState<AppState> = SharedState::new(AppState::default());
}

/// The app wide state, shared by all pages.
pub fn app_state() -> SharedState<AppState> {
    APP_STATE.with(SharedState::clone)
}

/// Send `msg` to a component on every change of the app state, as long as it keeps the observer.
pub fn observe_app_state<C: Component>(
    link: &Scope<C>,
    msg: impl Fn() -> C::Message + 'static,
) -> SharedStateObserver<AppState> {
    app_state().add_listener(link.callback(move |_| msg()))
}

/// Applies the animation settings to the material app.
struct App {
    animation: AnimationSettings,
    _observer: SharedStateObserver<AppState>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let animation = app_state().read().animation;
        Self {
            animation,
            _observer: observe_app_state(ctx.link(), || ()),
        }
    }

    // Only the animation settings concern the app itself, the pages listen to the rest.
    fn update(&mut self, _ctx: &Context<Self>, _msg: ()) -> bool {
        let animation = app_state().read().animation;
        if animation == self.animation {
            return false;
        }
        let class_list = gloo_utils::document_element().class_list();
        let _ = if animation.slow_motion {
            class_list.add_1("slow-motion")
        } else {
            class_list.remove_1("slow-motion")
        };
        self.animation = animation;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // keep snackbars above the bottom navigation bar
        MaterialApp::new(switch)
            .page_animation(self.animation.style)
            .snackbar_bottom_offset(88)
            .into()
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    pwt::state::set_available_languages(vec![LanguageInfo::new(
        "en",
        "English",
        gettext_noop("English"),
    )]);

    yew::Renderer::<App>::new().render();
}
