use js_sys::{Date, JsString};
use sycamore::prelude::*;

use crate::utility::date_time_helper::{get_day_of_week_from_integer, get_month_from_integer};

#[component(Demo<G>)]
pub fn demo() -> View<G> {
    let main_content_class: &str =
        "col-10 bg-white border-1 rounded-top rounded-end border-top border-end overflow-auto shadow p-5";
    let main_content_style: &str = "height: calc(100vh - 56px);";
    let javascript_date: Date = Date::new_0();

    let day_of_week: String = get_day_of_week_from_integer(javascript_date.get_day());
    let date: u32 = javascript_date.get_date();
    let year: u32 = javascript_date.get_full_year();
    let month: String = get_month_from_integer(javascript_date.get_month());

    let time: JsString = Date::to_time_string(&javascript_date);
    let date_time_value: String = format!("{day_of_week} , {month} {date}, {year}");
    view! {
        div(class="container-fluid") {
            div(class="row") {
                div(class=main_content_class, style=main_content_style) {
                    h1() { (date_time_value)  }
                    h2() { (time) }
                    div() {}
                    h2() { "Good Evening, Sean"}
                }
                div(class="col-2") {
                    div(class="card shadow") {
                        div(class="card-body") {
                            h5(class="card-title") { "Objectives"}
                            div(class="card-text") {
                                ul() {
                                    li() {"Exercise"}
                                    li() {"Study"}
                                    li() {"Code"}
                                    li() {"Eat"}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
