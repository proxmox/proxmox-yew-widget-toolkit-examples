use std::rc::Rc;

use serde::Deserialize;
use yew::prelude::*;
use yew::virtual_dom::Key;

use pwt::prelude::*;

use pwt::props::{LoadCallback, IntoLoadCallback, ExtractPrimaryKey};
use pwt::state::{use_selection, Selection2, Store};
use pwt::widget::Column;
use pwt::widget::data_table2::*;

#[derive(Deserialize)]
struct Record {
    title: String,
    year: usize,
    cast: Vec<String>,
    genres: Vec<String>,
}

impl ExtractPrimaryKey for Record {
    fn extract_key(&self) -> Key {
        Key::from(format!("{}{}", self.year, self.title))
    }
}

fn load_movie_list(store: &Store<Record>) {

    let url = "https://raw.githubusercontent.com/prust/wikipedia-movie-data/master/movies.json";

    let store = store.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let loader: LoadCallback::<Vec<Record>> = url.into_load_callback().unwrap();
        let res = loader.apply().await;
        match res {
            Ok(data) => {
                log::info!("Load sucessful - got {} records", data.len());
                // Seems the dataset contains duplicate entries.
                // We need to remove them to have unique primary keys.
                let mut new = Vec::new();
                let mut found = std::collections::HashSet::new();
                for record in data {
                    let key = record.extract_key();
                    if found.contains(&key) {
                        log::info!("Skip duplicate {}", key);
                        continue;
                    }
                    found.insert(key);
                    new.push(record);
                }
                store.write().set_data(new);
            }
            Err(err) => {
                log::error!("Load failed - {}", err);
            }
        }
    });
}

thread_local!{
    static COLUMNS: Rc<Vec<DataTableHeader<Record>>> = Rc::new(vec![
        DataTableColumn::new("Title")
            .sorter(|r1: &Record, r2: &Record| r1.title.cmp(&r2.title))
            .render(|r: &Record| html!{r.title.clone()})
            .into(),
        DataTableColumn::new("Genres")
            .fixed(100)
            .render(|r: &Record| {
                let text = r.genres.iter().fold(String::new(), |mut acc, name| {
                    if !acc.is_empty() { acc.push_str(", "); }
                    acc.push_str(name);
                    acc
                });
                html!{text}
            })
            .into(),
        DataTableColumn::new("Year")
            .fixed(90)
            .sorter(|r1: &Record, r2: &Record| r1.year.cmp(&r2.year))
            .sort_order(true)
            .render(|r: &Record| html!{r.year.to_string()})
            .into(),
        DataTableColumn::new("Cast")
            .fixed(200)
            .render(|r: &Record| {
                let text = r.cast.iter().fold(String::new(), |mut acc, name| {
                    if !acc.is_empty() { acc.push_str(", "); }
                    acc.push_str(name);
                    acc
                });
                html!{text}
            })
            .into(),
    ]);
}

#[function_component]
pub fn MovieDemo() -> Html {

    let columns = COLUMNS.with(Rc::clone);

    let store = use_state(move || {
        let store = Store::new();
        load_movie_list(&store);
        store
    });

    let selection = use_selection(|| {
        let selection = Selection2::new();
        selection
    });

    let table = DataTable::new(columns, (*store).clone())
        .class("pwt-fit pwt-border")
        .bordered(true)
        .selection(selection);

    Column::new()
        .class("pwt-fit")
        .padding(4)
        .gap(4)
        .with_child(table)
        .into()
}
