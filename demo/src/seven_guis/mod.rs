mod counter;
pub use counter::Counter;

mod temperature_converter;
pub use temperature_converter::TemperatureConverter;
use yew::prelude::*;

mod flight_booker;
pub use flight_booker::FlightBooker;

use pwt::prelude::*;
use pwt::widget::Column;

#[function_component]
pub fn SevenGuisDemo() -> Html {

    Column::new()
        .padding(2)
        .gap(2)
        .with_child(html!{<Counter/>})
        .with_child(html!{<TemperatureConverter/>})
        .with_child(html!{<FlightBooker/>})
        .into()
}
