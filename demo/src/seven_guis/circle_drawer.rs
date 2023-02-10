use indexmap::IndexMap;

use pwt::prelude::*;

use pwt::widget::{Button, Container, Dialog, Panel, Toolbar};
use pwt::widget::canvas::{Canvas, Circle};
use pwt::widget::form::Field;

pub enum Msg {
    Undo,
    Redo,
    CreateCircle(i32, i32),
    CircleClick(usize),
    ChangeRadius(usize),
    Select(usize),
    CloseDialog,
}

#[derive(Clone)]
struct CircleState {
    id: usize,
    x: i32,
    y: i32,
    r: usize,
}

pub struct CircleDrawer {
    undo: Vec<CircleState>,
    redo: Vec<CircleState>,
    selected: usize,
    show_resize_dialog: bool,
    unique_id_counter: usize,
}

impl CircleDrawer {

    fn create_resize_dialog(&self, ctx: &Context<Self>) -> Dialog {

        let circle = self.undo.iter().rev()
            .find(|circle| circle.id == self.selected)
            .unwrap();

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

        // use latest state, but keep drawing order
        let mut map = IndexMap::new();
        for circle in &self.undo {
            map.insert(circle.id, circle);
        }

        for (id, circle) in map {
            canvas.add_child(
                Circle::new()
                    .position(circle.x, circle.y)
                    .r(circle.r)
                    .stroke("black")
                    .stroke_width(2)
                    .fill(if self.selected == id { "grey" } else { "white" })
                    .onclick(Callback::from(move |e: MouseEvent| {
                        e.stop_propagation();
                    }))
                    .oncontextmenu(ctx.link().callback(move |e: MouseEvent| {
                        e.prevent_default();
                        Msg::CircleClick(id)
                    }))
                    .onmouseover(ctx.link().callback(move |_| {
                        Msg::Select(id)
                    }))
            );
        }

        canvas
    }

    fn create_toolbar(&self, ctx: &Context<Self>) -> Toolbar {
        Toolbar::new()
            .class("pwt-bg-color-surface")
            .with_flex_spacer()
            .with_child(
                Button::new("Undo")
                    .disabled(self.undo.is_empty())
                    .onclick(ctx.link().callback(|_| Msg::Undo))
            )
            .with_child(
                Button::new("Redo")
                    .disabled(self.redo.is_empty())
                    .onclick(ctx.link().callback(|_| Msg::Redo))
            )
            .with_flex_spacer()
    }
}

impl Component for CircleDrawer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            undo: Vec::new(),
            redo: Vec::new(),
            selected: 0,
            show_resize_dialog: false,
            unique_id_counter: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Undo => {
                if let Some(action) = self.undo.pop() {
                    self.redo.push(action);
                }
            }
            Msg::Redo => {
                if let Some(action) = self.redo.pop() {
                    self.undo.push(action);
                }
            }
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
                let id = self.unique_id_counter;
                self.unique_id_counter += 1;

                self.undo.push(CircleState { id, x, y, r: random_radius()})
            }
            Msg::ChangeRadius(new_radius) => {
                match self.undo.last_mut() {
                    Some(CircleState { id, r, .. }) if *id == self.selected => {
                        *r = new_radius;
                    }
                    _ => {
                        let mut circle = self.undo.iter().rev()
                            .find(|circle| circle.id == self.selected)
                            .unwrap()
                            .clone();
                        circle.r = new_radius;
                        self.undo.push(circle);
                    }
                }
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
