mod widgetcolumn_grid;
use widgetcolumn_grid::WidgetColumnGrid;

mod drag_drop;
use drag_drop::DragDropList;

use pwt::prelude::*;
use pwt::widget::Column;

#[function_component]
pub fn ExampleWidgets() -> Html {
    Column::new()
        .padding(2)
        .class("pwt-size-fit-content")
        .gap(3)
        .with_child(html! {<WidgetColumnGrid/>})
        .with_child(html! {<DragDropList/>})
        .into()
}
