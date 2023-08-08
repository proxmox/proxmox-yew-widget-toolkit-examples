mod widgetcolumn_grid;
use widgetcolumn_grid::WidgetColumnGrid;

mod drag_drop;
use drag_drop::DragDropList;

mod wizard;
use wizard::Wizard;

use pwt::prelude::*;
use pwt::widget::{Button, Column};

#[function_component]
pub fn ExampleWidgets() -> Html {
    let show_wizard = use_state(|| false);
    let close_callback = use_callback(
        {
            let show_wizard = show_wizard.clone();
            move |(), ()| show_wizard.set(false)
        },
        (),
    );

    let show_callback = {
        let show_wizard = show_wizard.clone();
        move |_| show_wizard.set(true)
    };

    let mut column = Column::new()
        .padding(2)
        .class("pwt-size-fit-content")
        .gap(3)
        .with_child(html! {<WidgetColumnGrid/>})
        .with_child(html! {<DragDropList/>})
        .with_child(Button::new("Show Wizard").onclick(show_callback));

    if *show_wizard {
        column.add_child(Wizard::new().on_close(close_callback))
    }

    column.into()
}
