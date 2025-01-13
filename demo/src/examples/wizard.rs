use std::{collections::HashSet, rc::Rc};

use pwt::{
    css::{ColorScheme, Flex, Overflow},
    props::{ContainerBuilder, CssPaddingBuilder, EventSubscriber, FieldBuilder, WidgetBuilder},
    state::Selection,
    widget::{
        form::{Checkbox, Combobox, Field, Form, FormContext, InputType, Number},
        Button, Column, Dialog, InputPanel, Row, TabBarItem, TabPanel,
    },
};
use yew::html;
use yew::virtual_dom::{VComp, VNode};
use yew::{html::IntoEventCallback, Callback, Component, Properties};

const PAGES: &[(&str, &str, &[u8])] = &[
    (
        "General",
        "General Information, please enter text to continue.",
        &[0, 1, 2, 3, 4, 5],
    ),
    (
        "Options",
        "Some Options, please enter text to continue.",
        &[5, 4, 3, 2, 1, 0],
    ),
    (
        "Details",
        "Detailed Information, please enter text to continue.",
        &[0, 2, 1, 3, 4, 5],
    ),
];

#[derive(Clone, Properties, PartialEq)]
pub struct Wizard {
    #[prop_or_default]
    on_close: Option<Callback<()>>,
}

impl Default for Wizard {
    fn default() -> Self {
        Self::new()
    }
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
    form_ctx: Vec<FormContext>,
}

pub enum Msg {
    SelectPage(usize),
    ChangeValid(usize, bool),
    SelectionChange(Selection),
}

impl Component for PwtWizard {
    type Message = Msg;

    type Properties = Wizard;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let selection = Selection::new().on_select(ctx.link().callback(Msg::SelectionChange));
        let mut form_ctx = Vec::new();
        for i in 0..PAGES.len() {
            form_ctx.push(FormContext::new().on_change(ctx.link().callback(
                move |form_ctx: FormContext| Msg::ChangeValid(i, form_ctx.read().is_valid()),
            )));
        }
        Self {
            page: 0,
            pages_valid: HashSet::new(),
            selection,
            form_ctx,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectPage(page) => {
                self.page = page;
                self.selection.select(format!("{}", self.page));
            }
            Msg::ChangeValid(page, valid) => {
                if valid {
                    self.pages_valid.insert(page);
                } else {
                    self.pages_valid.remove(&page);
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
            .class(Overflow::Auto)
            .class(Flex::Fill)
            .tab_bar_style(pwt::widget::TabBarStyle::MaterialPrimary)
            .selection(self.selection.clone());

        let mut next_is_disabled = false;
        for i in 0..page_num {
            if !self.pages_valid.contains(&i) && i != PAGES.len() {
                next_is_disabled = true;
                break;
            }
        }

        for (i, (id, text, _)) in PAGES.iter().enumerate() {
            let mut disabled = false;
            for j in 0..i {
                if !self.pages_valid.contains(&j) {
                    disabled = true;
                    break;
                }
            }

            let custom_text = html! {<span key={"warning-text"}>{text}</span>};

            let mut input_panel = InputPanel::new()
                .class(Overflow::Auto)
                .class(Flex::Fill)
                .padding(2)
                .with_large_custom_child(custom_text);

            let field_list = PAGES[i].2;

            for i in (0..field_list.len()).step_by(2) {
                match field_list[i] {
                    0 => {
                        input_panel.add_field("Username", Field::new().name("user").required(true))
                    }
                    1 => input_panel.add_field(
                        "Password",
                        Field::new()
                            .input_type(InputType::Password)
                            .name("password")
                            .required(true),
                    ),
                    2 => input_panel.add_field("u8 Feld", Number::<u8>::new().name("u8-number")),
                    3 => input_panel.add_field(
                        "Combobox",
                        Combobox::new()
                            .name("combobox")
                            .with_trigger("fa fa-user", false)
                            .with_item("Admin")
                            .with_item("User")
                            .with_item("Root"),
                    ),
                    4 => input_panel.add_field("Textfield", Field::new().name("textfield")),
                    5 => input_panel.add_field("Checkbox", Checkbox::new().name("checkbox")),
                    _ => {}
                }

                match field_list[i + 1] {
                    0 => input_panel
                        .add_right_field("Username", Field::new().name("user").required(true)),
                    1 => input_panel.add_right_field(
                        "Password",
                        Field::new()
                            .input_type(InputType::Password)
                            .name("password")
                            .required(true),
                    ),
                    2 => input_panel
                        .add_right_field("u8 Feld", Number::<u8>::new().name("u8-number")),
                    3 => input_panel.add_right_field(
                        "Combobox",
                        Combobox::new()
                            .name("combobox")
                            .with_trigger("fa fa-user", false)
                            .with_item("Admin")
                            .with_item("User")
                            .with_item("Root"),
                    ),
                    4 => input_panel.add_right_field("Textfield", Field::new().name("textfield")),
                    5 => input_panel.add_right_field("Checkbox", Checkbox::new().name("checkbox")),
                    _ => {}
                }
            }

            tab_panel.add_item(
                TabBarItem::new()
                    .key(format!("{i}"))
                    .label(*id)
                    .disabled(disabled),
                Form::new()
                    .class(Overflow::Auto)
                    .class(Flex::Fill)
                    .form_context(self.form_ctx[i].clone())
                    .with_child(input_panel),
            );
        }

        let mut summary = Column::new()
            .gap(1)
            .padding(1)
            .class(Overflow::Auto)
            .class(Flex::Fill);
        for (i, val) in self
            .form_ctx
            .iter()
            .map(|f| f.get_submit_data())
            .enumerate()
        {
            let text = serde_json::to_string_pretty(&val).unwrap();
            summary.add_child(
                Row::new()
                    .with_child(format!("{}: ", PAGES[i].0))
                    .with_child(html! {<pre>{text}</pre>}),
            )
        }

        tab_panel.add_item(
            TabBarItem::new()
                .key(format!("{}", PAGES.len()))
                .label("Summary")
                .disabled((0..PAGES.len()).any(|i| !self.pages_valid.contains(&i))),
            InputPanel::new()
                .padding(2)
                .with_large_custom_child(summary),
        );

        let is_last = self.page + 1 >= PAGES.len() + 1;
        let is_first = self.page == 0;

        let form = Form::new()
            .class(Overflow::Auto)
            .class(Flex::Fill)
            .with_child(tab_panel);

        let window = Dialog::new(format!("Wizard (Page {page_num} of {})", PAGES.len() + 1))
            .on_close(ctx.props().on_close.clone())
            .with_child(
                Column::new()
                    .class(Flex::Fill)
                    .class(Overflow::Auto)
                    .with_child(
                        Row::new()
                            .class(Overflow::Auto)
                            .class(Flex::Fill)
                            .with_child(form),
                    )
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

impl From<Wizard> for VNode {
    fn from(val: Wizard) -> Self {
        let comp = VComp::new::<PwtWizard>(Rc::new(val), None);
        VNode::from(comp)
    }
}
