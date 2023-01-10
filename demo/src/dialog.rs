use yew::prelude::*;

use pwt::{
    props::{ContainerBuilder, EventSubscriber, WidgetBuilder},
    widget::{AlertDialog, Button, Column, Dialog, Row},
};

#[derive(PartialEq)]
enum ViewState {
    Main,
    ShowDialog,
    ShowAlert,
}

#[function_component]
pub fn DialogDemo() -> Html {
    let view_state = use_state(|| ViewState::Main);

    let close_callback = use_callback({
        let view_state = view_state.clone();
        move |(), ()| {
            view_state.set(ViewState::Main)
        }
    }, ());

    let mut row = Column::new()
        .padding(2)
        .gap(2)
        .with_child(Button::new("Show Dialog").onclick({
            let view_state = view_state.clone();
            move |_| view_state.set(ViewState::ShowDialog)
        }))
        .with_child(Button::new("Show Alert").onclick({
            let view_state = view_state.clone();
            move |_| view_state.set(ViewState::ShowAlert)
        }));

    match *view_state  {
        ViewState::Main => { /* do nothing */ }
        ViewState::ShowAlert => {
            row.add_child(
                AlertDialog::new("This is an alert!").on_close(close_callback)
            );
        }
        ViewState::ShowDialog => {
            row.add_child(
                Dialog::new("Example Dialog")
                    .on_close(close_callback)
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
                                        Button::new("OK").onclick({
                                            let view_state = view_state.clone();
                                            move |_| view_state.set(ViewState::Main)
                                        })
                                    ),
                            ),
                    ),
            );
        }
    }

    row.into()
}
