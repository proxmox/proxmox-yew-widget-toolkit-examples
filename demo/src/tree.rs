use std::rc::Rc;

use serde::Deserialize;
use serde_json::json;

use yew::prelude::*;
use yew::virtual_dom::Key;

use pwt::prelude::*;

//use pwt::props::Selection2;
use pwt::css::{Flex, Overflow};
use pwt::props::ExtractPrimaryKey;
use pwt::state::{use_tree_store, TreeStore};
use pwt::widget::{data_table::*, Fa, Tooltip};
use pwt::widget::{ActionIcon, Button, Row};

fn test0() {
    use pwt::state::Store;

    let store: Store<usize> = Store::with_extract_key(|d: &usize| Key::from(*d));

    store.write().set_data(vec![4, 1, 3, 2]);

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
    let tree: TreeStore<usize> = TreeStore::with_extract_key(|d: &usize| Key::from(*d));

    {
        // limit scope
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

#[derive(Deserialize, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "kebab-case")]
enum PveType {
    Root,
    Node,
    Ct,
    Vm,
}

#[derive(Deserialize, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "kebab-case")]
enum PveStatus {
    Online,
    Offline,
    Running,
    Stopped,
}

#[derive(Deserialize, PartialEq, Clone, PartialOrd)]
struct PveTreeEntry {
    id: String,
    text: String,
    #[serde(alias = "iconCls")]
    icon_class: Option<String>,
    #[serde(alias = "type")]
    node_type: PveType,
    status: Option<PveStatus>,
}

impl ExtractPrimaryKey for PveTreeEntry {
    fn extract_key(&self) -> Key {
        Key::from(self.id.clone())
    }
}

impl PveTreeEntry {
    fn icon(&self) -> &'static str {
        match self.node_type {
            PveType::Root => "server",
            PveType::Node => "building",
            PveType::Ct => "cube",
            PveType::Vm => "desktop",
        }
    }

    fn color(&self) -> &'static str {
        match self.status {
            Some(PveStatus::Online) => "",
            Some(PveStatus::Offline) => "#FF6C59",
            Some(PveStatus::Running) => "#21BF4b",
            Some(PveStatus::Stopped) => "",
            None => "",
        }
    }
}

fn create_tree_store() -> TreeStore<PveTreeEntry> {
    use pwt::state::SlabTree;

    let tree_data = json!({
        "record": {
            "id": "root",
            "text": "Datacenter",
            "type": "root",
        },
        "expanded": true,

        "children": [{
            "record":  {
                "id": "nodes/pve-node-1",
                "text": "pve-node-1",
                "status": "online",
                "type": "node"
            },

            "expanded": true,

            "children": [{
                "record":  {
                    "id": "vms/1000",
                    "text": "VM 1000",
                    "status": "running",
                    "type": "vm"
                },
            },
            {
                "record":  {
                    "id": "ct/500",
                    "text": "CT 500",
                    "status": "stopped",
                    "type": "ct"
                },
            },
            {
                "record":  {
                    "id": "vms/100",
                    "text": "VM 100",
                    "status": "stopped",
                    "type": "vm"
                },
            }],
        },
        {
            "record":  {
                "id": "nodes/pve-node-2",
                "text": "pve-node-2",
                "status": "online",
                "type": "node"
            },

            "expanded": true,

            "children": [{
                "record":  {
                    "id": "vms/102",
                    "text": "VM 102",
                    "status": "stopped",
                    "type": "vm"
                },
            },
            {
                "record":  {
                    "id": "ct/103",
                    "text": "CT 103",
                    "status": "running",
                    "type": "ct"
                },
            },
            {
                "record":  {
                    "id": "ct/104",
                    "text": "CT 100",
                    "status": "running",
                    "type": "ct"
                },
            }],
        },{
            "record": {
                "id": "nodes/pve-node-3",
                "text": "pve-node-3",
                "status": "offline",
                "type": "node"
            }
        }]
    });

    let tree_root: SlabTree<PveTreeEntry> = match serde_json::from_value(tree_data) {
        Ok(root) => root,
        Err(err) => {
            log::error!("error during deserialize: {err}");
            return TreeStore::new();
        }
    };

    let tree: TreeStore<PveTreeEntry> = TreeStore::new();

    tree.write().set_root_tree(tree_root);
    tree.write()
        .root_mut()
        .unwrap()
        .visit_mut(&mut |node| node.set_expanded(true));
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
        //.sort_order(false)
        //.render(|item: &String| html!{item.to_string()});
        .tree_column(store.clone())
        .render_cell(move |args: &mut DataTableCellRenderArgs<PveTreeEntry>| {
            match args.record().node_type {
                PveType::Root | PveType::Node => {
                    args.set_attribute("colspan", "3");
                }
                _ => {}
            }
            let record = args.record();
            let icon = Fa::new(record.icon())
                .fixed_width()
                .padding_end(2)
                .style("color", record.color());
            html! {<>{icon} {args.record().text.to_string()}</>}
        });

    let column2 = DataTableColumn::new("Buttons").render(|record: &PveTreeEntry| {
        let running = record.status == Some(PveStatus::Running);
        Row::new()
            .gap(2)
            .class("pwt-justify-content-between")
            .with_child(
                Button::new("Start")
                    .icon_class("fa fa-play")
                    .disabled(running)
                    .tabindex(-1)
                    .onclick({
                        let record = record.clone();
                        move |_| log::info!("Pressed Start on '{}'", record.text)
                    }),
            )
            .with_child(
                Button::new("Stop")
                    .icon_class("fa fa-stop")
                    .disabled(!running)
                    .tabindex(-1)
                    .onclick({
                        let record = record.clone();
                        move |_| log::info!("Pressed Stop on '{}'", record.text)
                    }),
            )
            .with_child(
                Button::new("Shutdown")
                    .icon_class("fa fa-power-off")
                    .disabled(!running)
                    .tabindex(-1)
                    .onclick({
                        let record = record.clone();
                        move |_| log::info!("Pressed Shutdown on '{}'", record.text)
                    }),
            )
            .into()
    });

    let column3 = DataTableColumn::new("Actions").render(|record: &PveTreeEntry| {
        Row::new()
            .gap(2)
            .class("pwt-justify-content-evenly")
            .with_child(
                Tooltip::new(
                    ActionIcon::new("fa fa-history")
                        .aria_label("snapshot")
                        .on_activate({
                            let record = record.clone();
                            move |_| log::info!("Pressed Icon 'snapshot' on '{}'", record.text)
                        }),
                )
                .tip(tr!("Snapshot")),
            )
            .with_child(
                Tooltip::new(
                    ActionIcon::new("fa fa-terminal")
                        .aria_label("console")
                        .on_activate({
                            let record = record.clone();
                            move |_| log::info!("Pressed Icon 'console' on '{}'", record.text)
                        }),
                )
                .tip(tr!("Console")),
            )
            .into()
    });

    let columns: Rc<Vec<DataTableHeader<PveTreeEntry>>> = Rc::new(vec![
        header0.clone().into(),
        header1.clone().into(),
        column2.clone().into(),
        column3.clone().into(),
    ]);

    DataTable::new(columns, store.clone())
        .class(Flex::Fill)
        .class(Overflow::Auto)
        .margin(2)
        .border(true)
        .bordered(false)
        .borderless(true)
        .hover(true)
        .into()
}
