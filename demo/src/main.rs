mod align;
use align::AlignDemo;

mod buttons;
use buttons::ButtonsDemo;

mod canvas;
use canvas::CanvasDemo;

mod cards;
pub use cards::CardsDemo;

mod gestures;
pub use gestures::GestureDetectorDemo;

mod list;
pub use list::ListDemo;

mod theme;
use pwt::widget::Column;
use pwt::widget::Toolbar;
use theme::ThemeDemo;

mod lorem_ipsum;
pub use lorem_ipsum::LoremIpsum;

mod miscellaneous;
use miscellaneous::MiscellaneousDemo;

mod tab_panel;
use tab_panel::TabPanelDemo;

mod toolbar;
pub use toolbar::ToolbarDemo;

mod tooltip;
pub use tooltip::TooltipDemo;

mod menu;
pub use menu::MenuDemo;

mod datatable;
pub use datatable::DataTableDemo;

mod i18n;
pub use i18n::I18NDemo;

mod seven_guis;
pub use seven_guis::SevenGuisDemo;

mod split_pane;
pub use split_pane::*;

mod tree;
pub use tree::TreeDemo;

mod movie;
pub use movie::MovieDemo;

mod navigation_drawer;
pub use navigation_drawer::NavigationDrawerDemo;

mod navigation_rail;
pub use navigation_rail::NavigationRailDemo;

mod navigation_bar;
pub use navigation_bar::NavigationBarDemo;

mod application_bar;
pub use application_bar::ApplicationBarDemo;

mod dialog;
pub use dialog::DialogDemo;

mod fields;
pub use fields::FieldsDemo;

mod examples;
pub use examples::ExampleWidgets;

use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::HashRouter;

use pwt::css::*;
use pwt::prelude::*;

use pwt::state::{use_selection, LanguageInfo, NavigationContainer, Selection};
use pwt::widget::nav::{Menu, MenuItem, NavigationDrawer};
use pwt::widget::{
    CatalogLoader, Pane, RtlSwitcher, SelectionView, SelectionViewRenderInfo, SplitPane,
    ThemeLoader, ThemeModeSelector, ThemeNameSelector,
};

fn register_view(
    menu: &mut Menu,
    view: &mut SelectionView,
    text: &'static str,
    id: &'static str,
    renderer: impl 'static + Fn(&SelectionViewRenderInfo) -> Html,
) {
    view.add_builder(id, renderer);
    menu.add_item(MenuItem::new(text).key(id));
}

fn register_submenu(
    menu: &mut Menu,
    view: &mut SelectionView,
    text: &'static str,
    id: &'static str,
    renderer: impl 'static + Fn(&SelectionViewRenderInfo) -> Html,
    submenu: Menu,
) {
    view.add_builder(id, renderer);
    menu.add_item(MenuItem::new(text).key(id).submenu(submenu));
}

