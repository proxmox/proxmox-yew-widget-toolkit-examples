use std::rc::Rc;

use pwt::css::{Flex, Overflow};
use pwt::widget::form::{Checkbox, Combobox};
use yew::prelude::*;

use pwt::prelude::*;

use pwt::props::ExtractKeyFn;
use pwt::state::{use_selection, Selection, Store};
use pwt::widget::{data_table::*, Column, InputPanel, Panel};

#[function_component]
pub fn DataTableDemo() -> Html {
    let header0 = DataTableColumn::selection_indicator();

    let header1 = DataTableColumn::new("RecordId")
        .width("max-content")
        .sorter(usize::cmp)
        .sort_order(false)
        .render(|n: &usize| html! {n.to_string()});

    let header2 = DataTableColumn::new("TEHE2").flex(1).render(|n: &usize| {
        html! {
            <>
                <h3>{"This is a header"}</h3>
                <p>{format!("This is record {n} with more than one line")}</p>
                <p><a tabindex="-1" href="/">{"This is a link"}</a></p>
                <input tabindex="-1" class="pwt-input"/>
            </>
        }
    });

    let header3 = DataTableColumn::new("TESTHEADER3")
        .percentage(20)
        .render(|n: &usize| html! {format!("This is record {n} (a longer line)")});

    let extract_key = ExtractKeyFn::new(|n: &usize| (*n).into());
    let extract_key2 = extract_key.clone();

    let store = use_state(move || {
        let mut data: Vec<usize> = Vec::new();
        for i in 0..30_000 {
            data.push(i);
        }

        let store = Store::with_extract_key(extract_key2);
        store.write().set_data(data);
        store
    });

    let group3 = DataTableHeaderGroup::new("Group3")
        .with_child(header2.clone())
        .with_child(header3.clone());

    let group2 = DataTableHeaderGroup::new("Group2")
        .with_child(header1.clone())
        .with_child(group3)
        .with_child(header3.clone());

    let columns: Rc<Vec<DataTableHeader<usize>>> = Rc::new(vec![
        header0.clone().into(),
        header1.clone().into(),
        group2.into(),
        header2.clone().into(),
    ]);

    let multiselect = use_state(|| true);
    let selection = use_selection(|| {
        Selection::new().on_select(|sel: Selection| {
            let keys = sel.selected_keys();
            if keys.len() > 10 {
                log::info!("Current Selection: {} records", keys.len());
            } else {
                log::info!("Current Selection: {:?}", sel.selected_keys());
            }
        })
    });

    let selection = selection.multiselect(*multiselect);

    let borderless = use_state(|| false);
    let bordered = use_state(|| true);
    let show_headers = use_state(|| true);
    let striped = use_state(|| true);
    let hover = use_state(|| true);
    let mmode = use_state(|| MultiSelectMode::Default);

    Column::new()
        .class(Flex::Fill)
        .class(Overflow::Auto)
        .padding(2)
        .gap(2)
        .with_child(
            Panel::new()
                .title("DataTable Options")
                .border(true)
                .with_child(
                    InputPanel::new()
                        .padding(2)
                        .with_field(
                            "Borderless",
                            Checkbox::new()
                                .name("borderless")
                                .checked(*borderless)
                                .on_change({
                                    let borderless = borderless.clone();
                                    move |v| borderless.set(v)
                                }),
                        )
                        .with_field(
                            "Bordered",
                            Checkbox::new()
                                .name("bordered")
                                .checked(*bordered)
                                .on_change({
                                    let bordered = bordered.clone();
                                    move |v| bordered.set(v)
                                }),
                        )
                        .with_field(
                            "Show Headers",
                            Checkbox::new()
                                .name("header")
                                .checked(*show_headers)
                                .on_change({
                                    let show_headers = show_headers.clone();
                                    move |v| show_headers.set(v)
                                }),
                        )
                        .with_right_field(
                            "Striped",
                            Checkbox::new().name("striped").checked(*striped).on_change({
                                let striped = striped.clone();
                                move |v| striped.set(v)
                            }),
                        )
                        .with_right_field(
                            "Hover",
                            Checkbox::new().name("hover").checked(*hover).on_change({
                                let hover = hover.clone();
                                move |v| hover.set(v)
                            }),
                        )
                        .with_right_field(
                            "Multiselect",
                            Checkbox::new()
                                .name("multiselect")
                                .checked(*multiselect)
                                .on_change({
                                    let multiselect = multiselect.clone();
                                    move |v| multiselect.set(v)
                                }),
                        )
                        .with_right_field(
                            "Multiselect Mode",
                            Combobox::new()
                                .with_item("Default")
                                .with_item("Simple")
                                .default("Default")
                                .required(true)
                                .name("multiselect")
                                .on_change({
                                    let mmode = mmode.clone();
                                    move |v: String| {
                                        let v = match v.as_str() {
                                            "Default" => MultiSelectMode::Default,
                                            "Simple" => MultiSelectMode::Simple,
                                            _ => MultiSelectMode::Default,
                                        };

                                        mmode.set(v);
                                    }
                                }),
                        ),
                ),
        )
        .with_child(
            DataTable::new(columns, (*store).clone())
                .class(Flex::Fill)
                .class(Overflow::Auto)
                .border(true)
                .bordered(*bordered)
                .show_header(*show_headers)
                .borderless(*borderless)
                .min_row_height(110)
                .striped(*striped)
                .hover(*hover)
                .multiselect_mode(*mmode)
                .autoselect(false)
                //.cell_class("pwt-p-2 pwt-fs-4")
                //.vertical_align("middle")
                //.min_row_height(100)
                //.header_class("pwt-scheme-primary-container pwt-fs-4")
                .selection(selection),
        )
        .into()
}
