use gloo_timers::future::TimeoutFuture;
use perseus::{RenderFnResultWithCause, Template};
use sycamore::prelude::{cloned, view, Html, Keyed, KeyedProps, Signal, SsrNode, View};
use time::{format_description::FormatItem, macros::format_description};
use web_sys::Event;

use crate::{
    components::{
        container::{Container, ContainerProperties},
        widget::notification::alert::{Alert, AlertProperties},
    },
    data::entity::{action::Action, person_meta::PersonMeta, sleep_schedule::SleepSchedule},
    utility::{
        constants::{
            API_ACTION_LIST_ROUTE, API_ACTION_NEW_ROUTE, API_BASE_URL,
            API_PERSON_EMAIL_ADDRESS_UPDATE_ROUTE, API_PERSON_META_DATA_ROUTE,
            API_PERSON_META_UPDATE_ROUTE, API_PERSON_SLEEP_SCHEDULE_ROUTE,
            API_PERSON_SLEEP_SCHEDULE_UPDATE_ROUTE,
        },
        http_service,
    },
};

#[perseus::make_rx(YouPageStateRx)]
pub struct YouPageState {
    pub email_address: String,
    pub alias: String,
    pub new_action: String,
    pub action_list: Vec<Action>,
    pub sleep_start: String,
    pub sleep_end: String,
}

