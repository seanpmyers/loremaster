use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WeekDayInformation {
    pub number: u8,
    pub week_day: time::Weekday,
}

pub struct WeekWidgetProperties {
    pub selected_date: time::OffsetDateTime,
    pub days: Vec<WeekDayInformation>,
}

#[component(WeekWidget<G>)]
pub fn week_widget(
    WeekWidgetProperties {
        selected_date,
        days,
    }: WeekWidgetProperties,
) -> View<G> {
    view! {
        div(class="d-flex flex-column", id="") {
            // Keyed( KeyedProps {
            //         iterable: days.handle(),
            //         template: move |day| view!{

            //         },
            //         key: |day| day.number
            //     })
        }
    }
}

pub fn createWeekList(selected_date: &time::OffsetDateTime) -> Vec<WeekDayInformation> {
    let mut result: Vec<WeekDayInformation> = vec![];
    let days_of_week = vec![
        time::Weekday::Sunday,
        time::Weekday::Monday,
        time::Weekday::Tuesday,
        time::Weekday::Wednesday,
        time::Weekday::Thursday,
        time::Weekday::Friday,
        time::Weekday::Saturday,
    ];

    result
}
