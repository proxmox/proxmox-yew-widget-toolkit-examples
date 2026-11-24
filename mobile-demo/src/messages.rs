use pwt::prelude::*;
use pwt::touch::{SnackBar, SnackBarController};

/// How long a message with an action stays, in milliseconds.
const ACTION_TIMEOUT: u32 = 6000;

/// Show a message with an action, like "Undo", in place of all earlier messages.
///
/// A snackbar with an action stays until the user reacts, and it holds back all later
/// messages, so close it after a while. Replacing the earlier messages shows it right away, so it
/// stays visible for the whole time.
pub fn show_with_action(
    controller: &SnackBarController,
    message: impl Into<AttrValue>,
    action_label: &'static str,
    on_action: impl Fn() + 'static,
) {
    controller.dismiss_all();
    let id = controller.show_snackbar(
        SnackBar::new()
            .message(message.into())
            .action_label(action_label)
            .on_action(move |_| on_action()),
    );
    let controller = controller.clone();
    gloo_timers::callback::Timeout::new(ACTION_TIMEOUT, move || controller.dismiss(id)).forget();
}
