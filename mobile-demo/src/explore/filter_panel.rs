use pwt::css::{ColorScheme, FontStyle, JustifyContent};
use pwt::prelude::*;
use pwt::state::SharedStateObserver;
use pwt::touch::MaterialAppScopeExt;
use pwt::widget::form::Checkbox;
use pwt::widget::{Button, Column, Container, Row, SegmentedButton};

use crate::data::Category;
use crate::{app_state, observe_app_state, AppState, Filter};

/// Shown in the end drawer, applies every change right away.
#[derive(Properties, PartialEq, Default)]
pub struct FilterPanel {}

impl FilterPanel {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(FilterPanel => FilterPanelComp);

enum Msg {
    StateChanged,
    Category(Category, bool),
    MaxDays(Option<u32>),
    Reset,
    Done,
}

struct FilterPanelComp {
    _observer: SharedStateObserver<AppState>,
}

impl Component for FilterPanelComp {
    type Message = Msg;
    type Properties = FilterPanel;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _observer: observe_app_state(ctx.link(), || Msg::StateChanged),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged => return true,
            Msg::Category(category, true) => {
                app_state().write().filter.categories.insert(category);
            }
            Msg::Category(category, false) => {
                app_state().write().filter.categories.remove(&category);
            }
            Msg::MaxDays(max_days) => app_state().write().filter.max_days = max_days,
            Msg::Reset => app_state().write().filter = Filter::default(),
            Msg::Done => {
                if let Some(page_controller) = ctx.link().page_controller() {
                    page_controller.close_side_dialog();
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let shared = app_state();
        let state = shared.read();
        let filter = &state.filter;

        let mut categories = Column::new().gap(1);
        for category in Category::ALL.iter().copied() {
            categories.add_child(
                Checkbox::new()
                    .switch(true)
                    .box_label(category.label())
                    .checked(filter.categories.contains(&category))
                    .on_change(link.callback(move |checked| Msg::Category(category, checked))),
            );
        }

        let mut days = SegmentedButton::new().aria_label("Recommended stay");
        for (label, max_days) in [
            ("Any", None),
            ("Up to 3 days", Some(3)),
            ("Up to 5 days", Some(5)),
        ] {
            days.add_button(
                Button::new(label)
                    .pressed(filter.max_days == max_days)
                    .onclick(link.callback(move |_| Msg::MaxDays(max_days))),
            );
        }

        let heading = |text: &'static str| {
            Container::new()
                .class(FontStyle::TitleSmall)
                .with_child(text)
        };

        Column::new()
            .padding(2)
            .gap(2)
            .min_width(260)
            .with_child(
                Container::new()
                    .class(FontStyle::TitleLarge)
                    .with_child("Filters"),
            )
            .with_child(heading("Category"))
            .with_child(categories)
            .with_child(heading("Recommended stay"))
            .with_child(days)
            .with_flex_spacer()
            .with_child(
                Row::new()
                    .gap(2)
                    .class(JustifyContent::End)
                    .with_child(
                        Button::new("Reset")
                            .disabled(!filter.is_active())
                            .onclick(link.callback(|_| Msg::Reset)),
                    )
                    .with_child(
                        Button::new("Done")
                            .class(ColorScheme::Primary)
                            .onclick(link.callback(|_| Msg::Done)),
                    ),
            )
            .into()
    }
}
