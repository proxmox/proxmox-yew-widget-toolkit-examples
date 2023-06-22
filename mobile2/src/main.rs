use pwt::touch::prelude::*;
use pwt::widget::nav::{Menu, MenuItem};
use pwt::widget::nav::NavigationDrawer;

use pwt::touch::{ApplicationBar, Fab, NavigationBar, Scaffold, ScaffoldController, SnackBar, SnackBarController, MaterialApp};
use pwt::widget::{ActionIcon, Container, TabBarItem};

pub enum Msg {}

pub struct App {
    scaffold_controller: ScaffoldController,
    snackbar_controller: SnackBarController,

    snackbar_counter: usize,
}

impl App {
    fn create_drawer(&self) -> Html {
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

        let drawer = NavigationDrawer::new(menu)
            .class("pwt-flex-fill")
            .header(html! {<div class="pwt-p-2 pwt-font-size-title-large">{"Menu Header"}</div>});

        drawer.into()
    }

    fn add_snackbar(&mut self) {
        self.snackbar_counter += 1;
        self.snackbar_controller.show_snackbar(
            SnackBar::new()
                .message(format!("This is snackbar message #{}", self.snackbar_counter))
                .action_label("Press me")
                .show_close_icon(true)
        );
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let scaffold_controller = ScaffoldController::new();
        let snackbar_controller = SnackBarController::new();

        let mut me = Self {
            scaffold_controller,
            snackbar_controller,
            snackbar_counter: 0,
        };

        me.add_snackbar();
        me.add_snackbar();

        me
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let page0 = Container::new()
            //.attribute("style", "background: red;")
            .class("pwt-flex-fill pwt-overflow-auto")
            .with_child("PAGE0xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

        let app_bar = ApplicationBar::new()
            .leading(ActionIcon::new("fa fa-lg fa-arrow-left"))
            .title("APP BARRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR")
            .actions(vec![
                ActionIcon::new("fa fa-lg fa-bell")
                    .on_activate({
                        let controller = self.snackbar_controller.clone();
                        move |_| {
                            controller.show_snackbar(SnackBar::new().message("You pressed the notification button."));
                        }
                    })
                    .into(),
                ActionIcon::new("fa fa-lg fa-refresh").into(),
                ActionIcon::new("fa fa-lg fa-bars")
                    .on_activate({
                        let controller = self.scaffold_controller.clone();
                        move |_| {
                            controller.toggle_drawer();
                            log::info!("CLICK");
                        }
                    })
                    .into(),
            ])
            .bottom("TEST");

        let items = vec![
            TabBarItem::new()
                .key("delete")
                .icon_class("fa fa-trash-o")
                .active_icon_class("fa fa-trash")
                .label("Delete"),
            TabBarItem::new()
                .key("settings")
                .icon_class("fa fa-gear")
                .label("Settings"),
            TabBarItem::new()
                .key("users")
                .icon_class("fa fa-user-o")
                .active_icon_class("fa fa-user")
                .label("Users"),
        ];

        let navigation_bar = NavigationBar::new(items).router(true);

        let fab = Fab::new("fa fa-pencil").large();

        let scaffold = Scaffold::new()
            .controller(self.scaffold_controller.clone())
            .application_bar(app_bar)
            .body(page0)
            .navigation_bar(navigation_bar)
            .favorite_action_button(fab)
            .drawer(self.create_drawer());

        MaterialApp::new()
            .snackbar_controller(self.snackbar_controller.clone())
            .snackbar_bottom_offset(220)
            .home(scaffold)
            .into()
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
