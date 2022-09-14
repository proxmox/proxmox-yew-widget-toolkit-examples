use std::rc::Rc;

use yew::prelude::*;
use pwt::prelude::*;

use pwt::widget::{Button, Column, Container, TabPanel};
use pwt::widget::data_table2::*;

#[function_component]
pub fn DataTableDemo() -> Html {

    let header1 = DataTableColumn::new("RecordId")
        .width(80)
        .render(|n: &usize| html!{n.to_string()});


    let header2 = DataTableColumn::new("TEHE2")
        .flex(1)
        .render(|n: &usize| html!{
            <>
                <h3>{"This is a header"}</h3>
                <p>{format!("This is record {n} with more than one line")}</p>
                <input/>

            </>
        });

    let header3 = DataTableColumn::new("TESTHEADER3")
        .render(|n: &usize| html!{format!("This is record {n} (a longer line)")});

    thread_local!{
        static DATA: Rc<Vec<usize>> = Rc::new({
            let mut data = Vec::new();
            for i in 0..30 { data.push(i); }
            data
        });
    }

    let data = DATA.with(Rc::clone);

    let group3 = HeaderGroup::new()
        .content(html!{"Group3"})
        .with_child(header2.clone())
        .with_child(header3.clone());

    let group2 = HeaderGroup::new()
        .content(html!{"Group2"})
        .with_child(header1.clone())
        .with_child(group3)
        .with_child(header3.clone());

    let columns: Rc<Vec<Header<usize>>> = Rc::new(vec![
        header1.clone().into(),
        group2.into(),
        header2.clone().into(),
    ]);

    let table1 = DataTable::new(columns)
        .class("pwt-fit pwt-border")
    //.cell_class("pwt-truncate pwt-p-2")
    //.vertical_align("middle")
    //.min_row_height(100)
        .bordered(true)
        .data(data);

    Column::new()
        .class("pwt-fit")
        .padding(4)
        .gap(4)
        .with_child(table1)
        .into()
}
