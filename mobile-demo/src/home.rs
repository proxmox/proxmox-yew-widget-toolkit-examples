use yew::context::ContextHandle;
use yew::virtual_dom::VNode;
use yew_router::prelude::*;

use pwt::css::{AlignItems, FontStyle};
use pwt::prelude::*;
use pwt::state::NavigationContext;
use pwt::touch::{AdaptiveScaffold, ApplicationBar, MaterialAppScopeExt, PageController};
use pwt::widget::nav::{Menu, MenuItem, NavigationDrawer};
use pwt::widget::{ActionIcon, Container, Fa, Row, TabBarItem, ThemeModeSelector};

use crate::explore::{ExploreList, FilterButton, SurpriseFab};
use crate::favorites::FavoriteGrid;
use crate::map::MapToggle;
use crate::route::Route;
use crate::settings::SettingsPanel;
use crate::trips::{TripList, TripsFab};

/// The page below all others. It shows the tab of the current route, also while a detail page
/// is on top of it.
#[derive(Properties, PartialEq, Default)]
pub struct HomePage {}

impl HomePage {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(HomePage => HomePageComp);

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Explore,
    Favorites,
    Trips,
    Settings,
}

impl Tab {
    const ALL: &'static [Tab] = &[Tab::Explore, Tab::Favorites, Tab::Trips, Tab::Settings];

    /// The first path segment of the tab's route.
    fn key(&self) -> &'static str {
        match self {
            Tab::Explore => "explore",
            Tab::Favorites => "favorites",
            Tab::Trips => "trips",
            Tab::Settings => "settings",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Tab::Explore => "Explore",
            Tab::Favorites => "Favorites",
            Tab::Trips => "Trips",
            Tab::Settings => "Settings",
        }
    }

    fn icon(&self) -> (&'static str, &'static str) {
        match self {
            Tab::Explore => ("fa fa-compass", "fa fa-compass"),
            Tab::Favorites => ("fa fa-heart-o", "fa fa-heart"),
            Tab::Trips => ("fa fa-suitcase", "fa fa-suitcase"),
            Tab::Settings => ("fa fa-sliders", "fa fa-sliders"),
        }
    }

    fn route(&self) -> Route {
        match self {
            Tab::Explore => Route::Explore,
            Tab::Favorites => Route::Favorites,
            Tab::Trips => Route::Trips,
            Tab::Settings => Route::Settings,
        }
    }

    fn from_path(path: &str) -> Tab {
        Tab::ALL
            .iter()
            .copied()
            .find(|tab| tab.key() == path)
            .unwrap_or(Tab::Explore)
    }
}

fn tab_items() -> Vec<TabBarItem> {
    Tab::ALL
        .iter()
        .map(|tab| {
            let (icon, active_icon) = tab.icon();
            TabBarItem::new()
                .key(tab.key())
                .icon_class(icon)
                .active_icon_class(active_icon)
                .label(tab.label())
        })
        .collect()
}

enum Msg {
    NavigationChanged(NavigationContext),
    OpenDrawer,
}

struct HomePageComp {
    tab: Tab,
    _navigation: Option<ContextHandle<NavigationContext>>,
}

impl Component for HomePageComp {
    type Message = Msg;
    type Properties = HomePage;

    fn create(ctx: &Context<Self>) -> Self {
        let (tab, navigation) = match ctx
            .link()
            .context(ctx.link().callback(Msg::NavigationChanged))
        {
            Some((nav_ctx, handle)) => (Tab::from_path(&nav_ctx.path()), Some(handle)),
            None => (Tab::Explore, None),
        };
        Self {
            tab,
            _navigation: navigation,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NavigationChanged(nav_ctx) => {
                let tab = Tab::from_path(&nav_ctx.path());
                let changed = tab != self.tab;
                self.tab = tab;
                changed
            }
            Msg::OpenDrawer => {
                let link = ctx.link();
                if let (Some(page_controller), Some(navigator)) =
                    (link.page_controller(), link.navigator())
                {
                    page_controller.show_drawer(drawer(&page_controller, &navigator, self.tab));
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let menu_button = ActionIcon::new("fa fa-lg fa-bars")
            .aria_label("Menu")
            .on_activate(ctx.link().callback(|_| Msg::OpenDrawer));

        let (body, actions, fab): (Html, Vec<VNode>, Option<Html>) = match self.tab {
            Tab::Explore => (
                ExploreList::new().into(),
                vec![MapToggle::new().into(), FilterButton::new().into()],
                Some(SurpriseFab::new().into()),
            ),
            Tab::Favorites => (FavoriteGrid::new().into(), Vec::new(), None),
            Tab::Trips => (
                TripList::new().into(),
                Vec::new(),
                Some(TripsFab::new().into()),
            ),
            Tab::Settings => (SettingsPanel::new().into(), Vec::new(), None),
        };

        let app_bar = ApplicationBar::new()
            .leading(menu_button)
            .title(self.tab.label())
            .actions(actions);

        let logo = Fa::new("globe")
            .class("pwt-color-primary")
            .large_2x()
            .padding_y(2);

        let mut scaffold = AdaptiveScaffold::new(tab_items())
            .router(true)
            .default_active(Tab::Explore.key())
            .rail_leading(logo)
            .application_bar(app_bar)
            .body(body);
        if let Some(fab) = fab {
            scaffold = scaffold.favorite_action_button(fab);
        }
        scaffold.into()
    }
}

/// The app closes the drawer on every route change, so its entries only navigate.
fn drawer(
    page_controller: &PageController,
    navigator: &Navigator,
    active: Tab,
) -> NavigationDrawer {
    let mut menu = Menu::new();
    for tab in Tab::ALL {
        let navigator = navigator.clone();
        let route = tab.route();
        menu.add_item(
            MenuItem::new(tab.label())
                .key(tab.key())
                .icon_class(tab.icon().0)
                .on_activate(move |_| navigator.push(&route)),
        );
    }
    menu.add_item(
        MenuItem::new("About")
            .key("about")
            .icon_class("fa fa-info-circle")
            .on_activate({
                let navigator = navigator.clone();
                move |_| navigator.push(&Route::About)
            }),
    );
    menu.add_item(
        MenuItem::new("Close")
            .key("close")
            .icon_class("fa fa-times")
            .on_activate({
                let page_controller = page_controller.clone();
                move |_| page_controller.close_side_dialog()
            }),
    );

    let header = Row::new()
        .padding(2)
        .gap(2)
        .class(AlignItems::Center)
        .with_child(Fa::new("globe").class("pwt-color-primary").large())
        .with_child(
            Container::new()
                .class(FontStyle::TitleLarge)
                .class("pwt-flex-fill")
                .with_child("Travel Explorer"),
        )
        .with_child(ThemeModeSelector::new());

    NavigationDrawer::new(menu)
        .class("pwt-flex-fill")
        .header(header)
        .default_active(active.key())
}
