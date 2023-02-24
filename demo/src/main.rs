mod align;
use align::AlignDemo;

mod buttons;
use buttons::ButtonsDemo;

mod canvas;
use canvas::CanvasDemo;

mod gestures;
pub use gestures::GestureDetectorDemo;

mod theme;
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

mod seven_guis;
pub use seven_guis::SevenGuisDemo;

mod split_pane;
pub use split_pane::*;

mod tree;
pub use tree::TreeDemo;

mod movie;
pub use movie::MovieDemo;

mod navigation;
pub use navigation::NavigationDemo;

mod dialog;
pub use dialog::DialogDemo;

mod fields;
pub use fields::FieldsDemo;

use yew::prelude::*;
use yew_router::HashRouter;

use pwt::prelude::*;

use pwt::widget::nav_menu::{MenuItem, NavigationMenu};
use pwt::widget::{Panel, ThemeLoader, ThemeModeSelector, ThemeNameSelector, RtlSwitcher};

#[function_component]
fn MainMenu() -> Html {
    NavigationMenu::new()
        .aria_label("Examples")
        .with_item(MenuItem::new(
            "gesture_panel",
            "Gestures",
            None::<&str>,
            |_| {
                html! {<GestureDetectorDemo/>}
            },
        ))
        .with_item(MenuItem::new(
            "canvas_panel",
            "Canvas",
            None::<&str>,
            |_| {
                html! {<CanvasDemo/>}
            },
        ))
        .with_item(MenuItem::new(
            "7guis_panel",
            "7GUIs",
            None::<&str>,
            |_| {
                html! {<SevenGuisDemo/>}
            },
        ))
        .with_item(MenuItem::new(
            "toolbar_panel",
            "Toolbar",
            None::<&str>,
            |_| {
                html! {<ToolbarDemo/>}
            },
        ))
        .with_item(MenuItem::new(
            "tooltip_panel",
            "Tooltips",
            None::<&str>,
            |_| {
                html! {<TooltipDemo/>}
            },
        ))
        .with_item(MenuItem::new(
            "tab_panel",
            "Tab Panel",
            None::<&str>,
            |_| {
                html! {<TabPanelDemo/>}
            },
        ))
        .with_item(MenuItem::new("forms_panel", "Fields", None::<&str>, |_| {
            html! {<FieldsDemo />}
        }))
        .with_item(MenuItem::new(
            "button_panel",
            "Buttons",
            None::<&str>,
            |_| {
                html! {<ButtonsDemo/>}
            },
        ))
        .with_item(MenuItem::new(
            "datatable_panel",
            "DataTable",
            None::<&str>,
            |_| {
                html! {<DataTableDemo/>}
            },
        ))
        .with_item(MenuItem::new(
            "tree_panel",
            "Tree",
            None::<&str>,
            |_| {
                html! {<TreeDemo/>}
            },
        ))
        .with_item(MenuItem::new(
            "miscellaneous_panel",
            "Miscellaneous",
            None::<&str>,
            |_| {
                html! {<MiscellaneousDemo/>}
            }
        ))
        .with_item(MenuItem::new(
            "movies_panel",
            "Movies",
            None::<&str>,
            |_| {
                html! {<MovieDemo/>}
            },
        ))
        .with_item(MenuItem::new("menu_panel", "Menus", None::<&str>, |_| {
            html! {<MenuDemo/>}
        }))
        .with_item(MenuItem::new(
            "nav_panel",
            "Navigation",
            None::<&str>,
            |_| html!(<NavigationDemo/>),
        ))
        .with_item(MenuItem::new(
            "dialog_panel",
            "Dialogs",
            None::<&str>,
            |_| html!(<DialogDemo/>),
        ))
        .with_item(MenuItem::new(
            "align_panel",
            "Alignment",
            None::<&str>,
            |_| html!(<AlignDemo/>),
        ))
        .with_item(MenuItem::new(
            "theme_panel",
            "Theme",
            None::<&str>,
            |_| html!(<ThemeDemo/>),
        ))
        .with_item(
            MenuItem::new(
                "split_pane_panel",
                "SplitPane",
                None::<&str>,
                |_| html!(<SplitPaneDemoNested/>),
            )
           .submenu()
                .with_item(
                    MenuItem::new(
                        "split_pane_panel_horizontal",
                        "Horizontal",
                        None::<&str>,
                        |_| html!(<SplitPaneDemoHorizontal/>),
                    )
                )
                .with_item(
                    MenuItem::new(
                        "split_pane_panel_vertical",
                        "Vertical",
                        None::<&str>,
                        |_| html!(<SplitPaneDemoVertical/>),
                    )
                )
                .with_item(
                    MenuItem::new(
                        "split_pane_panel_sizing",
                        "Default Size",
                        None::<&str>,
                        |_| html!(<SplitPaneDemoSizing/>),
                    )
                )
                 .with_item(
                    MenuItem::new(
                        "split_pane_panel_min_max",
                        "Min/Max Size",
                        None::<&str>,
                        |_| html!(<SplitPaneDemoMinMax/>),
                    )
                )
        )
        .navigation_container()
        .into()
}

#[function_component]
fn DemoApp() -> Html {
    let panel = Panel::new()
    //.class("pwt-viewport")
    //.class("pwt-viewport pwt-scheme-primary")
        .class("pwt-viewport pwt-scheme-neutral-alt")
    //.class("pwt-viewport pwt-scheme-secondary")
        .border(false)
        .attribute("role", "banner")
        .attribute("aria-label", "Demo")
        .title("Proxmox Yew Widget Toolkit Demo Application")
        .with_tool(html! {{"RTL"}})
        .with_tool(RtlSwitcher::new())
        .with_tool(ThemeNameSelector::new())
        .with_tool(ThemeModeSelector::new())
        .with_child(html!{<MainMenu/>});

    ThemeLoader::new(panel).into()
}

#[function_component]
fn RoutedApp() -> Html {
    html!{ <HashRouter><DemoApp/></HashRouter> }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<RoutedApp>::new().render();
}
