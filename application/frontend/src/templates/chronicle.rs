use futures_util::{future::ready, stream::StreamExt};
use gloo_timers::future::IntervalStream;
use js_sys::{Date, JsString};

use perseus::{RenderFnResultWithCause, Template};
use sycamore::{
    prelude::{cloned, view, Html, SsrNode, View},
    reactive::Signal,
};
use time::Time;
use uuid::Uuid;

use crate::{
    components::{
        container::{Container, ContainerProperties},
        widget::{
            calendar::week::Week, calendar::week::WeekProperties, goal_list::GoalList,
            goal_list::GoalListProperties,
        },
    },
    data::entity::{person_chronicle::PersonChronicle, sleep_schedule::SleepSchedule},
    utility::{
        constants::{API_CHRONICLE_TODAY_URL, API_PERSON_SLEEP_SCHEDULE_ROUTE},
        date_time_helper::{get_day_of_week_from_integer, get_month_from_integer},
        http_service,
    },
};

#[perseus::make_rx(ChroniclePageStateRx)]
pub struct ChroniclePageState {
    pub user_alias: String,
    pub chronicle_id: Uuid,
    pub date_display: String,
    pub short_date_display: String,
    pub time_display: String,
    pub greeting: String,
    pub sleep_start_time: Option<Time>,
    pub sleep_end_time: Option<Time>,
    pub hours_remaining: u32,
}

#[perseus::template_rx]
pub fn chronicle_page(
    ChroniclePageStateRx {
        user_alias,
        chronicle_id,
        date_display,
        short_date_display,
        time_display,
        greeting,
        sleep_start_time,
        sleep_end_time,
        hours_remaining,
    }: ChroniclePageStateRx,
) -> View<G> {
    if G::IS_BROWSER {
        perseus::spawn_local(
            cloned!((date_display, short_date_display, time_display, chronicle_id, greeting, sleep_start_time, sleep_end_time) => async move {
                let javascript_date: Date = Date::new_0();

                let day_of_week: String = get_day_of_week_from_integer(javascript_date.get_day());
                let date: u32 = javascript_date.get_date();
                let year: u32 = javascript_date.get_full_year();
                let month: String = get_month_from_integer(javascript_date.get_month());

                let time: JsString = Date::to_locale_time_string(&javascript_date, "en-US");
                time_display.set(time.as_string().unwrap());
                date_display.set(format!("{day_of_week}, {month} {date}, {year}"));
                short_date_display.set(format!("{}/{}/{}", javascript_date.get_full_year(), javascript_date.get_month(), javascript_date.get_date()));

                let mut query_response = http_service::get_endpoint(API_CHRONICLE_TODAY_URL, None).await;
                match query_response {
                    Some(response) => {
                        let chronicle_data: PersonChronicle = serde_json::from_str(&response).unwrap();
                        chronicle_id.set(chronicle_data.chronicle_id);
                        if let Some(alias) = chronicle_data.person_alias {
                            user_alias.set(alias);
                        }
                    },
                    None => (),
                }

                query_response = http_service::get_endpoint(API_PERSON_SLEEP_SCHEDULE_ROUTE, None).await;
                match query_response {
                    Some(response) => {
                        let potential_sleep_schedule: Option<SleepSchedule> = serde_json::from_str(&response).unwrap();
                        match potential_sleep_schedule {
                            Some(schedule) => {
                                sleep_start_time.set(Some(schedule.start_time));
                                sleep_end_time.set(Some(schedule.end_time));
                            },
                            None => (),
                        }
                    },
                    None => (),
                }

                match javascript_date.get_hours() {
                    hour if hour < 11_u32 && hour >= 5_u32 => greeting.set(format!("Good Morning, {}", user_alias.get())),
                    hour if hour >= 12_u32 && hour < 17_u32 => greeting.set(format!("Good Afternoon, {}", user_alias.get())),
                    hour if hour >= 17_u32 || hour < 5_u32 => greeting.set(format!("Good Evening, {}", user_alias.get())),
                    _ => greeting.set(format!("Hello, {}", user_alias.get()))
                }

                IntervalStream::new(1_000).for_each(|_| {
                    let javascript_date: Date = Date::new_0();

                    let time: JsString = Date::to_locale_time_string(&javascript_date, "en-US");
                    time_display.set(time.as_string().unwrap());
                    short_date_display.set(format!("{}/{}/{}", javascript_date.get_full_year(), javascript_date.get_month(), javascript_date.get_date()));
                    ready(())
                }).await;


            }),
        );
    }

    let now_utc_date = time::OffsetDateTime::now_utc();
    let local_date = now_utc_date.to_offset(time::macros::offset!(-5));
    let current_hour: u8 = local_date.hour();
    let display_sleep_start = sleep_start_time.clone();
    let display_sleep_end = sleep_end_time.clone();

    view! {
            Container(ContainerProperties {
                title: String::from("Chronicle"),
                children: view! {
                    div(class="container-fluid d-flex flex-grow-1 bg-light") {
                        div(class="row flex-grow-1 text-black"){
                            div(class="col-9 bg-light p-5 border-0 rounded") {
                                div(class="d-flex align-items-baseline") {
                                    h2(class="display-6 flex-grow-1") { (date_display.get()) }
                                    div(class="fw-normal flex-shrink-1 badge fs-5 bg-primary") {
                                        (format!("{} {}", short_date_display.get(), time_display.get()))
                                    }
                                }
                                h3(class="display-6") { (greeting.get()) }
                                div() {
                                    Week(WeekProperties{
                                        days: Signal::new(vec![]),
                                        selected_date: Signal::new(local_date),
                                    })
                                }
                                div() {
                                    label() { "What do you intend to do today?" }
                                }
                                div() {
                                    label() { "Hours until sleep" }
                                    div() { (
                                        match display_sleep_start.get().as_ref() {
                                            Some(time) => (time.hour() as i8 - current_hour as i8).to_string(),
                                            None => String::from("")
                                        }
                                    ) }
                                }
                                div() {
                                    label() { "Hours awake" }
                                    div() { (
                                        match display_sleep_end.get().as_ref() {
                                            Some(time) => (current_hour as i8 - time.hour() as i8).to_string(),
                                            None => String::from("")
                                        }
                                    ) }
                                 }
                                div(class="d-flex flex-column") {
                                    label() { "Notes" }
                                    textarea(class="border rounded bg-white", rows="4", cols="50") {}
                                }
                            }
                            div(class="col-3 border-start") {
                                div(class="card shadow-sm border-0 rounded mt-2") {
                                    div(class="card-body") {
                                        h3(class="card-title") { "Goals" }
                                        p(class="card-text") {
                                            GoalList(GoalListProperties{goals: Signal::new(Vec::new())})
                                        }
                                    }
                                }
                            }
                        }
                    }
            },
        })
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<ChroniclePageState> {
    Ok(ChroniclePageState {
        user_alias: String::from("Stranger"),
        chronicle_id: Uuid::nil(),
        date_display: String::new(),
        short_date_display: String::new(),
        time_display: String::new(),
        greeting: String::new(),
        sleep_start_time: None,
        sleep_end_time: None,
        hours_remaining: 24_u32,
    })
}

#[perseus::head]
pub fn head(_props: ChroniclePageState) -> View<SsrNode> {
    view! {
        title { "Chronicle | Loremaster" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("chronicle")
        .build_state_fn(get_build_state)
        .template(chronicle_page)
        .head(head)
}
