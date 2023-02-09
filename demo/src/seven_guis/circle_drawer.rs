use pwt::prelude::*;

use pwt::widget::{Button, Container, Dialog, Panel, Toolbar};
use pwt::widget::canvas::{Canvas, Circle};
use pwt::widget::form::Field;

// TODO: implement Undo/Redo functionality

pub enum Msg {
    CreateCircle(i32, i32),
    CircleClick(usize),
    ChangeRadius(usize),
    Select(usize),
    CloseDialog,
}

struct CircleState {
    x: i32,
    y: i32,
    r: usize,
}

pub struct CircleDrawer {
    circles: Vec<CircleState>,
    selected: usize,
    show_resize_dialog: bool,
}

impl CircleDrawer {

    fn create_resize_dialog(&self, ctx: &Context<Self>) -> Dialog {

        let circle = &self.circles[self.selected];
        
        Dialog::new(format!(
            "Adjust diameter of circle at ({}, {})",
            circle.x,
            circle.y,
        ))
            .on_close(ctx.link().callback(|_| Msg::CloseDialog))
            .with_child(
                Field::new()
                    .number(10.0, 200.0, 1.0)
                    .input_type("range")
                    .value(format!("{}", circle.r))
                    .on_change(ctx.link().callback(|v: String| {
                        Msg::ChangeRadius(v.parse::<f32>().unwrap() as usize)
                    }))   
            )
    }
    
    fn create_canvas(&self, ctx: &Context<Self>) -> Canvas {
        let mut canvas = Canvas::new()
            .class("pwt-w-100")
            .class("pwt-bg-color-neutral")
            .border(true)
            .height(300)
            .oncontextmenu(Callback::from(move |e: MouseEvent| {
                e.prevent_default();
            }))
            .onclick(ctx.link().callback(|e: MouseEvent| {
                Msg::CreateCircle(e.offset_x(), e.offset_y())
            }));

        for (i, circle) in self.circles.iter().enumerate() {
            canvas.add_child(
                Circle::new()
                    .position(circle.x, circle.y)
                    .r(circle.r)
                    .stroke("black")
                    .stroke_width(2)
                    .fill(if self.selected == i { "grey" } else { "white" })
                    .onclick(Callback::from(move |e: MouseEvent| {
                        e.stop_propagation();
                    }))
                    .oncontextmenu(ctx.link().callback(move |e: MouseEvent| {
                        e.prevent_default();
                        Msg::CircleClick(i)
                    }))
                    .onmouseover(ctx.link().callback(move |_| {
                        Msg::Select(i)
                    }))
            );
        }

        canvas
    }
    
    fn create_toolbar(&self, _ctx: &Context<Self>) -> Toolbar {
        Toolbar::new()
            .class("pwt-bg-color-surface")
            .with_flex_spacer()
            .with_child(
                Button::new("Undo")
            )
            .with_child(
                Button::new("Redo")
            )
            .with_flex_spacer()
    }
}

impl Component for CircleDrawer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            circles: Vec::new(),
            selected: 0,
            show_resize_dialog: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CloseDialog => {
                self.show_resize_dialog = false; 
            }
            Msg::Select(index) => {
                self.selected = index;
            }
            Msg::CircleClick(index) => {
                self.selected = index;
                self.show_resize_dialog = true;
            }
            Msg::CreateCircle(x, y) => {
                log::info!("CC {x} {y}");
                
                self.circles.push(CircleState {
                    x, y, r: random_radius(),
                })
            }
            Msg::ChangeRadius(r) => {
                let circle = &mut self.circles[self.selected];
                circle.r = r;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let dialog = self.show_resize_dialog
            .then(|| self.create_resize_dialog(ctx));
        
        Panel::new()
            .title("Circle Drawer")
            .with_child(self.create_toolbar(ctx))
            .with_child(
                Container::new()
                    .class("pwt-bg-color-surface")
                    .padding_x(2)
                    .padding_bottom(2)
                    .with_child(self.create_canvas(ctx))
            )
            .with_optional_child(dialog)
            .into()
    }
}
   
fn random_radius() -> usize {
    use std::cell::RefCell;
    
    thread_local!{
        static SEED: RefCell<usize> = RefCell::new(13456374);
    }

    let rand = SEED.with(|cell| {
        let mut seed = cell.borrow_mut();
        *seed = ((*seed).wrapping_mul(49109) + 60004519) % 79;
        *seed
    });

    10 + rand
}