#[perseus::template_rx]
pub fn you_page(
    YouPageStateRx {
        email_address,
        alias,
        new_action,
        action_list,
        sleep_start,
        sleep_end,
    }: YouPageStateRx,
) -> View<G> {
    let login_success: Signal<Option<bool>> = Signal::new(None);
    let login_display: Signal<Option<bool>> = login_success.clone();

    let email_address_input: Signal<String> = email_address.clone();
    let alias_input: Signal<String> = alias.clone();
    let display_alias: Signal<String> = alias.clone();
    let new_action_input: Signal<String> = new_action.clone();
    let sleep_start_input: Signal<String> = sleep_start.clone();
    let sleep_end_input: Signal<String> = sleep_end.clone();

    if G::IS_BROWSER {
        perseus::spawn_local(
            cloned!((email_address, alias, action_list, sleep_start, sleep_end) => async move {

                let mut query_response: Option<String> = http_service::get_endpoint(format!("{}/{}",API_BASE_URL,API_PERSON_META_DATA_ROUTE).as_str(), None).await;
                match query_response {
                    Some(response) => {
                        let person_meta_data: PersonMeta = serde_json::from_str(&response).unwrap();
                        email_address.set(person_meta_data.email_address);
                        if let Some(existing_alias) = person_meta_data.alias {
                            alias.set(existing_alias);
                        }
                    },
                    None => {},
                }
                query_response= http_service::get_endpoint(format!("{}/{}",API_BASE_URL,API_ACTION_LIST_ROUTE).as_str(), None).await;
                match query_response {
                    Some(response) => {
                        let action_list_data: Vec<Action> = serde_json::from_str(&response).unwrap();
                        action_list.set(action_list_data);
                    },
                    None => {},
                }

                query_response = http_service::get_endpoint(API_PERSON_SLEEP_SCHEDULE_ROUTE, None).await;
                    match query_response {
                        Some(response) => {
                            let potential_sleep_schedule: Option<SleepSchedule> = serde_json::from_str(&response).unwrap();
                            match potential_sleep_schedule {
                                Some(schedule) => {
                                    let format: &[FormatItem] = format_description!("[hour]:[minute]");
                                    sleep_start.set(schedule.start_time.format(&format).unwrap());
                                    sleep_end.set(schedule.end_time.format(&format).unwrap());
                                },
                                None => (),
                            }
                        },
                        None => (),
                    }

            }),
        );
    }

    let update_email_address_handler = move |event: Event| {
        event.prevent_default();
        perseus::spawn_local(cloned!((email_address) => async move {
            http_service::post_html_form(&format!("{}/{}",API_BASE_URL,API_PERSON_EMAIL_ADDRESS_UPDATE_ROUTE), &vec![
                (String::from("email_address"), email_address.get().as_ref().to_string()),
            ]).await;

        }));
    };

    let update_meta_handler = move |event: Event| {
        event.prevent_default();
        perseus::spawn_local(cloned!((alias) => async move {
            http_service::post_html_form(&format!("{}/{}",API_BASE_URL,API_PERSON_META_UPDATE_ROUTE), &vec![
                (String::from("alias"), alias.get().as_ref().to_string()),
            ]).await;

        }));
    };

    let new_action_handler = move |event: Event| {
        event.prevent_default();
        perseus::spawn_local(cloned!((new_action, login_success) => async move {
            http_service::post_html_form(&format!("{}/{}",API_BASE_URL,API_ACTION_NEW_ROUTE), &vec![
                (String::from("action"), new_action.get().as_ref().to_string()),
            ]).await;
            new_action.set(String::new());

            login_success.set(Some(true));
            TimeoutFuture::new(10000_u32).await;
            login_success.set(None);
        }));
    };

    let update_sleep_schedule_handler = move |event: Event| {
        event.prevent_default();
        perseus::spawn_local(cloned!((sleep_start, sleep_end) => async move {
            http_service::post_html_form(&format!("{}/{}",API_BASE_URL,API_PERSON_SLEEP_SCHEDULE_UPDATE_ROUTE), &vec![
                (String::from("start_time"), sleep_start.get().as_ref().to_string()),
                (String::from("end_time"), sleep_end.get().as_ref().to_string()),
            ]).await;

        }));
    };

    let section_classes: &str = "border rounded bg-white shadow-sm p-2 m-2 ";

    view! {
        Container(ContainerProperties{title: String::from("You"), children: view!{
            div(class="d-flex flex-column flex-grow-1 p-4 align-items-center bg-light") {
                div() {
                    h1(class="display-3") { ( display_alias.get()) }
                    p() { "This is a page dedicated to you." }
                }
                div(class="d-flex flex-wrap") {
                    form(on:submit=update_email_address_handler, class=(section_classes)) {
                        div(class="mb-3") {
                            label(class="form-label") {"Email Address"}
                            input(
                                type="email",
                                class="form-control",
                                name="email_address",
                                bind:value= email_address_input,
                                placeholder = "Enter your email address"
                            ) {}
                        }
                        div(class="mb-3") {
                            button(class="btn btn-primary", type="submit") { "Save" }
                        }
                     }
                     form(on:submit=update_meta_handler, class=(section_classes)) {
                        div(class="mb-3") {
                            label(class="form-label") {"Alias"}
                            input(
                                type="text",
                                name="alias",
                                class="form-control",
                                bind:value= alias_input,
                                placeholder = "Enter an alias"
                            ) {}
                        }
                        div(class="mb-3") {
                            button(class="btn btn-primary", type="submit") { "Save" }
                        }
                     }
                     form(on:submit=new_action_handler, class=(section_classes)) {
                        div(class="mb-3") {
                            label(class="form-label") {"New Action"}
                            input(
                                type="text",
                                class="form-control",
                                name="action",
                                minLength="1",
                                bind:value= new_action_input,
                                placeholder = "Enter a new action"
                            ) {}
                        }
                        div(class="mb-3") {
                            button(class="btn btn-primary", type="submit") { "Add" }
                        }
                     }
                     div(class=(section_classes)) {
                        div() { "Actions" }
                        ul() {
                            Keyed(KeyedProps {
                                iterable: action_list.handle(),
                                template: move |action| view! {
                                    li() { (action.name) }
                                },
                                key: |action| action.id
                            })
                         }
                     }
                     form(class=(section_classes)) {
                        div(class="mb-3") {
                            label(class="form-label") {"New Intention"}

                        }
                        div(class="mb-3") {
                            label(class="form-label") {"Select action"}
                            select(name="action", class="form-select") {
                                option(selected=true, disabled=true) { "Select an action" }
                                Keyed(KeyedProps {
                                    iterable: action_list.handle(),
                                    template: move |action| view! {
                                        option(value=(action.id)) { (action.name) }
                                    },
                                    key: |action| action.id
                                })
                            }
                        }
                        div(class="mb-3") {
                            label(class="form-label") {"Date"}
                            input(type="datetime-local",class="form-control") {}
                        }
                        div(class="mb-3") {
                            button(class="btn btn-primary", type="submit") { "Add" }
                        }
                     }
                    form(
                        class=(section_classes),
                        on:submit=update_sleep_schedule_handler
                    ) {
                        div(class="mb-3") {
                            label(class="form-label") {"Sleep Schedule"}
                        }
                        div(class="mb-3") {
                            label(class="form-label") {"Start Hour"}
                            input(
                                type="time",
                                class="form-control",
                                name="start_time",
                                bind:value=sleep_start_input
                            ) {}
                        }
                        div(class="mb-3") {
                            label(class="form-label") {"End Hour"}
                            input(
                                type="time",
                                class="form-control",
                                name="end_time",
                                bind:value=sleep_end_input
                            ) {}
                        }
                        div(class="mb-3") {
                            label(class="form-label") {"Total hours"}
                            input(type="number",class="form-control", disabled=true) {}
                        }
                        div(class="mb-3") {
                            button(class="btn btn-primary", type="submit") { "Save" }
                        }
                     }
                     div(class=(section_classes)) {
                        div() { "Intentions" }
                        ul() {

                         }
                     }
                     div(class=(section_classes)) {
                        div() { "Goals" }
                        ul() {

                         }
                     }
                }
            }
            (if login_display.get().is_some() {
                view! {
                    Alert(AlertProperties{
                        message_title: Signal::new(String::from("Success!")),
                        message_body: Signal::new(String::from("You have successfully updated your information.")),
                        display_time: Signal::new(None),
                    })
                }
            }
            else {
                view!{ div() {""}}
            })
        }})
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("you")
        .build_state_fn(get_build_state)
        .template(you_page)
        .head(head)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<YouPageState> {
    Ok(YouPageState {
        email_address: String::new(),
        alias: String::from("You"),
        new_action: String::new(),
        action_list: Vec::new(),
        sleep_start: String::new(),
        sleep_end: String::new(),
    })
}

#[perseus::head]
pub fn head(_props: YouPageState) -> View<SsrNode> {
    view! {
        title { "You | Loremaster" }
    }
}
