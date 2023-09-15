use std::{collections::HashSet, rc::Rc};

use pwt::{
    css::{ColorScheme, Flex, Overflow},
    props::{ContainerBuilder, CssPaddingBuilder, EventSubscriber, WidgetBuilder},
    state::Selection,
    widget::{form::Field, Button, Column, Dialog, InputPanel, Row, TabBarItem, TabPanel},
};
use yew::html;
use yew::virtual_dom::{VComp, VNode};
use yew::{html::IntoEventCallback, Callback, Component, Properties};

const PAGES: &[(&str, &str)] = &[
    (
        "General",
        "General Information, please enter text to continue.",
    ),
    ("Options", "Some Options, please enter text to continue."),
    (
        "Details",
        "Detailed Information, please enter text to continue.",
    ),
    (
        "Additional",
        "Additional Information, please enter text to continue.",
    ),
    ("Last", "Last Page, please enter text to be able to finish."),
];

#[derive(Clone, Properties, PartialEq)]
pub struct Wizard {
    on_close: Option<Callback<()>>,
}

impl Wizard {
    pub fn new() -> Self {
        yew::props!(Wizard {})
    }
    pub fn on_close(mut self, cb: impl IntoEventCallback<()>) -> Self {
        self.on_close = cb.into_event_callback();
        self
    }
}

pub struct PwtWizard {
    page: usize,
    selection: Selection,
    pages_valid: HashSet<usize>,
}

pub enum Msg {
    SelectPage(usize),
    TextChange(usize, String),
    SelectionChange(Selection),
}

impl Component for PwtWizard {
    type Message = Msg;

    type Properties = Wizard;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let selection = Selection::new().on_select(ctx.link().callback(Msg::SelectionChange));
        Self {
            page: 0,
            pages_valid: HashSet::new(),
            selection,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectPage(page) => {
                self.page = page;
                self.selection.select(format!("{}", self.page));
            }
            Msg::TextChange(page, text) => {
                if text.is_empty() {
                    self.pages_valid.remove(&page);
                } else {
                    self.pages_valid.insert(page);
                }
            }
            Msg::SelectionChange(selection) => {
                if let Some(key) = selection.selected_key() {
                    let index: usize = key.parse().unwrap();
                    self.page = index;
                }
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let page_num = self.page + 1;
        let mut tab_panel = TabPanel::new()
            .class(Flex::Fill)
            .tab_bar_style(pwt::widget::TabBarStyle::MaterialPrimary)
            .selection(self.selection.clone());

        let mut next_is_disabled = false;
        for i in 0..page_num {
            if !self.pages_valid.contains(&i) {
                next_is_disabled = true;
                break;
            }
        }

        for (i, (id, text)) in PAGES.iter().enumerate() {
            let link = ctx.link().clone();
            let mut disabled = false;
            for j in 0..i {
                if !self.pages_valid.contains(&j) {
                    disabled = true;
                    break;
                }
            }

            tab_panel.add_item(
                TabBarItem::new()
                    .key(format!("{i}"))
                    .label(*id)
                    .disabled(disabled),
                InputPanel::new()
                    .class(Flex::Fill)
                    .padding(2)
                    .with_custom_child(html! {{text}})
                    .with_field(
                        "Required Field",
                        Field::new().on_change(move |value| {
                            link.send_message(Msg::TextChange(i, value));
                        }),
                    ),
            );
        }

        let is_last = self.page + 1 >= PAGES.len();
        let is_first = self.page == 0;

        let window = Dialog::new(format!("Wizard (Page {page_num} of {})", PAGES.len()))
            .style("min-width: 500px")
            .resizable(true)
            .on_close(ctx.props().on_close.clone())
            .with_child(
                Column::new()
                    .class(Flex::Fill)
                    .class(Overflow::Auto)
                    .with_child(Row::new().class(Flex::Fill).with_child(tab_panel))
                    .with_child(
                        Row::new()
                            .padding(2)
                            .gap(2)
                            .with_flex_spacer()
                            .class(ColorScheme::Surface)
                            .class("pwt-panel-header")
                            .with_optional_child(if !is_first {
                                Some(
                                    Button::new("Back")
                                        .class(ColorScheme::Primary)
                                        .disabled(is_first)
                                        .onclick({
                                            let link = ctx.link().clone();
                                            let new_page =
                                                if self.page > 0 { self.page - 1 } else { 0 };
                                            move |_| {
                                                if !is_first {
                                                    link.send_message(Msg::SelectPage(new_page));
                                                }
                                            }
                                        }),
                                )
                            } else {
                                None
                            })
                            .with_child(
                                Button::new(if is_last { "OK" } else { "Next" })
                                    .class(ColorScheme::Primary)
                                    .disabled(next_is_disabled)
                                    .onclick({
                                        let link = ctx.link().clone();
                                        let close_callback = ctx.props().on_close.clone();
                                        let new_page = self.page + 1;
                                        move |_| {
                                            if is_last {
                                                if let Some(cb) = &close_callback {
                                                    cb.emit(());
                                                }
                                            } else {
                                                link.send_message(Msg::SelectPage(new_page));
                                            }
                                        }
                                    }),
                            ),
                    ),
            );

        window.into()
    }
}

impl Into<VNode> for Wizard {
    fn into(self) -> VNode {
        let comp = VComp::new::<PwtWizard>(Rc::new(self), None);
        VNode::from(comp)
    }
}
