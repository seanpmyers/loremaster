use std::ops::Add;

use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::utility::constants::DAYS_OF_WEEK;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WeekDayInformation {
    pub number: u8,
    pub week_day: time::Weekday,
}

#[derive(Prop)]
pub struct WeekProperties<'a> {
    pub selected_date: &'a Signal<time::OffsetDateTime>,
    pub days: &'a Signal<Vec<WeekDayInformation>>,
}

#[component]
pub fn Week<'a, 'b: 'a, G: Html>(
    context: Scope<'a>,
    WeekProperties {
        selected_date,
        days,
    }: WeekProperties<'b>,
) -> View<G> {
    days.set(create_week_list(&selected_date.get()));
    view! {context,
        div(class="week-widget", id="") {
            Keyed( KeyedProps {
                    iterable: days,
                    view: |context, day: WeekDayInformation|
                   {
                    let mut day_div_classes = String::from("card");
                    if &day.number == &selected_date.get().day() { day_div_classes.push_str(" active-card text-light") ;}
                    else { day_div_classes.push_str(" bg-white")}
                    view!{ context,
                        div(class=(day_div_classes)) {
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
                -(selected_weekday.number_days_from_sunday() as i64),
            ))
            .add(time::Duration::days(index as i64))
            .day();

        result.push(WeekDayInformation {
            number,
            week_day: *day,
        })
    }

    result
}
