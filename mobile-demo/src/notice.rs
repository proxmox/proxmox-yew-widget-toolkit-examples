use pwt::css::{AlignItems, FlexFit, JustifyContent};
use pwt::prelude::*;
use pwt::widget::{Column, Fa};

/// A large icon with a short text in the middle of a page, for example for an empty list.
///
/// Add a button with the way out, like resetting a filter, as further child.
pub fn notice(icon: Fa, text: impl Into<Html>) -> Column {
    Column::new()
        .class(FlexFit)
        .class(AlignItems::Center)
        .class(JustifyContent::Center)
        .gap(2)
        .padding(4)
        .with_child(icon.large_3x())
        .with_child(text)
}
