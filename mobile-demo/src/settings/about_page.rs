use pwt::css::{AlignItems, FlexFit, FontStyle};
use pwt::touch::prelude::*;
use pwt::touch::Scaffold;
use pwt::widget::{Column, Container, Fa};

/// What the app is about, opened from the drawer and from the settings.
#[derive(Properties, PartialEq, Default)]
pub struct AboutPage {}

impl AboutPage {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(AboutPage => AboutPageComp);

struct AboutPageComp;

impl Component for AboutPageComp {
    type Message = ();
    type Properties = AboutPage;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Scaffold::with_title("About")
            .body(
                Column::new()
                    .class(FlexFit)
                    .style("overflow", "auto")
                    .with_child(
                        Column::new()
                            .class("travel-content")
                            .class(AlignItems::Center)
                            .padding(4)
                            .gap(2)
                            .with_child(Fa::new("globe").class("pwt-color-primary").large_4x())
                            .with_child(
                                Container::new()
                                    .class(FontStyle::HeadlineSmall)
                                    .with_child("Travel Explorer"),
                            )
                            .with_child(Container::from_tag("p").with_child(
                                "A demo of the touch widgets of the Proxmox Yew Widget Toolkit. \
                                 The destinations, trips, and messages are made up, and nothing \
                                 leaves your browser.",
                            ))
                            .with_child(Container::from_tag("p").with_child(
                                "The illustrations were made for this demo and share its license.",
                            )),
                    ),
            )
            .into()
    }
}
