use gloo_timers::future::TimeoutFuture;
use perseus::{
    engine_only_fn, prelude::spawn_local_scoped, state::StateGeneratorInfo, template::Template,
    ReactiveState,
};
use serde::{Deserialize, Serialize};
use sycamore::{
    prelude::{view, Html, Keyed, KeyedProps, Signal, SsrNode, View},
    reactive::{create_signal, BoundedScope, Scope},
};
use time::{format_description::FormatItem, macros::format_description};
use web_sys::Event;

use crate::{
    components::{
        container::{Container, ContainerProperties},
        widget::notification::alert::{Alert, AlertProperties},
        widget::{
            frequency_options::{FrequencyOptions, FrequencyOptionsProperties},
            goal_list::{GoalList, GoalListProperties},
        },
    },
    data::entity::{action::Action, person_meta::PersonMeta, sleep_schedule::SleepSchedule},
    utility::{
        constants::{
            API_ACTION_LIST_ROUTE, API_ACTION_NEW_ROUTE, API_BASE_URL, API_GOAL_NEW_ROUTE,
            API_PERSON_EMAIL_ADDRESS_UPDATE_ROUTE, API_PERSON_META_DATA_ROUTE,
            API_PERSON_META_UPDATE_ROUTE, API_PERSON_SLEEP_SCHEDULE_ROUTE,
            API_PERSON_SLEEP_SCHEDULE_UPDATE_ROUTE,
        },
        http_service,
    },
};

const PAGE_ROUTE_PATH: &str = "you";
const PAGE_TITLE: &str = "You | Loremaster";

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "YouPageStateRx")]
pub struct YouPageState {
    pub email_address: String,
    pub alias: String,
    pub new_action: String,
    pub action_list: Vec<Action>,
    pub sleep_start: String,
    pub sleep_end: String,
}

