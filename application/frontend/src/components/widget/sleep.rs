use js_sys::Date;
use sycamore::prelude::*;
use time::{macros::format_description, Time};

use crate::{
    components::{
        icon::{BATTERY_CHARGING_SVG_HTML, MOON_SVG_HTML, SUN_SVG_HTML},
        state::ComponentState,
    },
    data::entity::sleep_schedule::SleepSchedule,
    utility::{constants::API_PERSON_SLEEP_SCHEDULE_ROUTE, http_service},
};

const HOURS_IN_A_DAY: u8 = 24;

#[component(SleepWidget<G>)]
pub fn sleep_widget() -> View<G> {
    let state: Signal<ComponentState> = Signal::new(ComponentState::Loading);
    let loading: Signal<ComponentState> = state.clone();

    let sleep_start_time: Signal<Option<Time>> = Signal::new(None);
    let sleep_end_time: Signal<Option<Time>> = Signal::new(None);

    let hours_awake: Signal<String> = Signal::new(String::from(""));
    let hours_until_sleep: Signal<String> = Signal::new(String::from(""));
    let hours_of_sleep: Signal<String> = Signal::new(String::from(""));
    let wake_up: Signal<String> = Signal::new(String::from(""));
    let bedtime: Signal<String> = Signal::new(String::from(""));

    if G::IS_BROWSER {
        perseus::spawn_local(
            cloned!(loading, sleep_start_time, sleep_end_time, hours_awake, hours_until_sleep, hours_of_sleep, wake_up, bedtime  => async move {
            let query_response = http_service::get_endpoint(API_PERSON_SLEEP_SCHEDULE_ROUTE, None).await;
            let current_date = Date::new_0();
            match query_response {
                Some(response) => {
                    let potential_sleep_schedule: Option<SleepSchedule> =
                        serde_json::from_str(&response).unwrap();
                        match potential_sleep_schedule {
                            Some(schedule) => {
                                sleep_start_time.set(Some(schedule.start_time));
                                sleep_end_time.set(Some(schedule.end_time));
                                let format = format_description!("[hour]:[minute] [period]");
                                wake_up.set(schedule.end_time.format(format).unwrap());
                                bedtime.set(schedule.start_time.format(format).unwrap());
                                hours_until_sleep.set((schedule.start_time.hour() - current_date.get_hours() as u8).to_string());
                                hours_awake.set((current_date.get_hours() as u8 - schedule.end_time.hour()).to_string());
                                hours_of_sleep.set((HOURS_IN_A_DAY - (schedule.start_time.hour() - schedule.end_time.hour())).to_string());
                                loading.set(ComponentState::Visible);
                            },
                            None => loading.set(ComponentState::Error),
                        }
                }
                None => loading.set(ComponentState::Hidden),
            }
            }),
        );
    }

    view! {
       (match state.get().as_ref() {
           ComponentState::Visible => {
            let display_hours_awake: Signal<String> = hours_awake.clone();
            let display_hours_until_sleep: Signal<String> = hours_until_sleep.clone();
            let display_hours_of_sleep: Signal<String> = hours_of_sleep.clone();
            let display_wake_up: Signal<String> = wake_up.clone();
            let display_bedtime: Signal<String> = bedtime.clone();
            view! {
               div(class="sleep-widget") {
                   div(class="sleep-widget-section") {
                       div(class="sleep-widget-section-top") {
                           div(class="sleep-widget-number") { (display_hours_awake.get()) }
                           div(class="sleep-widget-icon", dangerously_set_inner_html=SUN_SVG_HTML) {}
                       }
                       div(class="sleep-widget-section-bottom") {
                           div() { "Hours awake" }
                           div() { (display_wake_up.get()) }
                       }
                   }
                   div(class="sleep-widget-section") {
                       div(class="sleep-widget-section-top") {
                           div(class="sleep-widget-number") { (display_hours_until_sleep.get()) }
                           div(class="sleep-widget-icon", dangerously_set_inner_html=MOON_SVG_HTML) {}
                       }
                       div(class="sleep-widget-section-bottom") {
                           div() { "Hours until sleep" }
                           div() { (display_bedtime.get()) }
                       }
                   }
                   div(class="sleep-widget-section") {
                       div(class="sleep-widget-section-top") {
                           div(class="sleep-widget-number") { (display_hours_of_sleep.get()) }
                           div(class="sleep-widget-icon", dangerously_set_inner_html=BATTERY_CHARGING_SVG_HTML) {}
                       }
                       div(class="sleep-widget-section-bottom") {
                           div() { "Hours of sleep" }
                       }
                   }
               }
           }},
           ComponentState::Hidden => view! {
               div(class="sleep-widget") { "No sleep schedule found." }
           },
           ComponentState::Error => view! {
               div(class="sleep-widget") { "No sleep schedule found." }
           },
           ComponentState::Loading => view! {
               div(class="sleep-widget") { "Loading..." }
           },
       })
    }
}
