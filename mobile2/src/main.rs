use pwt::touch::prelude::*;
use pwt::widget::{Button, Column, Container};

use pwt::touch::{
    ApplicationBar, Fab, MaterialApp, PageController, Scaffold, SnackBar, SnackBarController,
};
use pwt::widget::{ActionIcon, Card};

use yew_router::prelude::*;

mod settings;
use settings::{Settings, SubSettings};

mod navigation_bar;
pub use navigation_bar::navigation_bar;

mod drawer;
pub use drawer::create_drawer;

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

fn switch(path: &str) -> Vec<Html> {
    let route = Route::recognize(&path).unwrap_or_default();
    match route {
        Route::Home | Route::HomeAlias => vec![html! {<Home/>}],
        Route::Settings => vec![html! {<Settings/>}],
        Route::SubSettings => vec![
            html! {<Settings/>},
            // fixme: we cannot use Switch here, because that render NotFound during page stack transitions
            // html! { <Switch<SubSettings> render={switch_subsettings} />},
            SubSettings::switch(SubSettings::recognize(&path).unwrap_or_default()),
        ],
        Route::Favorite => vec![html! {<Favorite/>}],
        Route::Profile => vec![html! {<Profile/>}],
        Route::NotFound => vec![html! {<NotFound/>}],
    }
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
pub fn NotFound() -> Html {
    Scaffold::new()
        .application_bar(ApplicationBar::new().title("Page not found!"))
        .body(Container::new().padding(2).with_child(html! {
            <p>
                {"Sorry, we are unable to find this page. Please continue "}
                <Link<Route> to={Route::Home}>{"here"}</Link<Route>>
            </p>
        }))
        //.navigation_bar(navigation_bar())
        .into()
}

#[function_component]
fn App() -> Html {
    MaterialApp::new(switch).snackbar_bottom_offset(220).into()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
