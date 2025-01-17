use pwt::css::FlexFit;
use yew::prelude::*;

use pwt::prelude::*;

use pwt::state::NavigationContainer;
use pwt::widget::{Button, Column, Container, MiniScrollMode, TabBarItem, TabBarStyle, TabPanel};

const TAB_TITLE: &[(&str, &str)] = &[
    ("Flights", "fa fa-plane"),
    ("Trips", "fa fa-suitcase"),
    ("Explore", "fa fa-compass"),
    ("Map", "fa fa-map-o"),
    ("World", "fa fa-globe"),
    ("Community", "fa fa-users"),
];

#[function_component]
pub fn TabPanelDemo() -> Html {
    fn render_tab(text: &str) -> Html {
        Container::new().padding(2).with_child(text).into()
    }

    fn create_panel(style: TabBarStyle, router: bool) -> TabPanel {
        let mut panel = TabPanel::new()
            .class(FlexFit)
            .router(router)
            .tab_bar_style(style)
            .scroll_mode(MiniScrollMode::Native)
            .title("This is a simple Tab Panel")
            .tool(Button::new("?").class("circle").disabled(true))
            .tool(Button::new("!").class("circle").disabled(false));

        for (i, (title, icons)) in TAB_TITLE.iter().enumerate() {
            panel.add_item(
                TabBarItem::new()
                    .key(format!("tab{i}"))
                    .label(*title)
                    .icon_class(*icons)
                    .disabled(i == 4),
                render_tab(&format!("This is Tab {i}")),
            );
        }

        panel
    }

    NavigationContainer::new()
        .with_child(
            Column::new()
                .class("pwt-content-spacer")
                .class(FlexFit)
                .with_child(create_panel(TabBarStyle::Pills, true))
                .with_child(create_panel(TabBarStyle::MaterialPrimary, true))
                .with_child(create_panel(TabBarStyle::MaterialSecondary, true)),
        )
        .into()
}
