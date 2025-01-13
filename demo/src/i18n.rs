use pwt::prelude::*;
use pwt::widget::{Button, Container, LanguageSelector, Panel};

use pwt::ngettext;

#[function_component]
pub fn I18NDemo() -> Html {
    // TRANSLATORS: This is a comment
    log::info!("{}", tr!("This is a test!"));
    let state = use_state(|| 0);

    let button_callback = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    Panel::new()
        .class("pwt-flex-fit")
        .border(false)
        .title(pgettext!(
            "This is a panel title",
            "Internationalization with gettext"
        ))
        .with_tool(LanguageSelector::new())
        .with_child(
            Container::new()
                .margin(2)
                .with_child(Button::new(tr!("Add Message")).onclick(button_callback)),
        )
        .with_child(Container::new().margin(2).with_child(ngettext!(
            "You have one new message",
            "You have {n} new messages",
            *state
        )))
        .with_child(
            Container::new()
                .margin(2)
                .with_child(tr!("Static number: {}", "5")),
        )
        .into()
}
