use js_sys::Date;
use perseus::Template;
use sycamore::{
    prelude::{view, Html, Indexed, IndexedProps, SsrNode, View},
    reactive::{cloned, Signal},
};

use crate::{
    components::container::{Container, ContainerProperties},
    utility::date_time_helper::{get_day_of_week_from_integer, get_month_from_integer},
};

pub const WEEKS_IN_YEAR: u16 = 52_u16;

#[perseus::template_rx]
pub fn timeline_page() -> View<G> {
    let date: Signal<String> = Signal::new(String::from(""));
    let display_date: Signal<String> = date.clone();
    let circles: Signal<Vec<u16>> = Signal::new((1_u16..WEEKS_IN_YEAR).collect());
    let year_start: u32 = 1996_u32;
    let desired_life_length_years: u32 = 90_u32;
    let life_years: Signal<Vec<u32>> =
        Signal::new((year_start..(desired_life_length_years + year_start)).collect());
    if G::IS_BROWSER {
        cloned!((date) => {
            let javascript_date: Date = Date::new_0();
            let day_of_week: String = get_day_of_week_from_integer(javascript_date.get_day());
            let month: String = get_month_from_integer(javascript_date.get_month());
            date.set(format!("{}, {} {}, {}", day_of_week, month, javascript_date.get_date(), javascript_date.get_full_year()));
        });
    }
    view! {
        Container(ContainerProperties{title: String::from("Timeline"), children: view!{
            div(class="d-flex flex-column flex-grow-1 p-4") {
                div(class="d-flex") {
                    h3(class="fw-normal") { (display_date.get()) }

                }
                div(class="d-flex flex-column") {
                    div(class="d-flex align-items-center") {
                        div(class="me-1 ms-1") { "Year" }
                        div(class="me-1 ms-1") { "Age" }
                        div(class="me-1 ms-1") { "Weeks" }
                    }
                    Indexed(IndexedProps{
                        iterable: life_years.handle(),
                        template: move |year| view! {
                            div(class="d-flex align-items-center") {
                                div(class="me-1 ms-1") { (year.to_string()) }
                                div(class="me-1 ms-1") { ((year - year_start).to_string()) }
                                div(class="d-flex me-1 ms-1") {
                                    Indexed(IndexedProps{
                                        iterable: circles.handle(),
                                        template: |_circle| view !{
                                            span(class="timeline-circle m-1") {}
                                        }
                                    })
                                }
                            }
                        }
                    })
                }
            }
        }})
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "Timeline | Loremaster" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("timeline").template(timeline_page).head(head)
}
