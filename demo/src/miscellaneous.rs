use yew::prelude::*;

use pwt::prelude::*;

use pwt::touch::{SnackBar, SnackBarController, SnackBarManager};
use pwt::widget::{
    Button, Column, Container, Mask, Meter, MiniScroll, MiniScrollMode, Panel, Progress, Row,
};

fn mini_scoll_demo1() -> Html {
    let mut content = Row::new().padding(2).gap(2);

    for i in 0..10 {
        content.add_child(Button::new(format!("Button {i}")));
    }

    Panel::new()
        .border(true)
        .title("MiniScroll mode 'MiniScrollMode::Native' (default)")
        .with_child(MiniScroll::new(content))
        .into()
}

fn mini_scoll_demo2() -> Html {
    let mut content = Row::new().padding(2).gap(2);

    for i in 0..10 {
        content.add_child(Button::new(format!("Button {i}")));
    }

    Panel::new()
        .border(true)
        .title("MiniScroll mode 'MiniScrollMode::Arrow'")
        .with_child(MiniScroll::new(content).scroll_mode(MiniScrollMode::Arrow))
        .into()
}

fn load_mask_demo() -> Html {
    Panel::new()
        .border(true)
        .title("Load Mask")
        .with_child(
            Mask::new(
                Container::new()
                    .padding(2)
                    .min_height(50)
                    .with_child("You can  set a load mask above any content."),
            )
            .visible(true),
        )
        .into()
}

fn load_with_progress_demo() -> Html {
    Panel::new()
        .border(true)
        .title("Use progress bar as bottom border")
        .with_child(Progress::new())
        .with_child(
            Container::new()
                .padding(2)
                .min_height(50)
                .with_child("You can use the progess indicator like an additional border."),
        )
        .into()
}

#[function_component]
pub fn MiscellaneousDemo() -> Html {
    let meter = Meter::new()
        .min(200.0)
        .max(500.0)
        .low(350.0)
        .high(450.0)
        .optimum(200.0);

    let snackbar_controller = use_state(|| {
        let controller = SnackBarController::new();
        controller.show_snackbar(
            SnackBar::new()
                .action_label("Okay, got it")
                .message("This is a SnackBar."),
        );
        controller
    });

    Column::new()
        .class(pwt::css::FlexFit)
        .class("pwt-content-spacer-padding")
        .gap(2)
        .with_child("Progress indicator (ongoing progress)")
        .with_child(Progress::new())
        .with_child("Progress indicator (value)")
        .with_child(Progress::new().max(60.0).value(50.0))
        .with_child(load_with_progress_demo())
        .with_child("Meter (min 200, max 500, low 350, height 450, optimum 200)")
        .with_child(meter.clone().value(220.0))
        .with_child(meter.clone().value(320.0))
        .with_child(meter.clone().value(420.0))
        .with_child(meter.clone().value(460.0))
        .with_child("Meter with text (min 200, max 500, low 350, height 450, optimum 200)")
        .with_child(
            meter
                .clone()
                .value(220.0)
                .render_text(|v: &f32| v.to_string()),
        )
        .with_child(
            meter
                .clone()
                .value(320.0)
                .render_text(|v: &f32| v.to_string()),
        )
        .with_child(
            meter
                .clone()
                .value(420.0)
                .render_text(|v: &f32| v.to_string()),
        )
        .with_child(
            meter
                .clone()
                .value(460.0)
                .render_text(|v: &f32| v.to_string()),
        )
        .with_child(mini_scoll_demo1())
        .with_child(mini_scoll_demo2())
        .with_child(load_mask_demo())
        .with_child(Button::new("Show Snackbar").onclick({
            let controller = (*snackbar_controller).clone();
            move |_| {
                controller.show_snackbar(
                    SnackBar::new()
                        .message("You pressed the show snackbar button.")
                        .duration(2000),
                );
            }
        }))
        .with_child(SnackBarManager::new().controller((*snackbar_controller).clone()))
        .into()
}
