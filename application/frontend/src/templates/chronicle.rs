use js_sys::{Date, JsString};
use log::info;
use perseus::{RenderFnResultWithCause, Template};
use sycamore::prelude::{cloned, view, Html, Signal, SsrNode, View};
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

use crate::{
    components::container::{Container, ContainerProperties},
    utility::{
        constants::API_CHRONICLE_TODAY_URL,
        date_time_helper::{get_day_of_week_from_integer, get_month_from_integer},
        http_service,
    },
};

#[perseus::make_rx(ChroniclePageStateRx)]
pub struct ChroniclePageState {
    pub user_alias: String,
    pub chronicle_id: String,
    pub date_time: String,
}

#[perseus::template_rx]
pub fn chronicle_page(
    ChroniclePageStateRx {
        user_alias,
        chronicle_id,
        date_time,
    }: ChroniclePageStateRx,
) -> View<G> {
    if G::IS_BROWSER {
        perseus::spawn_local(
            cloned!((user_alias, date_time, chronicle_id) => async move {
                let javascript_date: Date = Date::new_0();

                let day_of_week: String = get_day_of_week_from_integer(javascript_date.get_day());
                let date: u32 = javascript_date.get_date();
                let year: u32 = javascript_date.get_full_year();
                let month: String = get_month_from_integer(javascript_date.get_month());

                let time: JsString = Date::to_time_string(&javascript_date);
                let date_time_value: String = format!("{day_of_week} , {month} {date}, {year}");
                date_time.set(date_time_value);
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
                let cookies_string = html_document.cookie().unwrap();
                let cookie_value = cookies_string.split("; user_id=");
                for cookie in cookie_value {
                    info!("{cookie}");
                    let query_response = http_service::get_endpoint(API_CHRONICLE_TODAY_URL, ("user_id".to_string(), cookie.to_string()),None).await;

                }

            }),
        );

        perseus::spawn_local(cloned!( => async move {
        }));
    }

    view! {
            Container(ContainerProperties {
                title: String::from("Chronicle"),
                children: view! {
                    div(class="row flex-grow-1"){
                        div(class="col-10 bg-white p-4 shadow border-0 rounded") {
                            h1 { "Hello, " (user_alias.get()) }
                            h2 { (date_time.get()) }
                        }
                        div(class="col-2") {}
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
        chronicle_id: String::new(),
        date_time: String::new(),
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
