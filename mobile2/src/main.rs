use pwt::touch::prelude::*;
use pwt::widget::nav::{Menu, MenuItem, NavigationDrawer};
use pwt::widget::{Button, Column, Row, ThemeModeSelector};

use pwt::touch::{
    ApplicationBar, Fab, MaterialApp, NavigationBar, PageController, Scaffold, SnackBar,
    SnackBarController,
};
use pwt::widget::{ActionIcon, Card, TabBarItem};

use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,

    #[at("/_")]
    HomeAlias,

    #[at("/settings")]
    Settings,

    #[at("/settings/*")]
    SubSettings,

    #[at("/favorite")]
    Favorite,

    #[at("/profile")]
    Profile,

    #[not_found]
    #[at("/404")]
    NotFound,
}
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum SubSettings {
    #[at("/settings/network")]
    Network,

    #[at("/settings/account")]
    Account,

    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

fn switch_subsettings(route: SubSettings) -> Html {
    match route {
        SubSettings::Account => html! {<AccountSettings/>},
        SubSettings::Network => html! {<NetworkSettings/>},
        SubSettings::NotFound => html! {<NotFound/>},
    }
}

fn switch(route: &Route) -> Vec<Html> {
    log::info!("SWITCH TO {:?}", route);
    match route {
        Route::Home | Route::HomeAlias => vec![html! {<Home/>}],
        Route::Settings => vec![html! {<Settings/>}],
        Route::SubSettings => vec![
            html! {<Settings/>},
            // fixme: we cannot use Switch here, because that render NotFound during page stack transitions
            html! { <Switch<SubSettings> render={switch_subsettings} />},
        ],
        Route::Favorite => vec![html! {<Favorite/>}],
        Route::Profile => vec![html! {<Home/>}, html! {<Profile/>}],
        Route::NotFound => vec![html! {<NotFound/>}],
    }
}

fn navigation_bar() -> NavigationBar {
    let items = vec![
        TabBarItem::new()
            .key("")
            .icon_class("fa fa-home")
            .label("Home"),
        TabBarItem::new()
            .key("settings")
            .icon_class("fa fa-gear")
            .label("Settings"),
        TabBarItem::new()
            .key("favorite")
            .icon_class("fa fa-star")
            .label("Favorite"),
        TabBarItem::new()
            .key("profile")
            .icon_class("fa fa-user-o")
            .active_icon_class("fa fa-user")
            .label("Profile"),
    ];

    NavigationBar::new(items).router(true)
}

fn create_drawer(page_controller: PageController, navigator: Navigator) -> NavigationDrawer {
    let menu = Menu::new()
        .with_item(
            MenuItem::new("Home")
                .key("id1")
                .icon_class("fa fa-paper-plane")
                .on_activate({
                    let navigator = navigator.clone();
                    move |_| navigator.push(&Route::Home)
                }),
        )
        .with_item(
            MenuItem::new("Profile")
                .key("id2")
                .icon_class("fa fa-paper-plane")
                .on_activate({
                    let navigator = navigator.clone();
                    move |_| {
                        navigator.push(&Route::Profile);
                    }
                }),
        )
        .with_item(
            MenuItem::new("Close Drawer")
                .key("id3")
                .icon_class("fa fa-paper-plane")
                .on_activate({
                    let page_controller = page_controller.clone();
                    move |_| {
                        page_controller.close_side_dialog();
                    }
                }),
        );

    let header = Row::new()
        .class("pwt-align-items-center")
        .with_child(html! {<div class="pwt-p-2 pwt-font-size-title-large">{"Menu Header"}</div>})
        .with_child(ThemeModeSelector::new());

    NavigationDrawer::new(menu)
        .class("pwt-flex-fill")
        .header(header)
}

#[function_component]
fn Home() -> Html {
    let snackbar_controller: SnackBarController = use_context().unwrap();
    let page_controller: PageController = use_context().unwrap();
    let navigator = use_navigator().unwrap();

    let open_drawer_button = ActionIcon::new("fa fa-lg fa-bars").on_activate({
        let page_controller = page_controller.clone();
        let navigator = navigator.clone();
        move |_| {
            log::info!("OPENDRAWER0");
            page_controller.show_drawer(create_drawer(page_controller.clone(), navigator.clone()));
        }
    });

    let fab = Fab::new("fa fa-pencil");

    Scaffold::new()
        .application_bar(
            ApplicationBar::new()
                .leading(open_drawer_button)
                .title("Home"),
        )
        .favorite_action_button(fab)
        .body(
            Column::new()
                .padding(2)
                .gap(2)
                .with_child(
                    Card::new()
                        .with_child(html! {<h6>{"This is the home page"}</h6>})
                        .with_child(html! {<hr/>})
                        .with_child(
                            html! {<p>{"The body of this card contains nothing usefull."}</p>},
                        ),
                )
                .with_child(Button::new("Show dismissive snackbar message").onclick({
                    let snackbar_controller = snackbar_controller.clone();
                    move |_| {
                        snackbar_controller.show_snackbar(
                            SnackBar::new().message("This is a test message (dismissive)"),
                        );
                    }
                }))
                .with_child(
                    Button::new("Show non-dismissive snackbar message").onclick({
                        let snackbar_controller = snackbar_controller.clone();
                        move |_| {
                            snackbar_controller.show_snackbar(
                                SnackBar::new()
                                    .message("This is a test message (non-dismissive)")
                                    .action_label("Okay, got it"),
                            );
                        }
                    }),
                )
                .with_child(Button::new("Push a test page to the page stack").onclick({
                    let page_controller = page_controller.clone();
                    move |_| {
                        page_controller.push_page(html! {<TestPage1/>});
                    }
                })),
        )
        .navigation_bar(navigation_bar())
        .into()
}

#[function_component]
fn Settings() -> Html {
    fn list_tile(titel: &str, body: &str) -> Card {
        Card::new()
            .class("pwt-interactive")
            .class("pwt-shape-none")
            .class("pwt-card-flat")
            .class("pwt-scheme-neutral")
            .class("pwt-border-bottom")
            .with_child(html! {
                <>
                    <h6>{titel}</h6>
                    <p>{body}</p>
                </>
            })
    }

    let navigator = use_navigator().unwrap();

    let body = Column::new()
        .with_child(
            Card::new()
                .class("pwt-shape-none")
                .with_child(html! {<h6>{"Settings"}</h6>}),
        )
        .with_child(
            list_tile(
                "Account",
                "Opens a new page to configure the account settings.",
            )
            .onclick({
                let navigator = navigator.clone();
                move |_| navigator.push(&SubSettings::Account)
            }),
        )
        .with_child(
            list_tile("Network", "Opens a new page to configure the network.").onclick({
                let navigator = navigator.clone();
                move |_| navigator.push(&SubSettings::Network)
            }),
        );

    Scaffold::with_title("Settings")
        .body(body)
        .navigation_bar(navigation_bar())
        .into()
}

#[function_component]
fn Favorite() -> Html {
    Scaffold::with_title("Favorite")
        .body(Card::new().margin(2).with_child("Favorite"))
        .navigation_bar(navigation_bar())
        .into()
}

#[function_component]
fn Profile() -> Html {
    Scaffold::with_title("Profile")
        .body(Card::new().margin(2).with_child("Profile"))
        .navigation_bar(navigation_bar())
        .into()
}

#[function_component]
fn TestPage1() -> Html {
    let page_controller: Option<PageController> = use_context();

    Scaffold::with_title("Test page one")
        .body(
            Column::new()
                .padding(2)
                .gap(2)
                .with_child(Card::new().with_child("Test page one."))
                .with_child(Button::new("Push a test page to the page stack").onclick({
                    let page_controller = page_controller.clone();
                    move |_| {
                        if let Some(controller) = &page_controller {
                            controller.push_page(html! {<TestPage2/>});
                        }
                    }
                })),
        )
        .into()
}

#[function_component]
fn TestPage2() -> Html {
    Scaffold::with_title("Test page two")
        .body(Card::new().margin(2).with_child("Test page two."))
        .into()
}


#[function_component]
fn AccountSettings() -> Html {
    Scaffold::with_title("Account settings")
        .body(Card::new().margin(2).with_child("Account Settings"))
        .into()
}

#[function_component]
fn NetworkSettings() -> Html {
    Scaffold::with_title("Network settings")
        .body(Card::new().margin(2).with_child("Network Settings"))
        .into()
}

#[function_component]
fn NotFound() -> Html {
    Scaffold::new()
        .application_bar(ApplicationBar::new().title("Page not found!"))
        .body(html! {
            <div class="pwt-p-2">
                <p>{
                    "Sorry, we are unable to fing this page. Please continue "
                }<Link<Route> to={Route::Home}>{"here"}</Link<Route>></p>
            </div>
        })
        .navigation_bar(navigation_bar())
        .into()
}
#[function_component]
fn App() -> Html {
    MaterialApp::<Route>::new(switch)
        .snackbar_bottom_offset(220)
        .into()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
