use pwt::css::{AlignItems, FlexFit, JustifyContent};
use pwt::prelude::*;
use pwt::touch::{ApplicationBar, Hero, PageView, Scaffold};
use pwt::widget::{ActionIcon, Column, Container, Image, Row};

use crate::data::{find_destination, Destination};
use crate::explore::picture_tag;

/// Full screen pictures of a destination, pushed on top of the page stack without a route.
#[derive(Properties, PartialEq)]
pub struct GalleryPage {
    /// The id of the destination.
    id: &'static str,
    /// The index of the picture to show first.
    #[prop_or_default]
    start: usize,
}

impl GalleryPage {
    pub fn new(id: &'static str) -> Self {
        yew::props!(Self { id })
    }

    /// The picture to show first.
    pub fn start(mut self, start: usize) -> Self {
        self.start = start;
        self
    }
}

into_vnode!(GalleryPage => GalleryPageComp);

/// The hero tag of a picture. The first picture is the one of the lists and the header.
pub fn gallery_tag(destination: &Destination, n: usize) -> String {
    match n {
        0 => picture_tag(destination),
        n => format!("picture-{}-{n}", destination.id),
    }
}

enum Msg {
    Show(usize),
    Step(isize),
}

struct GalleryPageComp {
    current: usize,
}

impl Component for GalleryPageComp {
    type Message = Msg;
    type Properties = GalleryPage;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current: ctx.props().start,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let count = Destination::PICTURES as isize;
        self.current = match msg {
            Msg::Show(page) => page,
            Msg::Step(delta) => (self.current as isize + delta).rem_euclid(count) as usize,
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let Some(destination) = find_destination(ctx.props().id) else {
            return Html::default();
        };
        let count = Destination::PICTURES;

        let mut view = PageView::new()
            .view_page(self.current)
            .on_page_change(link.callback(Msg::Show));
        for n in 0..count {
            // Only the visible picture flies back to its thumbnail, the others are off screen.
            let tag = if n == self.current {
                gallery_tag(destination, n)
            } else {
                format!("gallery-{}-{n}", destination.id)
            };
            view = view.with_child(
                // pages are plain boxes, so fill them explicitly to center the picture
                Column::new()
                    .style("height", "100%")
                    .class(AlignItems::Center)
                    .class(JustifyContent::Center)
                    .with_child(
                        Hero::new(tag).class("travel-gallery-picture").with_child(
                            Image::new(destination.picture(n))
                                .class("travel-picture travel-wide")
                                .attribute("alt", format!("{} picture {}", destination.name, n + 1))
                                // pictures are swiped, not dragged out of the page
                                .attribute("draggable", "false"),
                        ),
                    ),
            );
        }

        let mut dots = Row::new().gap(1).class(AlignItems::Center);
        for n in 0..count {
            dots.add_child(
                Container::new()
                    .class("travel-dot")
                    .class((n == self.current).then_some("active")),
            );
        }

        let controls = Row::new()
            .padding(2)
            .gap(4)
            .class(AlignItems::Center)
            .class(JustifyContent::Center)
            .with_child(
                ActionIcon::new("fa fa-lg fa-chevron-left")
                    .aria_label("Previous picture")
                    .on_activate(link.callback(|_| Msg::Step(-1))),
            )
            .with_child(dots)
            .with_child(
                ActionIcon::new("fa fa-lg fa-chevron-right")
                    .aria_label("Next picture")
                    .on_activate(link.callback(|_| Msg::Step(1))),
            );

        Scaffold::new()
            .application_bar(ApplicationBar::new().title(format!(
                "{} ({}/{count})",
                destination.name,
                self.current + 1
            )))
            .body(
                Column::new()
                    .class(FlexFit)
                    .class("pwt-bg-color-neutral-alt")
                    // the page view sizes its pages to its own box, so it needs a definite height
                    .with_child(Container::new().class("pwt-flex-fill").with_child(view))
                    .with_child(controls),
            )
            .into()
    }
}
