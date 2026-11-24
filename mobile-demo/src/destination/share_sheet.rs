use pwt::css::{AlignItems, FontStyle};
use pwt::touch::prelude::*;
use pwt::touch::{MaterialAppScopeExt, SnackBar};
use pwt::widget::{Column, Container, Fa, List, ListTile};

use crate::data::find_destination;

/// Bottom sheet with ways to share a destination.
#[derive(Properties, PartialEq)]
pub struct ShareSheet {
    /// The id of the destination to share.
    id: &'static str,
}

impl ShareSheet {
    pub fn new(id: &'static str) -> Self {
        Self { id }
    }
}

into_vnode!(ShareSheet => ShareSheetComp);

struct ShareSheetComp;

impl Component for ShareSheetComp {
    /// The message to show when done.
    type Message = &'static str;
    type Properties = ShareSheet;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, done: &'static str) -> bool {
        if let Some(page_controller) = ctx.link().page_controller() {
            page_controller.close_side_dialog();
        }
        ctx.link().show_snackbar(SnackBar::new().message(done));
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Some(destination) = find_destination(ctx.props().id) else {
            return Html::default();
        };

        let tiles = [
            ("link", "Copy link", "Link copied"),
            ("envelope-o", "Email", "Email draft created"),
            ("comment-o", "Message", "Message sent"),
            ("qrcode", "QR code", "QR code saved"),
        ]
        .into_iter()
        .map(|(icon, label, done)| {
            ListTile::new()
                .interactive(true)
                .class(AlignItems::Center)
                .with_child(Fa::new(icon).fixed_width().large())
                .with_child(label)
                .on_activate(ctx.link().callback(move |_| done))
        })
        .collect();

        Column::new()
            .padding_bottom(2)
            .with_child(
                Container::new()
                    .padding(2)
                    .class(FontStyle::TitleMedium)
                    .with_child(format!("Share {}", destination.name)),
            )
            .with_child(
                List::from_tiles(tiles)
                    .grid_template_columns("auto 1fr")
                    .column_gap(2),
            )
            .into()
    }
}
