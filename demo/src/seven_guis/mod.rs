mod counter;
pub use counter::SevenGuisCounter;

use yew::prelude::*;

use pwt::prelude::*;
use pwt::widget::{ActionIcon, Button, Column, Row};

#[function_component]
pub fn SevenGuisDemo() -> Html {

    Column::new()
        .padding(2)
        .gap(2)
        .with_child(html!{<SevenGuisCounter/>})
        .into()
}
