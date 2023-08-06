use js_sys::Date;
use perseus::{engine_only_fn, template::Template};
use sycamore::{
    prelude::{view, Html, Indexed, SsrNode, View},
    reactive::{create_selector, create_signal, BoundedScope, ReadSignal, Scope, Signal},
};

use crate::{
    components::container::Container,
    utility::date_time_helper::{get_day_of_week_from_integer, get_month_from_integer},
};

pub const WEEKS_IN_YEAR: u16 = 52_u16;
pub const ADULT_MINIMUM_AGE: i32 = 18_i32;
pub const ADULT_AVERAGE_LIFE: i32 = 80_i32;
pub const MAX_UI_ROWS: i32 = 200;

const PAGE_ROUTE_PATH: &str = "timeline";
const PAGE_TITLE: &str = "Timeline | Loremaster";

pub fn timeline_page<'page, G: Html>(context: BoundedScope<'_, 'page>) -> View<G> {
    let date: &Signal<String> = create_signal(context, String::from(""));
    let circles: &Signal<Vec<u16>> = create_signal(context, (1_u16..WEEKS_IN_YEAR).collect());
    let year_start: &Signal<String> = create_signal(
        context,
        if G::IS_BROWSER {
            (time::OffsetDateTime::now_local().unwrap().year() - ADULT_MINIMUM_AGE).to_string()
        } else {
            String::from("0")
        },
    );

    let year_end: &Signal<String> = create_signal(
        context,
        if G::IS_BROWSER {
            (time::OffsetDateTime::now_local().unwrap().year()
                + (ADULT_AVERAGE_LIFE - ADULT_MINIMUM_AGE))
                .to_string()
        } else {
            String::from("0")
        },
    );

    let year_start_number: &ReadSignal<i32> = create_selector(context, move || {
        if year_start.get().is_empty() {
            return 0_i32;
        }
        year_start.get().parse::<i32>().unwrap()
    });

    let year_end_number: &ReadSignal<i32> = create_selector(context, move || {
        if year_end.get().is_empty() {
            return 0_i32;
        }
        year_end.get().parse::<i32>().unwrap()
    });

    let current_year: &Signal<i32> = create_signal(
        context,
        if G::IS_BROWSER {
            time::OffsetDateTime::now_local().unwrap().year()
        } else {
            0
        },
    );

    let life_years: &ReadSignal<Vec<_>> = create_selector(context, move || {
        let start: i32 = *year_start_number.get();
        let end: i32 = *year_end_number.get();
        if end - start > MAX_UI_ROWS {
            return (start..start + MAX_UI_ROWS).collect();
        }
        (start..end).collect()
    });

    if G::IS_BROWSER {
        let javascript_date: Date = Date::new_0();
        let day_of_week: String = get_day_of_week_from_integer(javascript_date.get_day());
        let month: String = get_month_from_integer(javascript_date.get_month());
        date.set(format!(
            "{}, {} {}, {}",
            day_of_week,
            month,
            javascript_date.get_date(),
            javascript_date.get_full_year()
        ));
    }

    view! { context,
        Container(title="Timeline") {
            div(class="timeline-widget") {
                div(class="timeline-header") {
                    h3(class="") { (date.get()) }
                }
                div(class="timeline-content") {
                    div(class="timeline-inputs") {
                        div(class="") {
                            label() { "Birth Year" }
                            input(type="number", bind:value=year_start) {}
                        }
                        div(class="") {
                            label() { "Death Year" }
                            input(type="number", bind:value=year_end) {}
                        }
                        div(class="") {
                            label() { "Age" }
                            input(disabled=true, value=(*current_year.get() - *year_start_number.get())) {}
                        }
                        div(class="") {
                            label() { "Weeks" }
                            input(disabled=true, value=((*current_year.get() - *year_start_number.get()) * WEEKS_IN_YEAR as i32)) {}
                        }
                    }
                    Indexed(
                        iterable= life_years,
                        view= move |context, year| view! {context,
                            div(class="timeline-row") {
                                div(class="") { (year.to_string()) }
                                div(class="") { ((year - *year_start_number.get()).to_string()) }
                                div(class="") {
                                    Indexed(
                                        iterable=circles,
                                        view=move |context, _circle| view !{ context,
                                            span(class="timeline-circle") {}
                                        }
                                    )
                                }
                            }
                        }
                    )
                }
            }
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build(PAGE_ROUTE_PATH)
        .view(timeline_page)
        .head(head)
        .build()
}

#[engine_only_fn]
fn head(context: Scope) -> View<SsrNode> {
    view! { context,
        title { (PAGE_TITLE) }
    }
}