pub fn you_page<'page, G: Html>(
    context: BoundedScope<'_, 'page>,
    YouPageStateRx {
        email_address,
        alias,
        new_action,
        action_list,
        sleep_start,
        sleep_end,
    }: &'page YouPageStateRx,
) -> View<G> {
    let login_success: &Signal<Option<bool>> = create_signal(context, None);
    let login_display: &Signal<Option<bool>> = login_success.clone();

    let new_goal: &Signal<String> = create_signal(context, String::new());
    let new_goal_input: &Signal<String> = &new_goal.clone();

    if G::IS_BROWSER {
        spawn_local_scoped(context, async move {
            let mut query_response: Option<String> = http_service::get_endpoint(
                format!("{}/{}", API_BASE_URL, API_PERSON_META_DATA_ROUTE).as_str(),
                None,
            )
            .await;
            match query_response {
                Some(response) => {
                    let person_meta_data: PersonMeta = serde_json::from_str(&response).unwrap();
                    email_address.set(person_meta_data.email_address);
                    if let Some(existing_alias) = person_meta_data.alias {
                        alias.set(existing_alias);
                    }
                }
                None => {}
            }
            query_response = http_service::get_endpoint(
                format!("{}/{}", API_BASE_URL, API_ACTION_LIST_ROUTE).as_str(),
                None,
            )
            .await;
            match query_response {
                Some(response) => {
                    let mut action_list_data: Vec<Action> =
                        serde_json::from_str(&response).unwrap();
                    action_list_data.iter_mut().for_each(|action| {
                        action.name =
                            action.name.remove(0).to_ascii_uppercase().to_string() + &action.name
                    });
                    action_list.set(action_list_data);
                }
                None => {}
            }

            query_response =
                http_service::get_endpoint(API_PERSON_SLEEP_SCHEDULE_ROUTE, None).await;
            match query_response {
                Some(response) => {
                    let potential_sleep_schedule: Option<SleepSchedule> =
                        serde_json::from_str(&response).unwrap();
                    match potential_sleep_schedule {
                        Some(schedule) => {
                            let format: &[FormatItem] = format_description!("[hour]:[minute]");
                            sleep_start.set(schedule.start_time.format(&format).unwrap());
                            sleep_end.set(schedule.end_time.format(&format).unwrap());
                        }
                        None => (),
                    }
                }
                None => (),
            }
        });
    }

    let update_email_address_handler = move |event: Event| {
        event.prevent_default();
        spawn_local_scoped(context, async move {
            http_service::post_html_form(
                &format!("{}/{}", API_BASE_URL, API_PERSON_EMAIL_ADDRESS_UPDATE_ROUTE),
                &vec![(
                    String::from("email_address"),
                    email_address.get().as_ref().to_string(),
                )],
            )
            .await;
        });
    };

    let update_meta_handler = move |event: Event| {
        event.prevent_default();
        spawn_local_scoped(context, async move {
            http_service::post_html_form(
                &format!("{}/{}", API_BASE_URL, API_PERSON_META_UPDATE_ROUTE),
                &vec![(String::from("alias"), alias.get().as_ref().to_string())],
            )
            .await;
        });
    };

    let new_action_handler = move |event: Event| {
        event.prevent_default();
        spawn_local_scoped(context, async move {
            http_service::post_html_form(
                &format!("{}/{}", API_BASE_URL, API_ACTION_NEW_ROUTE),
                &vec![(
                    String::from("action"),
                    new_action.get().as_ref().to_string(),
                )],
            )
            .await;
            new_action.set(String::new());

            login_success.set(Some(true));
            TimeoutFuture::new(10000_u32).await;
            login_success.set(None);
        });
    };

    let new_goal_handler = move |event: Event| {
        event.prevent_default();
        spawn_local_scoped(context, async move {
            http_service::post_html_form(
                &format!("{}/{}", API_BASE_URL, API_GOAL_NEW_ROUTE),
                &vec![(String::from("goal"), new_goal.get().as_ref().to_string())],
            )
            .await;
            new_goal.set(String::new());
        })
    };

    let update_sleep_schedule_handler = move |event: Event| {
        event.prevent_default();
        spawn_local_scoped(context, async move {
            http_service::post_html_form(
                &format!(
                    "{}/{}",
                    API_BASE_URL, API_PERSON_SLEEP_SCHEDULE_UPDATE_ROUTE
                ),
                &vec![
                    (
                        String::from("start_time"),
                        sleep_start.get().as_ref().to_string(),
                    ),
                    (
                        String::from("end_time"),
                        sleep_end.get().as_ref().to_string(),
                    ),
                ],
            )
            .await;
        });
    };

    let section_classes: &str = "border rounded bg-white shadow-sm p-2 m-2 ";

    view! {context,
        Container(ContainerProperties{title: String::from("You"), children: view!{context,
            div(class="d-flex flex-column flex-grow-1 p-4 align-items-center bg-light") {
                div() {
                    h1(class="display-3") { ( alias.get()) }
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
                                bind:value=email_address,
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
                                bind:value=alias,
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
                                bind:value=new_action,
                                placeholder="Enter a new action"
                            ) {}
                        }
                        div(class="mb-3") {
                            button(class="btn btn-primary", type="submit") { "Add" }
                        }
                     }
                    form(on:submit=new_goal_handler, class=(section_classes)) {
                        div(class="mb-3") {
                            label(class="form-label") {"New Goal"}
                            input(
                                type="text",
                                class="form-control",
                                name="goal",
                                minLength="1",
                                bind:value=new_goal_input,
                                placeholder="Enter a new goal"
                            ) {}
                        }
                    div(class="mb-3") {
                        button(class="btn btn-primary", type="submit") { "Add" }
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
                                    iterable: action_list,
                                    view: |context, action| view! {context,
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
                            label(class="form-label") {"Frequency"}
                            FrequencyOptions(FrequencyOptionsProperties{})
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
                                bind:value=sleep_start
                            ) {}
                        }
                        div(class="mb-3") {
                            label(class="form-label") {"End Hour"}
                            input(
                                type="time",
                                class="form-control",
                                name="end_time",
                                bind:value=sleep_end
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
                        div() { "Actions" }
                        ul() {
                            Keyed(KeyedProps {
                                iterable: action_list,
                                view: |context, action| view! {context,
                                    li() { (action.name) }
                                },
                                key: |action| action.id
                            })
                         }
                     }
                    div(class=(section_classes)) {
                        div() { "Intentions" }
                        ul() {

                         }
                     }
                    div(class=(section_classes)) {
                        div() { "Goals" }
                        GoalList(GoalListProperties{goals: create_signal(context, Vec::new())})
                     }
                    div(class=(section_classes)) {
                        div() { "Values" }
                        ul() {

                         }
                    }
                }
            }
            (if login_display.get().is_some() {
                view! {context,
                    Alert(AlertProperties{
                        message_title: create_signal(context, String::from("Success!")),
                        message_body:create_signal(context, String::from("You have successfully updated your information.")),
                        display_time: create_signal(context, None),
                    })
                }
            }
            else {
                view!{context, div() {""}}
            })
        }})
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build(PAGE_ROUTE_PATH)
        .build_state_fn(get_build_state)
        .view_with_state(you_page)
        .head_with_state(head)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> YouPageState {
    YouPageState {
        email_address: String::new(),
        alias: String::from("You"),
        new_action: String::new(),
        action_list: Vec::new(),
        sleep_start: String::new(),
        sleep_end: String::new(),
    }
}

#[engine_only_fn]
fn head(context: Scope, _props: YouPageState) -> View<SsrNode> {
    view! { context,
        title { (PAGE_TITLE) }
    }
}
