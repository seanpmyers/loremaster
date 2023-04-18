use js_sys::Date;
use perseus::{engine_only_fn, template::Template};
use sycamore::{
    prelude::{view, Html, Indexed, SsrNode, View},
    reactive::{create_signal, BoundedScope, Scope, Signal},
};

use crate::{
    components::container::Container,
    utility::date_time_helper::{get_day_of_week_from_integer, get_month_from_integer},
};

pub const WEEKS_IN_YEAR: u16 = 52_u16;

const PAGE_ROUTE_PATH: &str = "timeline";
const PAGE_TITLE: &str = "Timeline | Loremaster";

pub fn timeline_page<'page, G: Html>(context: BoundedScope<'_, 'page>) -> View<G> {
    let date: &Signal<String> = create_signal(context, String::from(""));
    let circles: &Signal<Vec<u16>> = create_signal(context, (1_u16..WEEKS_IN_YEAR).collect());
    let year_start: &Signal<u32> = create_signal(context, 1996_u32);
    let desired_life_length_years: u32 = 90_u32;
    let life_years: &Signal<Vec<u32>> = create_signal(
        context,
        (*year_start.get()..(desired_life_length_years + *year_start.get())).collect(),
    );
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
            div(class="d-flex flex-column flex-grow-1 p-4") {
                div(class="d-flex") {
                    h3(class="fw-normal") { (date.get()) }

                }
                div(class="d-flex flex-column") {
                    div(class="d-flex align-items-center") {
                        div(class="me-1 ms-1") { "Year" }
                        div(class="me-1 ms-1") { "Age" }
                        div(class="me-1 ms-1") { "Weeks" }
                    }
                    Indexed(
                        iterable= life_years,
                        view= move |context, year| view! {context,
                            div(class="d-flex align-items-center") {
                                div(class="me-1 ms-1") { (year.to_string()) }
                                div(class="me-1 ms-1") { ((year - *year_start.get()).to_string()) }
                                div(class="d-flex me-1 ms-1") {
                                    Indexed(
                                        iterable=circles,
                                        view=move |context, _circle| view !{ context,
                                            span(class="timeline-circle m-1") {}
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
