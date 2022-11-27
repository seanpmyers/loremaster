use js_sys::{Date, JsString};

use perseus::{RenderFnResultWithCause, Template};
use sycamore::{
    prelude::{cloned, view, Html, SsrNode, View},
    reactive::Signal,
};
use uuid::Uuid;

use crate::{
    components::{
        container::{Container, ContainerProperties},
        widget::week_widget::{WeekWidget, WeekWidgetProperties},
    },
    data::entity::person_chronicle::PersonChronicle,
    utility::{
        constants::API_CHRONICLE_TODAY_URL,
        date_time_helper::{get_day_of_week_from_integer, get_month_from_integer},
        http_service,
    },
};

#[perseus::make_rx(ChroniclePageStateRx)]
pub struct ChroniclePageState {
    pub user_alias: String,
    pub chronicle_id: Uuid,
    pub date_display: String,
    pub short_date_display: String,
    pub time_display: String,
    pub greeting: String,
}

#[perseus::template_rx]
pub fn chronicle_page(
    ChroniclePageStateRx {
        user_alias,
        chronicle_id,
        date_display,
        short_date_display,
        time_display,
        greeting,
    }: ChroniclePageStateRx,
) -> View<G> {
    if G::IS_BROWSER {
        perseus::spawn_local(
            cloned!((date_display, short_date_display, time_display, chronicle_id, greeting) => async move {
                let javascript_date: Date = Date::new_0();

                let day_of_week: String = get_day_of_week_from_integer(javascript_date.get_day());
                let date: u32 = javascript_date.get_date();
                let year: u32 = javascript_date.get_full_year();
                let month: String = get_month_from_integer(javascript_date.get_month());

                let time: JsString = Date::to_locale_time_string(&javascript_date, "en-US");
                time_display.set(time.as_string().unwrap());
                date_display.set(format!("{day_of_week}, {month} {date}, {year}"));
                short_date_display.set(format!("{}/{}/{}", javascript_date.get_full_year(), javascript_date.get_month(), javascript_date.get_date()));

                let query_response = http_service::get_endpoint(API_CHRONICLE_TODAY_URL ,None).await;
                match query_response {
                    Some(response) => {
                        let chronicle_data: PersonChronicle = serde_json::from_str(&response).unwrap();
                        chronicle_id.set(chronicle_data.chronicle_id);
                        if let Some(alias) = chronicle_data.person_alias {
                            user_alias.set(alias);
                        }
                    },
                    None => todo!(),
                }
                match javascript_date.get_hours() {
                    hour if hour < 11_u32 && hour >= 5_u32 => greeting.set(format!("Good Morning, {}", user_alias.get())),
                    hour if hour >= 12_u32 && hour < 17_u32 => greeting.set(format!("Good Afternoon, {}", user_alias.get())),
                    hour if hour >= 17_u32 || hour < 5_u32 => greeting.set(format!("Good Evening, {}", user_alias.get())),
                    _ => greeting.set(format!("Hello, {}", user_alias.get()))
                }

            }),
        );
    }

    let now_utc_date = time::OffsetDateTime::now_utc();
    let local_date = now_utc_date.to_offset(time::macros::offset!(-5));

    view! {
            Container(ContainerProperties {
                title: String::from("Chronicle"),
                children: view! {
                    div(class="container-fluid d-flex flex-grow-1") {
                        div(class="row flex-grow-1 text-black"){
                            div(class="col-9 bg-white p-5 border-0 rounded") {
                                div(class="d-flex align-items-baseline") {
                                    h2(class="display-6 flex-grow-1") { (date_display.get()) }
                                    div(class="fw-normal flex-shrink-1 badge fs-5 bg-success") {
                                        (format!("{} {}", short_date_display.get(), time_display.get()))
                                    }
                                }
                                h3(class="display-6") { (greeting.get()) }
                                div() {
                                    WeekWidget(WeekWidgetProperties{
                                        days: Signal::new(vec![]),
                                        selected_date: Signal::new(local_date),
                                    })
                                }
                                div() {
                                    label() { "What do you intend to do today?" }
                                }
                                div(class="d-flex flex-column") {
                                    label() { "Notes" }
                                    textarea(rows="4", cols="50") {}
                                }
                            }
                            div(class="col-3") {
                                div(class="card shadow border-0 rounded") {
                                    div(class="card-body") {
                                        h3(class="card-title") { "Goals" }
                                        p(class="card-text") {
                                            ul() {
                                                li() { "-" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
            },
        })
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<ChroniclePageState> {
    Ok(ChroniclePageState {
        user_alias: String::from("Stranger"),
        chronicle_id: Uuid::nil(),
        date_display: String::new(),
        short_date_display: String::new(),
        time_display: String::new(),
        greeting: String::new(),
    })
}

#[perseus::head]
pub fn head(_props: ChroniclePageState) -> View<SsrNode> {
    view! {
        title { "Chronicle | Loremaster" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("chronicle")
        .build_state_fn(get_build_state)
        .template(chronicle_page)
        .head(head)
}
