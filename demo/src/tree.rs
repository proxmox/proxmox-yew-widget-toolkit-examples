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

    let header1 = DataTableColumn::new("Record")
        .fixed(200)
        .sorter(String::cmp)
        //.sort_order(false)
        //.render(|item: &String| html!{item.to_string()});
        .on_cell_click({
            let store = store.clone();
            move |event: DataTableMouseEvent| {
                log::info!("CLICK TEST {}", event.key);
                if let Some(mut node) = store.write().lookup_node_mut(&event.key) {
                    node.set_expanded(!node.expanded());
                }
            }
        })
        .render_node(move |item: &dyn DataNode<String>| {
            let indent: Html = (0..item.level())
                .map(|_| html!{ <span aria-hidden="" class="pwt-ps-4"/> })
                .collect();


            html!{<>{indent.clone()}{item.record().to_string()}</>}
        });



    let columns: Rc<Vec<DataTableHeader<String>>> = Rc::new(vec![
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
