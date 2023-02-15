use yew::prelude::*;

use pwt::prelude::*;
use pwt::widget::{Column, Progress};

#[function_component]
pub fn MiscellaneousDemo() -> Html {
    Column::new()
        .padding(2)
        .gap(2)
        .with_child("Progress indicator (ongoing progress)")
        .with_child(Progress::new())
        .with_child("Progress indicator (value)")
        .with_child(Progress::new().max(60.0).value(50.0))
        .into()
}
