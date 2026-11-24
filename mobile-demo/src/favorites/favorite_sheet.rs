use yew_router::prelude::*;

use pwt::css::{AlignItems, FontStyle};
use pwt::touch::prelude::*;
use pwt::touch::MaterialAppScopeExt;
use pwt::widget::{Column, Container, Fa, List, ListTile};

use super::set_favorite;
use crate::data::find_destination;
use crate::gallery::GalleryPage;
use crate::route::Route;

/// Actions for a favorite, shown in a bottom sheet after a long press.
#[derive(Properties, PartialEq)]
pub struct FavoriteSheet {
    /// The id of the favorite destination.
    id: &'static str,
}

impl FavoriteSheet {
    pub fn new(id: &'static str) -> Self {
        Self { id }
    }
}

into_vnode!(FavoriteSheet => FavoriteSheetComp);

#[derive(Clone, Copy)]
enum Msg {
    Details,
    Pictures,
    Remove,
}

struct FavoriteSheetComp;

impl Component for FavoriteSheetComp {
    type Message = Msg;
    type Properties = FavoriteSheet;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();
        let Some(destination) = find_destination(ctx.props().id) else {
            return false;
        };
        let Some(page_controller) = link.page_controller() else {
            return false;
        };
        page_controller.close_side_dialog();
        match msg {
            Msg::Details => {
                if let Some(navigator) = link.navigator() {
                    navigator.push(&Route::FavoriteDestination {
                        id: destination.id.into(),
                    });
                }
            }
            Msg::Pictures => {
                page_controller.push_page(GalleryPage::new(destination.id));
            }
            Msg::Remove => {
                if let Some(snackbar) = link.snackbar_controller() {
                    set_favorite(&snackbar, destination, false);
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Some(destination) = find_destination(ctx.props().id) else {
            return Html::default();
        };

        let entry = |icon: &str, label: &str, msg: Msg| {
            ListTile::new()
                .interactive(true)
                .class(AlignItems::Center)
                .with_child(Fa::new(icon).fixed_width().large())
                .with_child(label.to_string())
                .on_activate(ctx.link().callback(move |_| msg))
        };

        let tiles = vec![
            entry("info-circle", "Show details", Msg::Details),
            entry("picture-o", "Show pictures", Msg::Pictures),
            entry("heart-o", "Remove from favorites", Msg::Remove),
        ];

        Column::new()
            .padding_bottom(2)
            .with_child(
                Container::new()
                    .padding(2)
                    .class(FontStyle::TitleMedium)
                    .with_child(destination.name),
            )
            .with_child(
                List::from_tiles(tiles)
                    .grid_template_columns("auto 1fr")
                    .column_gap(2),
            )
            .into()
    }
}
