use js_sys::Date;
use perseus::prelude::spawn_local_scoped;
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

#[component]
pub fn SleepWidget<G: Html>(context: Scope) -> View<G> {
    let state: &Signal<ComponentState> = create_signal(context, ComponentState::Loading);

    let sleep_start_time: &Signal<Option<Time>> = create_signal(context, None);
    let sleep_end_time: &Signal<Option<Time>> = create_signal(context, None);

    let hours_awake: &Signal<String> = create_signal(context, String::new());
    let hours_until_sleep: &Signal<String> = create_signal(context, String::new());
    let hours_of_sleep: &Signal<String> = create_signal(context, String::new());
    let wake_up: &Signal<String> = create_signal(context, String::new());
    let bedtime: &Signal<String> = create_signal(context, String::new());

    if G::IS_BROWSER {
        spawn_local_scoped(context, async move {
            let query_response =
                http_service::get_endpoint(API_PERSON_SLEEP_SCHEDULE_ROUTE, None).await;
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
                            hours_until_sleep.set(
                                (schedule.start_time.hour() - current_date.get_hours() as u8)
                                    .to_string(),
                            );
                            hours_awake.set(
                                (current_date.get_hours() as u8 - schedule.end_time.hour())
                                    .to_string(),
                            );
                            hours_of_sleep.set(
                                (HOURS_IN_A_DAY
                                    - (schedule.start_time.hour() - schedule.end_time.hour()))
                                .to_string(),
                            );
                            state.set(ComponentState::Visible);
                        }
                        None => state.set(ComponentState::Error),
                    }
                }
                None => state.set(ComponentState::Hidden),
            }
        });
    }

    view! { context,
       (match state.get().as_ref() {
           ComponentState::Visible => {
            view! { context,
               section(class="sleep-widget") {
                   article(class="sleep-widget-section") {
                       div(class="sleep-widget-section-top") {
                           div(class="sleep-widget-number") { (hours_awake.get()) }
                           div(class="sleep-widget-icon", dangerously_set_inner_html=SUN_SVG_HTML) {}
                       }
                       div(class="sleep-widget-section-bottom") {
                           div() { "Hours awake" }
                           div() { (wake_up.get()) }
                       }
                   }
                   article(class="sleep-widget-section") {
                       div(class="sleep-widget-section-top") {
                           div(class="sleep-widget-number") { (hours_until_sleep.get()) }
                           div(class="sleep-widget-icon", dangerously_set_inner_html=MOON_SVG_HTML) {}
                       }
                       div(class="sleep-widget-section-bottom") {
                           div() { "Hours until sleep" }
                           div() { (bedtime.get()) }
                       }
                   }
                   article(class="sleep-widget-section") {
                       div(class="sleep-widget-section-top") {
                           div(class="sleep-widget-number") { (hours_of_sleep.get()) }
                           div(class="sleep-widget-icon", dangerously_set_inner_html=BATTERY_CHARGING_SVG_HTML) {}
                       }
                       div(class="sleep-widget-section-bottom") {
                           div() { "Hours of sleep" }
                       }
                   }
               }
           }},
           ComponentState::Hidden => view! { context,
               div(class="sleep-widget") { "No sleep schedule found." }
           },
           ComponentState::Error => view! { context,
               div(class="sleep-widget") { "No sleep schedule found." }
           },
           ComponentState::Loading => view! { context,
               div(class="sleep-widget") { "Loading..." }
           },
       })
    }
}
