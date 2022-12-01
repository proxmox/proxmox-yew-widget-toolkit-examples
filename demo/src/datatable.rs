use std::rc::Rc;

use yew::prelude::*;

use pwt::prelude::*;

use pwt::props::ExtractKeyFn;
use pwt::state::{use_selection, Selection2, Store};
use pwt::widget::Column;
use pwt::widget::data_table2::*;

#[function_component]
pub fn DataTableDemo() -> Html {

    let header0 = DataTableColumn::new("selection indicator")
        .width("2.5em")
        .resizable(false)
        .show_menu(false)
        .render_header(|args: &mut DataTableHeaderRenderArgs<usize>| {
            html!{}
        })
        .render_cell(pwt::widget::data_table2::render_selection_indicator);

    let header1 = DataTableColumn::new("RecordId")
        .fixed(90)
        .sorter(usize::cmp)
        .sort_order(false)
        .render(|n: &usize| html!{n.to_string()});


    let header2 = DataTableColumn::new("TEHE2")
        .flex(1)
        .render(|n: &usize| html!{
            <>
                <h3>{"This is a header"}</h3>
                <p>{format!("This is record {n} with more than one line")}</p>
                <p><a tabindex="-1" href="/">{"This is a link"}</a></p>
                <input tabindex="-1" class="pwt-input"/>
            </>
        });

    let header3 = DataTableColumn::new("TESTHEADER3")
        .percentage(20)
        .render(|n: &usize| html!{format!("This is record {n} (a longer line)")});

    let extract_key = ExtractKeyFn::new(|n: &usize| (*n).into());
    let extract_key2 = extract_key.clone();

    let store = use_state(move || {
        let mut data: Vec<usize> = Vec::new();
        for i in 0..30_000 { data.push(i); }

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

    let selection = use_selection(|| {
        let selection = Selection2::new()
            .multiselect(true)
            .on_select(|sel: Selection2| {
                let keys = sel.selected_keys();
                if keys.len() > 10 {
                    log::info!("Current Selection: {} records", keys.len());
                } else {
                    log::info!("Current Selection: {:?}", sel.selected_keys());
                }
            });
        //selection.select(2);
        //selection.select(4);
        selection
    });

    let table1 = DataTable::new(columns, (*store).clone())
        .class("pwt-fit pwt-border")
        .min_row_height(110)
    //.cell_class("pwt-p-2 pwt-fs-4")
    //.vertical_align("middle")
    //.min_row_height(100)
    //.header_class("pwt-scheme-primary-container pwt-fs-4")
        .bordered(true)
        .selection(selection);

    Column::new()
        .class("pwt-fit")
        .padding(4)
        .gap(4)
        .with_child(table1)
        .into()
}
