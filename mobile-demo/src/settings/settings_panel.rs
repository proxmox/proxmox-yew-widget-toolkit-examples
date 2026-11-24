use std::rc::Rc;

use yew_router::prelude::*;

use pwt::css::{AlignItems, FlexFit, FontStyle};
use pwt::state::SharedStateObserver;
use pwt::touch::prelude::*;
use pwt::touch::{PageAnimationStyle, SnackBar};
use pwt::widget::form::{Checkbox, Combobox};
use pwt::widget::{Button, Card, Column, Container, Row, ThemeDensitySelector, ThemeModeSelector};

use crate::route::Route;
use crate::{app_state, observe_app_state, AppState};

/// The settings tab: theme, page animations, and demo messages.
#[derive(Properties, PartialEq, Default)]
pub struct SettingsPanel {}

impl SettingsPanel {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(SettingsPanel => SettingsPanelComp);

const PAGE_ANIMATIONS: &[(&str, Option<PageAnimationStyle>)] = &[
    ("Fade from right", Some(PageAnimationStyle::FadeFromRight)),
    ("Parallax", Some(PageAnimationStyle::Parallax)),
    ("Push", Some(PageAnimationStyle::Push)),
    ("Fade", Some(PageAnimationStyle::Fade)),
    ("Fade from bottom", Some(PageAnimationStyle::FadeFromBottom)),
    ("Cover", Some(PageAnimationStyle::Cover)),
    ("Zoom", Some(PageAnimationStyle::Zoom)),
    ("Flip", Some(PageAnimationStyle::Flip)),
    ("None", None),
];

fn section(title: &str) -> Card {
    // a card is a plain block, it needs to be a flex column for the gap
    Card::new()
        .class("pwt-d-flex pwt-flex-direction-column pwt-gap-2")
        .padding(2)
        .with_child(
            Container::new()
                .class(FontStyle::TitleMedium)
                .class("pwt-color-primary")
                .with_child(title.to_string()),
        )
}

fn setting(label: &str, field: impl Into<Html>) -> Row {
    Row::new()
        .gap(2)
        .class(AlignItems::Center)
        .with_child(
            Container::new()
                .class("pwt-flex-fill")
                .with_child(label.to_string()),
        )
        .with_child(field)
}

enum Msg {
    StateChanged,
    Animation(String),
    SlowMotion(bool),
    Message,
    MessageWithAction,
    About,
}

struct SettingsPanelComp {
    _observer: SharedStateObserver<AppState>,
}

impl Component for SettingsPanelComp {
    type Message = Msg;
    type Properties = SettingsPanel;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _observer: observe_app_state(ctx.link(), || Msg::StateChanged),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();
        match msg {
            Msg::StateChanged => return true,
            Msg::Animation(label) => {
                if let Some((_, style)) = PAGE_ANIMATIONS.iter().find(|(l, _)| *l == label) {
                    app_state().write().animation.style = *style;
                }
            }
            Msg::SlowMotion(slow_motion) => app_state().write().animation.slow_motion = slow_motion,
            Msg::Message => {
                link.show_snackbar(SnackBar::new().message("Have a nice trip!"));
            }
            Msg::MessageWithAction => {
                link.show_snackbar(
                    SnackBar::new()
                        .message("Your passport expires in two months")
                        .action_label("Got it"),
                );
            }
            Msg::About => {
                if let Some(navigator) = link.navigator() {
                    navigator.push(&Route::About);
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let animation = app_state().read().animation;

        let current = PAGE_ANIMATIONS
            .iter()
            .find(|(_, style)| *style == animation.style)
            .map(|(label, _)| *label);

        let animation_selector = Combobox::new()
            .items(Rc::new(
                PAGE_ANIMATIONS
                    .iter()
                    .map(|(label, _)| AttrValue::from(*label))
                    .collect(),
            ))
            .required(true)
            .aria_label("Animation")
            .value(current)
            .on_change(link.callback(Msg::Animation));

        let slow_motion = Checkbox::new()
            .switch(true)
            .aria_label("Slow motion")
            .checked(animation.slow_motion)
            .on_change(link.callback(Msg::SlowMotion));

        let appearance = section("Appearance")
            .with_child(setting("Theme", ThemeModeSelector::new()))
            .with_child(setting("Density", ThemeDensitySelector::new()));

        let transitions = section("Page transitions")
            .with_child(setting("Animation", animation_selector))
            .with_child(setting("Slow motion", slow_motion))
            .with_child(
                Container::from_tag("p")
                    .class(FontStyle::BodySmall)
                    .with_child(
                    "Slow motion makes it easy to follow the pictures when they fly from the list \
                 into the destination page and on into the gallery.",
                ),
            );

        let messages = section("Messages").with_child(
            Row::new()
                .gap(2)
                .class("pwt-flex-wrap")
                .with_child(Button::new("Show a message").onclick(link.callback(|_| Msg::Message)))
                .with_child(
                    Button::new("Show a message with an action")
                        .onclick(link.callback(|_| Msg::MessageWithAction)),
                ),
        );

        let about = section("About").with_child(
            Row::new().with_child(
                Button::new("About this app")
                    .icon_class("fa fa-info-circle")
                    .onclick(link.callback(|_| Msg::About)),
            ),
        );

        Column::new()
            .class(FlexFit)
            .style("overflow", "auto")
            .with_child(
                Column::new()
                    .class("travel-content")
                    .padding(2)
                    .gap(2)
                    .with_child(appearance)
                    .with_child(transitions)
                    .with_child(messages)
                    .with_child(about),
            )
            .into()
    }
}
