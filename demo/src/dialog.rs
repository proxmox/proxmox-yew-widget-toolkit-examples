use std::rc::Rc;

use pwt::css::{Flex, Overflow};
use pwt::prelude::*;

use pwt::props::ExtractPrimaryKey;
use pwt::state::Store;
use pwt::widget::data_table::{DataTable, DataTableColumn, DataTableHeader};
use pwt::widget::Container;
use pwt::{
    props::{ContainerBuilder, EventSubscriber, WidgetBuilder},
    widget::{AlertDialog, Button, Column, Dialog, Panel, Row},
};

use crate::LoremIpsum;

#[derive(PartialEq)]
enum ViewState {
    Main,
    ShowDialog,
    ShowResizableDialog,
    ShowAlert,
    ShowGridDialog,
}

#[derive(PartialEq, Clone)]
struct Data {
    name: AttrValue,
    value: AttrValue,
}

impl ExtractPrimaryKey for Data {
    fn extract_key(&self) -> yew::virtual_dom::Key {
        yew::virtual_dom::Key::from(self.name.to_string())
    }
}

fn get_columns() -> Rc<Vec<DataTableHeader<Data>>> {
    Rc::new(vec![
        DataTableColumn::new("Name")
            .get_property(|r: &Data| &r.name)
            .sort_order(true)
            .into(),
        DataTableColumn::new("Value")
            .get_property(|r: &Data| &r.value)
            .into(),
    ])
}

#[function_component]
pub fn DialogDemo() -> Html {
    let view_state = use_state(|| ViewState::Main);
    let mut data = Vec::with_capacity(200);
    for i in 0..200 {
        data.push(Data {
            name: format!("Entry {i}").into(),
            value: format!("Value {i}").into(),
        });
    }
    let store = Store::new();
    store.set_data(data);

    let close_callback = use_callback((), {
        let view_state = view_state.clone();
        move |(), ()| view_state.set(ViewState::Main)
    });

    let mut row = Column::new()
        .padding(4)
        .gap(2)
        .with_child(Button::new("Show Dialog").onclick({
            let view_state = view_state.clone();
            move |_| view_state.set(ViewState::ShowDialog)
        }))
        .with_child(Button::new("Show resizable Dialog").onclick({
            let view_state = view_state.clone();
            move |_| view_state.set(ViewState::ShowResizableDialog)
        }))
        .with_child(Button::new("Show Alert").onclick({
            let view_state = view_state.clone();
            move |_| view_state.set(ViewState::ShowAlert)
        }))
        .with_child(Button::new("Show Grid Dialog").onclick({
            let view_state = view_state.clone();
            move |_| view_state.set(ViewState::ShowGridDialog)
        }));

    let child: Html = match *view_state {
        ViewState::Main => {
            /* do nothing */
            html! {}
        }
        ViewState::ShowAlert => AlertDialog::new("This is an alert!")
            .on_close(close_callback)
            .draggable(true)
            .into(),
        ViewState::ShowDialog => Dialog::new("Example Dialog")
            .draggable(true)
            .on_close(close_callback)
            .width(800)
            .min_width(200)
            .with_child(
                Column::new()
                    .class(Flex::Fill)
                    .class(Overflow::Auto)
                    .with_child(html! {
                        <div class="pwt-overflow-auto"><LoremIpsum/></div>
                    })
                    .with_child(
                        Row::new()
                            .padding(2)
                            .gap(2)
                            .with_flex_spacer()
                            .class("pwt-bg-color-neutral-container-alt")
                            .with_child(Button::new("OK").autofocus(true).onclick({
                                let view_state = view_state.clone();
                                move |_| view_state.set(ViewState::Main)
                            })),
                    ),
            )
            .into(),
        ViewState::ShowResizableDialog => Dialog::new("Example Resizable Dialog")
            .draggable(true)
            .resizable(true)
            .on_close(close_callback)
            .width(800)
            .min_width(200)
            .min_height(50)
            .with_child(
                Column::new()
                    .class(Flex::Fill)
                    .class(Overflow::Auto)
                    .with_child(html! {
                        <div class="pwt-overflow-auto"><LoremIpsum/></div>
                    })
                    .with_flex_spacer()
                    .with_child(
                        Row::new()
                            .padding(2)
                            .gap(2)
                            .with_flex_spacer()
                            .class("pwt-bg-color-neutral-container-alt")
                            .with_child(Button::new("OK").autofocus(true).onclick({
                                let view_state = view_state.clone();
                                move |_| view_state.set(ViewState::Main)
                            })),
                    ),
            )
            .into(),
        ViewState::ShowGridDialog => Dialog::new("Example Resizable Dialog with Grid")
            .draggable(true)
            .resizable(true)
            .on_close(close_callback)
            .width(500)
            .height(400)
            .min_width(200)
            .min_height(50)
            .with_child(DataTable::new(get_columns(), store).class(Overflow::Auto))
            .into(),
    };

    row.add_child(child);
    Container::new()
        .padding(2)
        .with_child(Panel::new().border(true).title("Dialogs").with_child(row))
        .into()
}