#[function_component]
fn MainMenu() -> Html {
    let selection = use_selection(Selection::new);

    let mut content = SelectionView::new()
        .selection(selection.clone())
        .class(Flex::Fill)
        .class(Overflow::Auto);

    let mut menu = Menu::new();

    menu.add_section_header("Elements");

    register_view(
        &mut menu,
        &mut content,
        "Buttons",
        "button_panel",
        |_| html! {<ButtonsDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Cards",
        "cards_panel",
        |_| html! {<CardsDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Canvas",
        "canvas_panel",
        |_| html! {<CanvasDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "DataTable",
        "datatable_panel",
        |_| html! {<DataTableDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Dialogs",
        "dialog_panel",
        |_| html! {<DialogDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Fields",
        "forms_panel",
        |_| html! {<FieldsDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "List",
        "list_panel",
        |_| html! {<ListDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Menus",
        "menu_panel",
        |_| html! {<MenuDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Miscellaneous",
        "miscellaneous_panel",
        |_| html! {<MiscellaneousDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Toolbar",
        "toolbar_panel",
        |_| html! {<ToolbarDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Tooltips",
        "tooltip_panel",
        |_| html! {<TooltipDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Tree",
        "tree_panel",
        |_| html! {<TreeDemo/>},
    );

    menu.add_spacer();
    menu.add_section_header("Container/Layout");

    register_view(
        &mut menu,
        &mut content,
        "Navigation Drawer",
        "nav_drawer_panel",
        |_| html! {<NavigationDrawerDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Tab Panel",
        "tab_panel",
        |_| html! {<TabPanelDemo/>},
    );

    let mut submenu = Menu::new();

    register_view(
        &mut submenu,
        &mut content,
        "Horizontal",
        "split_pane_panel_horizontal",
        |_| html! {<SplitPaneDemoHorizontal/>},
    );

    register_view(
        &mut submenu,
        &mut content,
        "Vertical",
        "split_pane_panel_vertical",
        |_| html! {<SplitPaneDemoVertical/>},
    );

    register_view(
        &mut submenu,
        &mut content,
        "Default Size",
        "split_pane_panel_sizing",
        |_| html! {<SplitPaneDemoSizing/>},
    );

    register_view(
        &mut submenu,
        &mut content,
        "Min/Max Size",
        "split_pane_panel_min_max",
        |_| html! {<SplitPaneDemoMinMax/>},
    );

    register_submenu(
        &mut menu,
        &mut content,
        "SplitPane",
        "split_pane_panel",
        |_| html! {<SplitPaneDemoNested/>},
        submenu,
    );

    menu.add_spacer();
    menu.add_section_header("Mobile");

    register_view(
        &mut menu,
        &mut content,
        "Gestures",
        "gesture_panel",
        |_| html! {<GestureDetectorDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Navigation Rail",
        "nav_rail_panel",
        |_| html! {<NavigationRailDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Navigation Bar",
        "nav_bar_panel",
        |_| html! {<NavigationBarDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Application Bar",
        "app_bar_panel",
        |_| html! {<ApplicationBarDemo/>},
    );

    menu.add_spacer();
    menu.add_section_header("Utilities");

    register_view(
        &mut menu,
        &mut content,
        "Alignment",
        "align_panel",
        |_| html! {<AlignDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Internationalization",
        "i18n_panel",
        |_| html! {<I18NDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Theme",
        "theme_panel",
        |_| html! {<ThemeDemo/>},
    );

    menu.add_spacer();
    menu.add_section_header("Complex Examples");

    register_view(
        &mut menu,
        &mut content,
        "7GUIs",
        "7guis_panel",
        |_| html! {<SevenGuisDemo/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Example Widgets",
        "example_widgets",
        |_| html! {<ExampleWidgets/>},
    );

    register_view(
        &mut menu,
        &mut content,
        "Movies",
        "movies_panel",
        |_| html! {<MovieDemo/>},
    );

    let drawer = NavigationDrawer::new(menu)
        .class(Overflow::Auto)
        .selection(selection)
        .aria_label("Examples")
        .router(true);

    NavigationContainer::new()
        .with_child(
            SplitPane::new()
                .class(Flex::Fill)
                .class(Overflow::Auto)
                .with_child(Pane::new(drawer).size(None))
                .with_child(Pane::new(content).flex(1)),
        )
        .into()
}

#[function_component]
fn DemoApp() -> Html {
    let header = Toolbar::new()
        .attribute("role", "banner")
        .attribute("aria-label", "Demo")
        .class("pwt-bg-color-neutral-alt")
        .class("pwt-color-on-neutral-alt")
        .border_bottom(true)
        .with_child(html! {<h3>{"Proxmox Yew Widget Toolkit Demo Application"}</h3>})
        .with_flex_spacer()
        .with_child(html! {{"RTL"}})
        .with_child(RtlSwitcher::new())
        .with_child(ThemeNameSelector::new())
        .with_child(ThemeModeSelector::new());

    Column::new()
        .class(Viewport)
        .with_child(header)
        .with_child(html! {<MainMenu/>})
        .into()
}

#[function_component]
fn RoutedApp() -> Html {
    let body = html! {<DemoApp/>};
    let body = CatalogLoader::new(body);
    let body = ThemeLoader::new(body);

    html! { <HashRouter>{body}</HashRouter> }
}

fn set_body(content: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let body: HtmlElement = document.create_element("body").unwrap().dyn_into().unwrap();
    body.set_inner_html(content);
    document.set_body(Some(&body));
}

fn panic_hook() -> Box<dyn Fn(&std::panic::PanicHookInfo) + 'static + Sync + Send> {
    Box::new(|info: &std::panic::PanicHookInfo<'_>| {
        let msg = format!("Application panicked: {info}");
        web_sys::console::error_1(&msg.into());

        set_body(&format!(
            r#"
<h1 class="panicked__title">Application panicked!</h1>
<p>Reason: {info}</p>
"#
        ));
    })
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::set_custom_panic_hook(panic_hook());

    static THEMES: &[&str] = &["Crisp", "Desktop", "Material"];
    pwt::state::set_available_themes(THEMES);

    pwt::state::set_available_languages(vec![
        LanguageInfo::new("de", "Deutsch", gettext_noop("German")),
        LanguageInfo::new("en", "English", gettext_noop("English")),
    ]);

    yew::Renderer::<RoutedApp>::new().render();
}
