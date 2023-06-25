use pwt::touch::prelude::*;
use pwt::widget::nav::{Menu, MenuItem, NavigationDrawer};
use pwt::widget::{Button, Column};

use pwt::touch::{
    ApplicationBar, Fab, MaterialApp, NavigationBar, PageController, Scaffold, ScaffoldController,
    SnackBar, SnackBarController,
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
    #[at("/favorite")]
    Favorite,
    #[at("/profile")]
    Profile,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: &Route) -> Vec<Html> {
    match route {
        Route::Home | Route::HomeAlias => vec![html! {<Home/>}],
        Route::Settings => vec![html! {<Settings/>}],
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

fn create_drawer() -> NavigationDrawer {
    let menu = Menu::new()
        .with_item(
            MenuItem::new("Entry 1")
                .key("id1")
                .icon_class("fa fa-paper-plane"),
        )
        .with_item(
            MenuItem::new("Entry 2")
                .key("id2")
                .icon_class("fa fa-paper-plane"),
        )
        .with_item(
            MenuItem::new("Entry 3")
                .key("id3")
                .icon_class("fa fa-paper-plane"),
        );

    NavigationDrawer::new(menu)
        .class("pwt-flex-fill")
        .header(html! {<div class="pwt-p-2 pwt-font-size-title-large">{"Menu Header"}</div>})
}

#[function_component]
fn Home() -> Html {
    let snackbar_controller: Option<SnackBarController> = use_context();
    let page_controller: Option<PageController> = use_context();

    let scaffold_controller = use_state(|| ScaffoldController::new());

    let open_drawer_button = ActionIcon::new("fa fa-lg fa-bars").on_activate({
        let scaffold_controller = (*scaffold_controller).clone();
        move |_| {
            log::info!("OPENDRAWER0");
            scaffold_controller.toggle_drawer();
        }
    });

    let fab = Fab::new("fa fa-pencil");

    Scaffold::new()
        .controller((*scaffold_controller).clone())
        .application_bar(
            ApplicationBar::new()
                .leading(open_drawer_button)
                .title("Home"),
        )
        .favorite_action_button(fab)
        .drawer(create_drawer())
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
                        if let Some(controller) = &snackbar_controller {
                            controller.show_snackbar(
                                SnackBar::new().message("This is a test message (dismissive)"),
                            );
                        }
                    }
                }))
                .with_child(
                    Button::new("Show non-dismissive snackbar message").onclick({
                        let snackbar_controller = snackbar_controller.clone();
                        move |_| {
                            if let Some(controller) = &snackbar_controller {
                                controller.show_snackbar(
                                    SnackBar::new()
                                        .message("This is a test message (non-dismissive)")
                                        .action_label("Okay, got it"),
                                );
                            }
                        }
                    }),
                )
                .with_child(Button::new("Push a test page to the page stack").onclick({
                    let page_controller = page_controller.clone();
                    move |_| {
                        if let Some(controller) = &page_controller {
                            controller.push_page(html! {<TestPage1/>});
                        }
                    }
                })),
        )
        .navigation_bar(navigation_bar())
        .into()
}

#[function_component]
fn Settings() -> Html {
    Scaffold::new()
        .application_bar(ApplicationBar::new().title("Settings"))
        .body(Card::new().margin(2).with_child("Settings"))
        .navigation_bar(navigation_bar())
        .into()
}

#[function_component]
fn Favorite() -> Html {
    Scaffold::new()
        .application_bar(ApplicationBar::new().title("Favorite"))
        .body(Card::new().margin(2).with_child("Favorite"))
        .navigation_bar(navigation_bar())
        .into()
}

#[function_component]
fn Profile() -> Html {
    Scaffold::new()
        .application_bar(ApplicationBar::new().title("Profile"))
        .body(Card::new().margin(2).with_child("Profile"))
        .navigation_bar(navigation_bar())
        .into()
}

fn create_back_button(page_controller: Option<PageController>) -> ActionIcon {
    ActionIcon::new("fa fa-lg fa-arrow-left").on_activate({
        let page_controller = page_controller.clone();
        move |_| {
            if let Some(page_controller) = &page_controller {
                page_controller.last_page();
            }
        }
    })
}

#[function_component]
fn TestPage1() -> Html {
    let page_controller: Option<PageController> = use_context();

    let back_button = create_back_button(page_controller.clone());

    Scaffold::new()
        .application_bar(
            ApplicationBar::new()
                .title("Test page one")
                .leading(back_button),
        )
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
    let page_controller: Option<PageController> = use_context();
    let back_button = create_back_button(page_controller.clone());

    Scaffold::new()
        .application_bar(
            ApplicationBar::new()
                .title("Test page two")
                .leading(back_button),
        )
        .body(Card::new().margin(2).with_child("Test page two."))
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
