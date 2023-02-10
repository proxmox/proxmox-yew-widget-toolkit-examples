use indexmap::IndexMap;

use pwt::prelude::*;

use pwt::widget::{Button, Container, Dialog, Panel, Toolbar};
use pwt::widget::canvas::{Canvas, Circle};
use pwt::widget::form::Field;

// TODO: implement Undo/Redo functionality

pub enum Msg {
    Undo,
    Redo,
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

enum Action {
    Create(usize, i32, i32, usize), /* (id, x , y, radius) */
    Radius(usize, usize), /* (id, new-radius */
}

pub struct CircleDrawer {
    undo: Vec<Action>,
    redo: Vec<Action>,
    selected: usize,
    show_resize_dialog: bool,
    unique_id_counter: usize,
}

impl CircleDrawer {

    fn create_resize_dialog(&self, ctx: &Context<Self>) -> Dialog {

        let mut circle: Option<CircleState> = None;
        for action in self.undo.iter() {
            match action {
                Action::Create(id, x, y, r) if *id == self.selected => {
                    circle = Some(CircleState { x: *x, y: *y, r: *r});
                }
                Action::Radius(id, r) if *id == self.selected => {
                    let old = circle.unwrap();
                    circle = Some(CircleState { x: old.x, y: old.y, r: *r});
                }
                _ => {}
            }

        }

        let circle = match circle {
            None => panic!("unable toö find circle {} - internal error", self.selected),
            Some(circle) => circle,
        };

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

        let mut map = IndexMap::new();
        for action in &self.undo {
            match action {
                Action::Create(id, x, y, r) => {
                    map.insert(id, CircleState { x: *x, y: *y, r: *r });
                }
                Action::Radius(id, r) => {
                    map.get_mut(id).unwrap().r = *r;
                }
            }
        }

        for (id, circle) in map {
            let id = *id;
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

                self.undo.push(Action::Create(id, x, y, random_radius()))
            }
            Msg::ChangeRadius(new_radius) => {
                match self.undo.last_mut() {
                    Some(Action::Radius(id, r)) if *id == self.selected => {
                        *r = new_radius;
                    }
                    _ => {
                        self.undo.push(Action::Radius(self.selected, new_radius));
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
