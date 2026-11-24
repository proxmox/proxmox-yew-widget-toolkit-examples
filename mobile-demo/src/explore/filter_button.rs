use pwt::prelude::*;
use pwt::touch::MaterialAppScopeExt;
use pwt::widget::ActionIcon;

use super::filter_panel::FilterPanel;

/// Opens the filters in the end drawer.
#[derive(Properties, PartialEq, Default)]
pub struct FilterButton {}

impl FilterButton {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(FilterButton => FilterButtonComp);

struct FilterButtonComp;

impl Component for FilterButtonComp {
    type Message = ();
    type Properties = FilterButton;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: ()) -> bool {
        if let Some(page_controller) = ctx.link().page_controller() {
            page_controller.show_end_drawer(FilterPanel::new());
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        ActionIcon::new("fa fa-lg fa-filter")
            .aria_label("Filter")
            .on_activate(ctx.link().callback(|_| ()))
            .into()
    }
}
