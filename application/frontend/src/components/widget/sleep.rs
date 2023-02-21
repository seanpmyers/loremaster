use js_sys::Date;
use sycamore::prelude::*;
use time::Time;

use crate::{
    components::icon::{BATTERY_CHARGING_SVG_HTML, MOON_SVG_HTML, SUN_SVG_HTML},
    data::entity::sleep_schedule::SleepSchedule,
    utility::{constants::API_PERSON_SLEEP_SCHEDULE_ROUTE, http_service},
};

#[component(SleepWidget<G>)]
pub fn sleep_widget() -> View<G> {
    let loading: Signal<bool> = Signal::new(false);
    let schedule_exists: Signal<bool> = Signal::new(true);

    let sleep_start_time: Signal<Option<Time>> = Signal::new(None);
    let sleep_end_time: Signal<Option<Time>> = Signal::new(None);

    if G::IS_BROWSER {
        perseus::spawn_local(cloned!(sleep_start_time, sleep_end_time => async move{
        let javascript_date: Date = Date::new_0();
        let query_response = http_service::get_endpoint(API_PERSON_SLEEP_SCHEDULE_ROUTE, None).await;
        match query_response {
            Some(response) => {
                let potential_sleep_schedule: Option<SleepSchedule> =
                    serde_json::from_str(&response).unwrap();
                    match potential_sleep_schedule {
                        Some(schedule) => {
                            sleep_start_time.set(Some(schedule.start_time));
                            sleep_end_time.set(Some(schedule.end_time));
                        },
                        None => (),
                    }
            }
            None => (),
        }
        }));
    }

    match schedule_exists.get().as_ref() {
        true => view! {
            div(class="sleep-widget") {
                div(class="sleep-widget-section") {
                    div(class="sleep-widget-section-top") {
                        div(class="sleep-widget-number") { "12" }
                        div(class="sleep-widget-icon", dangerously_set_inner_html=SUN_SVG_HTML) {}
                    }
                    div(class="sleep-widget-section-bottom") {
                        div() { "Hours awake" }
                        div() { "5:30 AM" }
                    }
                }
                div(class="sleep-widget-section") {
                    div(class="sleep-widget-section-top") {
                        div(class="sleep-widget-number") { "4" }
                        div(class="sleep-widget-icon", dangerously_set_inner_html=MOON_SVG_HTML) {}
                    }
                    div(class="sleep-widget-section-bottom") {
                        div() { "Hours until sleep" }
                        div() { "9:30 PM" }
                    }
                }
                div(class="sleep-widget-section") {
                    div(class="sleep-widget-section-top") {
                        div(class="sleep-widget-number") { "4" }
                        div(class="sleep-widget-icon", dangerously_set_inner_html=BATTERY_CHARGING_SVG_HTML) {}
                    }
                    div(class="sleep-widget-section-bottom") {
                        div() { "Hours of sleep" }
                    }
                }
            }
        },
        false => view! {
            div() {}
        },
    }
}
