use pwt::css::ColorScheme;
use pwt::prelude::*;
use pwt::widget::{Button, Panel, Row};

use web_sys::Element;
use yew::{AttrValue, Component, DragEvent, TargetCast};

pub struct DragDropList {
    items: Vec<AttrValue>,
    index_dragged: Option<usize>,
    hover_index: Option<usize>,
}

pub enum Msg {
    DragStart(usize),
    DragEnter(usize),
    Drop,
    CancelDrag,
}

impl Component for DragDropList {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            items: vec![
                "foo".into(),
                "bar".into(),
                "baz".into(),
                "bob".into(),
                "alice".into(),
            ],
            index_dragged: None,
            hover_index: None,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Drop => {
                if let (Some(mut target), Some(source)) = (self.hover_index, self.index_dragged) {
                    let item = self.items.remove(source);
                    if source < target {
                        target -= 1;
                    }
                    self.items.insert(target, item);
                }
                self.hover_index = None;
                self.index_dragged = None;
            }
            Msg::DragStart(index) => {
                self.index_dragged = Some(index);
                self.hover_index = Some(index);
            }
            Msg::DragEnter(id) => {
                self.hover_index = Some(id);
            }
            Msg::CancelDrag => {
                self.index_dragged = None;
                self.hover_index = None;
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut buttons = Vec::new();
        for (i, item) in self.items.iter().enumerate() {
            let is_dragged_index = self.index_dragged == Some(i);
            let is_hovered_over = self.hover_index == Some(i);

            let drag_over = {
                let link = ctx.link().clone();
                move |event: DragEvent| {
                    event.prevent_default();
                    // before for left half, after for right half
                    let target: Element = event.target_dyn_into().unwrap();
                    let rect = target.get_bounding_client_rect();

                    let middle = rect.x() + rect.width() / 2.0;
                    if (event.client_x() as f64) < middle || is_dragged_index {
                        link.send_message(Msg::DragEnter(i));
                    } else {
                        link.send_message(Msg::DragEnter(i + 1));
                    }
                }
            };

            let drag_enter = move |event: DragEvent| {
                if !is_dragged_index {
                    event.prevent_default();
                }
            };

            let drag_start = {
                let link = ctx.link().clone();
                move |event: DragEvent| {
                    link.send_message(Msg::DragStart(i));
                    let data = event.data_transfer().unwrap();
                    // we have to set some data, otherwise the drop handler does not trigger
                    let _ = data.set_data("", "");
                }
            };

            let drop = {
                let link = ctx.link().clone();
                move |_event: DragEvent| link.send_message(Msg::Drop)
            };

            let drag_end = {
                let link = ctx.link().clone();
                move |_event| link.send_message(Msg::CancelDrag)
            };

            buttons.push(
                Button::new(item)
                    .key(format!("drag-button-{i}"))
                    .class(if !is_hovered_over || !is_dragged_index {
                        ColorScheme::Primary
                    } else {
                        ColorScheme::Secondary
                    })
                    .attribute(
                        "style",
                        if !is_hovered_over && is_dragged_index {
                            "display:none;"
                        } else {
                            ""
                        },
                    )
                    .ondragstart(drag_start)
                    .ondragenter(drag_enter)
                    .ondragover(drag_over)
                    .ondragend(drag_end)
                    .ondrop(drop)
                    .attribute("draggable", "true"),
            );
        }

        match (self.hover_index, self.index_dragged) {
            (Some(hover_index), Some(index_dragged)) if hover_index != index_dragged => {
                let link = ctx.link().clone();
                buttons.insert(
                    hover_index,
                    Button::new(self.items[index_dragged].clone())
                        .key("ghost")
                        .class(ColorScheme::Secondary)
                        .ondragenter(|event: DragEvent| event.prevent_default())
                        .ondragover(|event: DragEvent| event.prevent_default())
                        .ondrop(move |_event: DragEvent| link.send_message(Msg::Drop)),
                );
            }
            _ => {}
        }

        let mut row = Row::new().gap(2).padding(2);
        for button in buttons.into_iter() {
            row.add_child(button);
        }

        Panel::new()
            .border(true)
            .title("Drag and Drop Items")
            .with_child(row)
            .into()
    }
}
