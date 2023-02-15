use yew::prelude::*;

use pwt::prelude::*;
use pwt::widget::{Column, Meter, Progress};

#[function_component]
pub fn MiscellaneousDemo() -> Html {

    let meter = Meter::new()
        .min(200.0)
        .max(500.0)
        .low(350.0)
        .high(450.0)
        .optimum(200.0);

    Column::new()
        .padding(2)
        .gap(2)
        .with_child("Progress indicator (ongoing progress)")
        .with_child(Progress::new())
        .with_child("Progress indicator (value)")
        .with_child(Progress::new().max(60.0).value(50.0))
        .with_child("Meter (min 200, max 500, low 350, height 450, optimum 200)")
        .with_child(meter.clone().value(220.0))
        .with_child(meter.clone().value(320.0))
        .with_child(meter.clone().value(420.0))
        .with_child(meter.clone().value(460.0))
        .into()
}
