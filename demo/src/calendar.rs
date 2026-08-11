use std::collections::VecDeque;

use pwt::css::FlexFit;
use pwt::prelude::*;
use pwt::widget::form::PlainDate;
use pwt::widget::{
    Button, CalendarGrid, CalendarGridDay, CalendarGridView, CalendarSpanBar, CalendarSpanClick,
    Container, Panel, Toolbar,
};

static MONTH_NAMES: &[&str] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

fn add_months(date: PlainDate, delta: i32) -> PlainDate {
    let total = date.year() * 12 + date.month() as i32 + delta;
    PlainDate::new(total.div_euclid(12), total.rem_euclid(12) as u32, 1)
}

fn demo_span_bars(today: PlainDate) -> Vec<CalendarSpanBar> {
    let iso = |days: i32| today.add_days(days).format("Y-m-d");
    vec![
        CalendarSpanBar::new(iso(-20), iso(30), "Release freeze")
            .id("freeze")
            .color("#c62828")
            .icon("fa fa-lock")
            .pinned(true),
        CalendarSpanBar::new(iso(3), iso(9), "Alice - vacation")
            .id("leave-alice")
            .color("#2e7d32")
            .icon("fa fa-plane"),
        CalendarSpanBar::new(iso(-1), iso(2), "Team offsite")
            .id("offsite")
            .color("#6a1b9a")
            .icon("fa fa-users"),
        CalendarSpanBar::new(iso(4), iso(5), "Bob - training")
            .id("training-bob")
            .color("#ef6c00"),
        CalendarSpanBar::new(iso(0), iso(0), "All-hands")
            .id("all-hands")
            .color("#1565c0")
            .icon("fa fa-bullhorn"),
    ]
}

pub enum Msg {
    Page(i32),
    Today,
    View(CalendarGridView),
    SpanClick(CalendarSpanClick),
    DayClick(CalendarGridDay),
}

pub struct CalendarDemo {
    today: PlainDate,
    anchor: PlainDate,
    view: CalendarGridView,
    selected: Option<String>,
    logs: VecDeque<String>,
}

impl CalendarDemo {
    fn log(&mut self, msg: String) {
        if self.logs.len() > 4 {
            self.logs.pop_front();
        }
        self.logs.push_back(msg);
    }

    fn title(&self) -> String {
        match self.view {
            CalendarGridView::Month => format!(
                "{} {}",
                MONTH_NAMES[self.anchor.month() as usize],
                self.anchor.year(),
            ),
            CalendarGridView::Week => {
                format!("Week {}, {}", self.anchor.iso_week(), self.anchor.year())
            }
        }
    }
}

impl Component for CalendarDemo {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let today = PlainDate::today();
        Self {
            today,
            anchor: today,
            view: CalendarGridView::Month,
            selected: None,
            logs: VecDeque::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Page(direction) => {
                self.anchor = match self.view {
                    CalendarGridView::Month => add_months(self.anchor, direction),
                    CalendarGridView::Week => self.anchor.add_days(direction * 7),
                };
            }
            Msg::Today => self.anchor = self.today,
            Msg::View(view) => self.view = view,
            Msg::SpanClick(click) => {
                self.selected = click.id.clone();
                self.log(format!(
                    "SPAN CLICK {} on {}",
                    click.id.as_deref().unwrap_or("-"),
                    click.date,
                ));
            }
            Msg::DayClick(day) => self.log(format!("DAY CLICK {}", day.date)),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let toolbar = Toolbar::new()
            .with_child(
                Button::new_icon("fa fa-chevron-left")
                    .aria_label("Previous")
                    .onclick(link.callback(|_| Msg::Page(-1))),
            )
            .with_child(Button::new("Today").onclick(link.callback(|_| Msg::Today)))
            .with_child(
                Button::new_icon("fa fa-chevron-right")
                    .aria_label("Next")
                    .onclick(link.callback(|_| Msg::Page(1))),
            )
            .with_child(Container::new().padding_x(2).with_child(self.title()))
            .with_flex_spacer()
            .with_child(
                Button::new("Week")
                    .pressed(self.view == CalendarGridView::Week)
                    .onclick(link.callback(|_| Msg::View(CalendarGridView::Week))),
            )
            .with_child(
                Button::new("Month")
                    .pressed(self.view == CalendarGridView::Month)
                    .onclick(link.callback(|_| Msg::View(CalendarGridView::Month))),
            );

        let grid = CalendarGrid::new(self.anchor.format("Y-m-d"))
            .view(self.view)
            .today(self.today.format("Y-m-d"))
            .span_bars(demo_span_bars(self.today))
            .selected_span(self.selected.clone().map(AttrValue::from))
            .render_span_tooltip(|bar: &CalendarSpanBar| {
                Container::new()
                    .padding(2)
                    .with_child(html! { <b>{&bar.label}</b> })
                    .with_child(html! { <br/> })
                    .with_child(format!("{} to {}", bar.start, bar.end))
                    .into()
            })
            .on_span_click(link.callback(Msg::SpanClick))
            .on_day_click(link.callback(Msg::DayClick));

        let log_panel = Panel::new().title("Calendar click log").with_child(
            Container::from_tag("pre").padding(2).with_child({
                self.logs
                    .iter()
                    .map(|t| html! {format!("{t}\n")})
                    .collect::<Html>()
            }),
        );

        Panel::new()
            .class("pwt-content-spacer")
            .class(FlexFit)
            .border(false)
            .with_child(toolbar)
            .with_child(grid)
            .with_child(log_panel)
            .into()
    }
}
