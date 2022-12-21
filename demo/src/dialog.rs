use yew::prelude::*;

use pwt::{
    props::{ContainerBuilder, EventSubscriber, WidgetBuilder},
    widget::{Button, Column, Dialog, Row},
};

#[function_component]
pub fn DialogDemo() -> Html {
    let show_dialog = use_state(|| false);

    let onclick = {
        let show_dialog = show_dialog.clone();
        move |_| show_dialog.set(true)
    };
    let mut row = Row::new()
        .padding(2)
        .gap(2)
        .with_child(Button::new("Show Dialog").onclick(onclick));

    if *show_dialog {
        let show_dialog2 = show_dialog.clone();
        row.add_child(
            Dialog::new("Example Dialog")
                .on_close(move |_| {
                    show_dialog.set(false);
                })
                .style("min-width: 200px;")
                .with_child(
                    Column::new()
                        .with_child(Row::new().padding(4).with_child(html! { {"Content"}}))
                        .with_child(
                            Row::new()
                                .padding(2)
                                .gap(2)
                                .with_flex_spacer()
                                .class("pwt-bg-color-neutral-container-alt")
                                .with_child(
                                    Button::new("OK").onclick(move |_| show_dialog2.set(false)),
                                ),
                        ),
                ),
        );
    }
    row.into()
}
