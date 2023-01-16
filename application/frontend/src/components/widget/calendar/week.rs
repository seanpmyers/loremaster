use std::ops::Add;

use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::utility::constants::DAYS_OF_WEEK;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WeekDayInformation {
    pub number: u8,
    pub week_day: time::Weekday,
}

pub struct WeekProperties {
    pub selected_date: Signal<time::OffsetDateTime>,
    pub days: Signal<Vec<WeekDayInformation>>,
}

#[component(Week<G>)]
pub fn week(
    WeekProperties {
        selected_date,
        days,
    }: WeekProperties,
) -> View<G> {
    days.set(create_week_list(&selected_date.get()));
    view! {
        div(class="d-flex flex-row week_widget", id="") {
            Keyed( KeyedProps {
                    iterable: days.handle(),
                    template: move |day: WeekDayInformation|
                   {
                    let mut classes = String::from("d-flex flex-column m-2 p-2 border rounded shadow-sm");
                    if &day.number == &selected_date.get().day() { classes.push_str(" bg-primary text-white") ;}
                    else { classes.push_str(" bg-white")}
                    view!{
                        div(class=(classes)) {
                            div(class="m-1") {
                                (day.number)
                            }
                            div(class="m-1") {
                                (day.week_day.to_string())
                            }
                        }
                    }},
                    key: |day| day.number
                })
        }
    }
}

pub fn create_week_list(selected_date: &time::OffsetDateTime) -> Vec<WeekDayInformation> {
    let selected_weekday: time::Weekday = selected_date.weekday();
    let mut result: Vec<WeekDayInformation> = vec![];

    for (index, day) in DAYS_OF_WEEK.iter().enumerate() {
        let number: u8 = selected_date
            .add(time::Duration::days(
                -1 * selected_weekday.number_days_from_sunday() as i64,
            ))
            .add(time::Duration::days(index as i64))
            .day();

        result.push(WeekDayInformation {
            number: number,
            week_day: day.clone(),
        })
    }

    result
}
