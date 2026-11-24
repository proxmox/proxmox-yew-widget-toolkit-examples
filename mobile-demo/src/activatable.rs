use pwt::prelude::*;
use pwt::widget::Container;

/// Let keyboard and screen reader users activate an element that reacts to pointer input, like a
/// picture that opens the gallery on a click.
///
/// The element gets the role of a button with the given label and a place in the tab order, and
/// reacts to Enter and Space. It keeps its own click or gesture handling.
pub fn activatable(
    container: Container,
    label: impl Into<AttrValue>,
    on_activate: Callback<()>,
) -> Container {
    container
        .attribute("role", "button")
        .attribute("tabindex", "0")
        .attribute("aria-label", label.into())
        .onkeydown(move |event: KeyboardEvent| {
            // event_key also copes with events that only look like keyboard events
            if matches!(pwt::dom::event_key(&event).as_str(), "Enter" | " ") {
                event.prevent_default();
                on_activate.emit(());
            }
        })
}
