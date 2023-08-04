use std::rc::Rc;

use pwt::{
    css::{ColorScheme, Flex},
    prelude::*,
    props::ExtractPrimaryKey,
    state::Store,
    widget::{
        data_table::{DataTable, DataTableColumn, DataTableHeader},
        form::Combobox,
        ActionIcon, Button, Column, Panel, Row,
    },
};
use serde::Deserialize;
use yew::Component;

#[derive(Clone, PartialEq, Deserialize)]
struct Record {
    id: usize,
    selection: String,
}

impl ExtractPrimaryKey for Record {
    fn extract_key(&self) -> yew::virtual_dom::Key {
        self.id.into()
    }
}

pub enum Msg {
    New,
    Delete(usize),
    Update(usize, String),
}

pub struct WidgetColumnGrid {
    store: Store<Record>,
    columns: Rc<Vec<DataTableHeader<Record>>>,
    next_id: usize,
}

impl Component for WidgetColumnGrid {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let store: Store<Record> = Store::new();
        let columns: Rc<Vec<DataTableHeader<Record>>> = {
            let store = store.clone();
            let link = ctx.link().clone();
            let link2 = ctx.link().clone();
            Rc::new(vec![
                DataTableColumn::new("Select")
                    .render(move |rec: &Record| {
                        let mut list: Vec<AttrValue> =
                            vec!["Foo".into(), "Bar".into(), "Baz".into()];
                        list.retain(|entry| {
                            store
                                .read()
                                .iter()
                                .find(|el| &el.selection == entry.as_str())
                                .is_none()
                        });
                        if rec.selection != "" {
                            list.push(rec.selection.clone().into());
                        }

                        let link = link.clone();
                        let key = rec.id;

                        if list.is_empty() {
                            "No elements left".into()
                        } else {
                            Combobox::new()
                                .items(Rc::new(list))
                                .on_change(move |e: String| {
                                    link.send_message(Msg::Update(key, e.clone()));
                                })
                                .into()
                        }
                    })
                    .into(),
                DataTableColumn::new("Remove")
                    .width("100px")
                    .render(move |rec: &Record| {
                        let link = link2.clone();
                        let id = rec.id;
                        ActionIcon::new("fa fa-minus")
                            .class(ColorScheme::Primary)
                            .on_activate(move |_| link.send_message(Msg::Delete(id)))
                            .into()
                    })
                    .into(),
            ])
        };
        Self {
            store,
            columns,
            next_id: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::New => {
                if self.store.read().len() < 3 {
                    self.store.write().push(Record {
                        id: self.next_id,
                        selection: "".to_owned(),
                    });
                    self.next_id += 1;
                }
            }
            Msg::Update(id, selection) => {
                let mut list = self.store.write();
                if let Some(entry) = list.iter_mut().find(|x| x.id == id) {
                    entry.selection = selection;
                }
            }
            Msg::Delete(id) => {
                let mut list = self.store.write();
                list.retain(|r| r.id != id);
            }
        };
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let on_add = {
            let link = ctx.link().clone();
            move |_| {
                link.send_message(Msg::New);
            }
        };

        Panel::new()
            .title("Dynamic Selection Grid")
            .with_child(
                Column::new()
                    .gap(2)
                    .with_child(
                        Row::new()
                            .with_child(
                                DataTable::new(Rc::clone(&self.columns), self.store.clone())
                                    .striped(false)
                                    .class(Flex::Fill),
                            )
                            .attribute("style", "height: 200px;"),
                    )
                    .with_child(
                        Row::new().margin(1).with_child(
                            Button::new("")
                                .class(ColorScheme::Primary)
                                .icon_class("fa fa-plus")
                                .disabled(self.store.read().len() >= 3)
                                .onclick(on_add),
                        ),
                    ),
            )
            .into()
    }
}
