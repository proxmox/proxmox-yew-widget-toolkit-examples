use std::rc::Rc;

use serde_json::json;

use yew::prelude::*;
use yew::virtual_dom::Key;

use pwt::prelude::*;

//use pwt::props::Selection2;
use pwt::state::{TreeStore, DataStore, DataNode, use_tree_store};
use pwt::widget::Column;
use pwt::widget::data_table2::*;

fn test0() {
    use pwt::state::Store;

    let store: Store<usize> = Store::with_extract_key(
        |d: &usize| Key::from(*d)
    );


    store.set_data(vec![4,1,3,2]);

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

        let mut root = tree.append(5);
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
    let tree: TreeStore<String> = TreeStore::with_extract_key(
        |d: &String| Key::from(d.clone())
    );

    { // limit scope
        let mut tree = tree.write();
        let mut record1 = tree.append(String::from("Record1"));
        record1.set_expanded(true);
        record1.append(String::from("Record2"));
        record1.append(String::from("Record4"));
        let mut record3 = record1.append(String::from("Record3"));
        record3.set_expanded(true);
        record3.append(String::from("Redord3-1"));
        record3.append(String::from("Redord3-3"));
        record3.append(String::from("Redord3-2"));
    }

    tree
}


#[function_component]
pub fn TreeDemo() -> Html {

    test0();
    test1();

    let store = use_tree_store(|| create_tree_store());

    let header1 = DataTableColumn::new("Record")
        .fixed(200)
        .sorter(String::cmp)
        //.sort_order(false)
        //.render(|item: &String| html!{item.to_string()});
        .on_cell_click({
            let store = store.clone();
            move |key| {
                log::info!("CLICK TEST {}", key);
                store.toggle_expanded(&key);
            }
        })
        .render_node(move |item: &dyn DataNode<String>| {
            let indent: Html = (0..item.level())
                .map(|_| html!{ <span aria-hidden="" class="pwt-ps-4"/> })
                .collect();


            html!{<>{indent.clone()}{item.record().to_string()}</>}
        });

    let _tree = json!({
        "record": "Redord1",
        "expanded": true,
        "children": [
            { "record": "Redord2" },
            {
                "record": "Redord3",
                "expanded": true,
                "children": [
                    { "record": "Redord3-1" },
                    { "record": "Redord3-2" },
                    { "record": "Redord3-3" },
                ],
            },
            { "record": "Redord4" },
        ],
    });


    let columns: Rc<Vec<Header<String>>> = Rc::new(vec![
        header1.clone().into(),
    ]);

    let table1 = DataTable::new(columns, store.clone())
        .class("pwt-fit pwt-border")
        .bordered(true);

    Column::new()
        .class("pwt-fit")
        .padding(4)
        .gap(4)
        .with_child(table1)
        .into()
}
