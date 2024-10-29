use sycamore::prelude::*;
use time::OffsetDateTime;

use crate::utility::constants::DAYS_OF_WEEK;

#[derive(Clone, PartialEq)]
pub struct CalendarDay {
    pub number: u32,
    pub day_of_week: time::Weekday,
}

#[component]
pub fn MonthWidget<G: Html>(context: Scope) -> View<G> {
    let date: &Signal<OffsetDateTime> = create_signal(context, OffsetDateTime::now_utc());
    if G::IS_BROWSER {
        date.set(time::OffsetDateTime::now_local().unwrap());
    }

    let days_of_week = create_signal(context, DAYS_OF_WEEK.to_vec());

    let calendar_days: &Signal<Vec<CalendarDay>> = create_signal(context, Vec::new());
    create_effect(context, move || {
        let total = time::util::days_in_year_month(date.get().year(), date.get().month()) as usize;
        for i in 0..total {}
    });
    view! { context,
        div(class="card month-widget") {
            div(class="today") {
                span() { (date.get().weekday().to_string()) }
                span() { (date.get().day().to_string()) }
            }
            div(class="calendar") {
                div(class="") { (date.get().month().to_string()) }
                div(class="weekdays-header") {
                   Indexed(
                    iterable=days_of_week,
                    view=move |context, weekday| {
                        view!{context,
                            div() { (weekday.to_string()[0..1].to_string()) (weekday.number_days_from_sunday()) }
                            Indexed(
                                iterable=calendar_days,
                                view=move |context, day| {
                                    view!{context,
                                        div() { (day.number) }
                                    }
                                }
                            )
                        }
                    }
                   )
                }
            }
        }
    }
}
