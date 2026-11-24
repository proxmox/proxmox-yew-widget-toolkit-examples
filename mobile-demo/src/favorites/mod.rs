//! The favorites tab and the favorite button of the destinations.

mod favorite_button;
mod favorite_grid;
mod favorite_sheet;

pub use favorite_button::FavoriteButton;
pub use favorite_grid::FavoriteGrid;

use pwt::touch::SnackBarController;

use crate::app_state;
use crate::data::Destination;
use crate::messages::show_with_action;

/// Change a favorite and offer to undo it.
pub fn set_favorite(
    snackbar: &SnackBarController,
    destination: &'static Destination,
    favorite: bool,
) {
    app_state().write().set_favorite(destination.id, favorite);
    let message = if favorite {
        format!("{} added to favorites", destination.name)
    } else {
        format!("{} removed from favorites", destination.name)
    };
    show_with_action(snackbar, message, "Undo", move || {
        app_state().write().set_favorite(destination.id, !favorite)
    });
}
