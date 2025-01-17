mod circle_drawer;
pub use circle_drawer::CircleDrawer;

mod counter;
pub use counter::Counter;

mod temperature_converter;
pub use temperature_converter::TemperatureConverter;

mod temperature_converter2;
pub use temperature_converter2::TemperatureConverter2;

use yew::prelude::*;

mod flight_booker;
pub use flight_booker::FlightBooker;

use pwt::prelude::*;
use pwt::widget::Column;

#[function_component]
pub fn SevenGuisDemo() -> Html {
    Column::new()
        .class("pwt-content-spacer")
        .class("pwt-align-items-start")
        .class(pwt::css::FlexFit)
        .with_child(html! {<CircleDrawer/>})
        .with_child(html! {<Counter/>})
        .with_child(html! {<TemperatureConverter/>})
        .with_child(html! {<TemperatureConverter2/>})
        .with_child(html! {<FlightBooker/>})
        .into()
}
