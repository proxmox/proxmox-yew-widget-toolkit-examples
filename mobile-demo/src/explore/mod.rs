//! The explore tab: the destinations as a list, and the hero helpers shared with the
//! other pages that show destinations.

mod explore_list;
mod filter_button;
mod filter_panel;
mod surprise_fab;

pub use explore_list::ExploreList;
pub use filter_button::FilterButton;
pub use surprise_fab::SurpriseFab;

use pwt::css::{ColorScheme, FontStyle};
use pwt::prelude::*;
use pwt::touch::Hero;
use pwt::widget::{Badge, Image, Row};

use crate::data::Destination;

/// The hero tag of the main picture of a destination.
pub fn picture_tag(destination: &Destination) -> String {
    format!("picture-{}", destination.id)
}

/// The hero tag of the name of a destination.
pub fn title_tag(destination: &Destination) -> String {
    format!("title-{}", destination.id)
}

/// The picture that flies into the header of the destination page.
pub fn picture_hero(destination: &Destination, class: &'static str) -> Hero {
    Hero::new(picture_tag(destination)).with_child(
        Image::new(destination.picture(0))
            .class("travel-picture")
            .class(class)
            .attribute("alt", destination.name),
    )
}

/// The name that flies into the destination page.
pub fn title_hero(destination: &Destination, font: FontStyle) -> Hero {
    // Only as wide as the text, so both ends of the flight have about the same aspect ratio and
    // the text scales evenly instead of being stretched to the width of the container.
    Hero::new(title_tag(destination))
        .class(font)
        .style("width", "fit-content")
        .with_child(destination.name)
}

/// The category and the recommended stay of a destination.
pub fn destination_badges(destination: &Destination) -> Row {
    Row::new()
        .gap(1)
        .class("pwt-flex-wrap")
        .with_child(
            Badge::new(destination.category.label())
                .icon(destination.category.icon())
                .color_scheme(ColorScheme::SecondaryContainer),
        )
        .with_child(
            Badge::new(format!("{} days", destination.days))
                .icon("clock-o")
                .color_scheme(ColorScheme::TertiaryContainer),
        )
}
