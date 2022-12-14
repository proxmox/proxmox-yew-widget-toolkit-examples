use std::rc::Rc;

use serde_json::json;

use yew::prelude::*;
use yew::virtual_dom::Key;

use pwt::prelude::*;

//use pwt::props::Selection2;
use pwt::state::{TreeStore, DataStore, use_tree_store};
use pwt::widget::{ActionIcon, Button, Column, Row};
use pwt::widget::data_table::*;

fn test0() {
    use pwt::state::Store;

    let store: Store<usize> = Store::with_extract_key(
        |d: &usize| Key::from(*d)
    );


    store.write().set_data(vec![4,1,3,2]);

    //let second = &store.read()[1];


    for item in store.read().iter() {
        log::info!("GOT ITEM {}", item);
    }

    store.write().sort();

    for (_cursor, node) in store.filtered_data() {
        log::info!("GOT node {}", *node.record());
    }

    store.write().push(5);

    for (_cursor, node) in store.filtered_data() {
        log::info!("GOT node {}", *node.record());
    }

}

fn test1() {

    let tree: TreeStore<usize> = TreeStore::with_extract_key(
        |d: &usize| Key::from(*d)
    );

    { // limit scope
        let mut tree = tree.write();

        let mut root = tree.set_root(5);
        assert!(*root.record() == 5);
        assert!(*root.append(1).record() == 1);
        assert!(*root.append(2).record() == 2);

        // fixme: *root.record() = 6;
        // assert!(*root.record() == 6);
    }

        //let mut root: Box<dyn DataNode<usize>> = Box::new(root);
    //*root.record() = 6;
    //assert!(*root.record() == 6);

}

fn create_tree_store() -> TreeStore<String> {
    use pwt::state::SlabTree;

    let tree_data = json!({
        "record": "Redord1",
        "expanded": true,
        "children": [
            { "record": "Redord2" },
            {
                "record": "Redord3",
                "expanded": false,
                "children": [
                    { "record": "Redord3-1" },
                    { "record": "Redord3-2" },
                    { "record": "Redord3-3" },
                ],
            },
            { "record": "Redord4" },
        ],
    });

    let tree_root: SlabTree<String> = serde_json::from_value(tree_data).unwrap();

    let tree: TreeStore<String> = TreeStore::with_extract_key(
        |d: &String| Key::from(d.clone())
    );

    tree.write().set_root_tree(tree_root);
    tree.write().root_mut().unwrap().visit_mut(&mut |node| node.set_expanded(true));
    tree
}

#[function_component]
pub fn TreeDemo() -> Html {

    if false {
        test0();
        test1();
    }

    let store = use_tree_store(|| create_tree_store());

    let header0 = DataTableColumn::row_number();

    let header1 = DataTableColumn::new("Record")
        .fixed(200)
        .sorter(String::cmp)
        //.sort_order(false)
        //.render(|item: &String| html!{item.to_string()});
        .on_cell_click({
            let store = store.clone();
            move |event: &mut DataTableMouseEvent| {
                log::info!("CLICK TEST {}", event.record_key);
                if let Some(mut node) = store.write().lookup_node_mut(&event.record_key) {
                    node.set_expanded(!node.expanded());
                }
            }
        })
        .render_cell(move |args: &mut DataTableCellRenderArgs<String>| {
            let is_leaf = args.node().is_leaf();
            if !is_leaf {
                args.set_attribute("colspan", "3");
            }
            render_tree_node(args, |record| (None, record.to_string()))
        });


    let column2 = DataTableColumn::new("Buttons")
        .fixed(130)
        .render(|record: &String| {
            Row::new()
                .gap(2)
                .class("pwt-justify-content-between")
                .with_child(
                    Button::new("B1")
                        .tabindex(-1)
                        .onclick({
                            let record = record.clone();
                            move |_| log::info!("Pressed B1 on '{record}'")
                        })
                )
                .with_child(
                    Button::new("B2")
                        .tabindex(-1)
                        .onclick({
                            let record = record.clone();
                            move |_| log::info!("Pressed B2 on '{record}'")
                        })
                )
                .with_child(
                    Button::new("B3")
                        .tabindex(-1)
                        .onclick({
                            let record = record.clone();
                            move |_| log::info!("Pressed B3 on '{record}'")
                        })
                )
                .into()
        });

    let column3 = DataTableColumn::new("Actions")
        .fixed(80)
        .render(|record: &String| {
            Row::new()
                .gap(2)
                .class("pwt-justify-content-evenly")
                .with_child(
                    ActionIcon::new("fa fa-bold")
                        .aria_label("bold")
                        .on_activate({
                            let record = record.clone();
                            move |_| log::info!("Pressed Icon 'bold' on '{record}'")
                        })
                )
                .with_child(
                    ActionIcon::new("fa fa-italic")
                        .aria_label("italic")
                        .on_activate({
                            let record = record.clone();
                            move |_| log::info!("Pressed Icon 'italic' on '{record}'")
                        })
                )
                .with_child(
                    ActionIcon::new("fa fa-font")
                        .aria_label("font")
                        .on_activate({
                            let record = record.clone();
                            move |_| log::info!("Pressed Icon 'font' on '{record}'")
                        })
                )
                .into()
        });

    let columns: Rc<Vec<DataTableHeader<String>>> = Rc::new(vec![
        header0.clone().into(),
        header1.clone().into(),
        column2.clone().into(),
        column3.clone().into(),
    ]);

    let table1 = DataTable::new(columns, store.clone())
        .class("pwt-fit pwt-border")
        .row_render_callback(move |args: &mut DataTableRowRenderArgs<String>| {
            args.set_attribute("style", "color:red;");
        })
        .bordered(true);

    Column::new()
        .class("pwt-fit")
        .padding(4)
        .gap(4)
        .with_child(table1)
        .into()
}
