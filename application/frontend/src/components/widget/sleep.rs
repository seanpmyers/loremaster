use js_sys::Date;
use sycamore::prelude::*;
use time::Time;

use crate::{
    data::entity::sleep_schedule::SleepSchedule,
    utility::{constants::API_PERSON_SLEEP_SCHEDULE_ROUTE, http_service},
};

#[component(SleepWidget<G>)]
pub fn sleep_widget() -> View<G> {
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

    match (
        sleep_start_time.get().as_ref(),
        sleep_end_time.get().as_ref(),
    ) {
        (Some(start), Some(end)) => view! {
            div(class="sleep-widget") {
                div() {

                }
                div() {

                }
                div() {

                }
            }
        },
        _ => view! {
            div() {}
        },
    }
}
