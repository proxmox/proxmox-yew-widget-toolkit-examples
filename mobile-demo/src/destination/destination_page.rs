use std::rc::Rc;

use yew::virtual_dom::{Key, VComp, VNode};

use pwt::css::{AlignItems, ColorScheme, FlexFit, FontStyle};
use pwt::touch::prelude::*;
use pwt::touch::{AdaptiveDialog, ApplicationBar, Hero, MaterialAppScopeExt, Scaffold};
use pwt::widget::{ActionIcon, Button, Column, Container, Fa, Image, List, ListTile, Row};

use super::share_sheet::ShareSheet;
use crate::activatable::activatable;
use crate::data::{find_destination, Destination};
use crate::explore::{destination_badges, picture_hero, title_hero};
use crate::favorites::FavoriteButton;
use crate::gallery::{gallery_tag, GalleryPage};
use crate::trips::TripForm;

/// The details of a destination with its pictures, the target of the hero animations from the
/// lists.
#[derive(Properties, PartialEq)]
pub struct DestinationPage {
    /// The id of the destination, taken from the route.
    id: AttrValue,
}

impl DestinationPage {
    pub fn new(id: impl Into<AttrValue>) -> Self {
        Self { id: id.into() }
    }
}

// The page takes the destination from its properties, so the page stack must not reuse it for
// another destination.
impl From<DestinationPage> for VNode {
    fn from(props: DestinationPage) -> Self {
        let key = Key::from(props.id.to_string());
        VComp::new::<DestinationPageComp>(Rc::new(props), Some(key)).into()
    }
}

enum Msg {
    OpenGallery(usize),
    Share,
    ShowPlan(bool),
}

struct DestinationPageComp {
    show_plan: bool,
}

impl Component for DestinationPageComp {
    type Message = Msg;
    type Properties = DestinationPage;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { show_plan: false }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Some(destination) = find_destination(&ctx.props().id) else {
            return false;
        };
        let page_controller = ctx.link().page_controller();
        match msg {
            Msg::OpenGallery(start) => {
                if let Some(page_controller) = page_controller {
                    page_controller.push_page(GalleryPage::new(destination.id).start(start));
                }
                false
            }
            Msg::Share => {
                if let Some(page_controller) = page_controller {
                    page_controller.show_modal_bottom_sheet(ShareSheet::new(destination.id));
                }
                false
            }
            Msg::ShowPlan(show_plan) => {
                self.show_plan = show_plan;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let Some(destination) = find_destination(&ctx.props().id) else {
            return Scaffold::with_title("Unknown destination")
                .body(
                    Container::new()
                        .padding(2)
                        .with_child(format!("There is no destination '{}'.", ctx.props().id)),
                )
                .into();
        };

        let share = ActionIcon::new("fa fa-lg fa-share-alt")
            .aria_label("Share")
            .on_activate(link.callback(|_| Msg::Share));

        let app_bar = ApplicationBar::new()
            .title(destination.place())
            .with_action(FavoriteButton::new(destination.id))
            .with_action(share);

        let gallery_button = |n: usize, container: Container| {
            activatable(
                container
                    .class("pwt-pointer")
                    .onclick(link.callback(move |_| Msg::OpenGallery(n))),
                format!("Show picture {} of {}", n + 1, destination.name),
                link.callback(move |_| Msg::OpenGallery(n)),
            )
        };

        let header = gallery_button(0, Container::new().class("travel-content"))
            .with_child(picture_hero(destination, "travel-wide"));

        let mut more_pictures = Row::new().gap(2);
        for n in 1..Destination::PICTURES {
            more_pictures.add_child(
                gallery_button(n, Container::new()).with_child(
                    Hero::new(gallery_tag(destination, n)).with_child(
                        Image::new(destination.picture(n))
                            .class("travel-picture travel-small")
                            .attribute("alt", format!("{} picture {}", destination.name, n + 1)),
                    ),
                ),
            );
        }

        let facts = List::from_tiles(vec![
            fact("map-marker", "Location", destination.place()),
            fact("calendar", "Best season", destination.season.into()),
            fact(
                "clock-o",
                "Recommended stay",
                format!("{} days", destination.days),
            ),
            fact("tags", "Highlights", destination.tags.join(", ")),
        ])
        .grid_template_columns("auto auto 1fr");

        let content = Column::new()
            .class("travel-content")
            .padding(2)
            .gap(2)
            .with_child(title_hero(destination, FontStyle::HeadlineSmall))
            .with_child(destination_badges(destination))
            .with_child(Container::from_tag("p").with_child(destination.description))
            .with_child(
                Button::new("Plan a trip")
                    .icon_class("fa fa-calendar-plus-o")
                    .class(ColorScheme::Primary)
                    .onclick(link.callback(|_| Msg::ShowPlan(true))),
            )
            .with_child(
                Container::new()
                    .class(FontStyle::TitleSmall)
                    .with_child("More pictures"),
            )
            .with_child(more_pictures)
            .with_child(facts);

        let plan_dialog = self.show_plan.then(|| {
            let close = link.callback(|_| Msg::ShowPlan(false));
            AdaptiveDialog::new(format!("Trip to {}", destination.name))
                .on_close(close.clone())
                .with_child(
                    Container::new()
                        .padding(2)
                        .with_child(TripForm::new(close).destination(destination.id)),
                )
        });

        Scaffold::new()
            .application_bar(app_bar)
            .body(
                Column::new()
                    .class(FlexFit)
                    .style("overflow", "auto")
                    .with_child(header)
                    .with_child(content)
                    .with_optional_child(plan_dialog),
            )
            .into()
    }
}

fn fact(icon: &str, label: &str, value: String) -> ListTile {
    ListTile::new()
        .class(AlignItems::Center)
        .with_child(Fa::new(icon).fixed_width().class("pwt-color-primary"))
        .with_child(
            Container::new()
                .class(FontStyle::LabelLarge)
                .with_child(label.to_string()),
        )
        .with_child(value)
}
