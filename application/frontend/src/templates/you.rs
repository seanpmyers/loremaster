use perseus::{RenderFnResultWithCause, Template};
use sycamore::prelude::{cloned, view, Html, Keyed, KeyedProps, Signal, SsrNode, View};
use web_sys::Event;

use crate::{
    components::container::{Container, ContainerProperties},
    data::entity::{action::Action, person_meta::PersonMeta},
    utility::{
        constants::{
            API_ACTION_LIST_ROUTE, API_ACTION_NEW_ROUTE, API_BASE_URL, API_PERSON_META_DATA_ROUTE,
            API_PERSON_META_UPDATE_ROUTE,
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
}

#[perseus::template_rx]
pub fn you_page(
    YouPageStateRx {
        email_address,
        alias,
        new_action,
        action_list,
    }: YouPageStateRx,
) -> View<G> {
    let email_address_input: Signal<String> = email_address.clone();
    let alias_input: Signal<String> = alias.clone();
    let new_action_input: Signal<String> = new_action.clone();
    if G::IS_BROWSER {
        perseus::spawn_local(cloned!((email_address, alias, action_list) => async move {

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
        }));
    }

    let save_handler = move |event: Event| {
        event.prevent_default();
        perseus::spawn_local(cloned!((email_address, alias) => async move {

            http_service::post_html_form(&format!("{}/{}",API_BASE_URL,API_PERSON_META_UPDATE_ROUTE), &vec![
                (String::from("email_address"), email_address.get().as_ref().to_string()),
                (String::from("alias"), alias.get().as_ref().to_string()),
            ]).await;

        }));
    };

    let save_action_handler = move |event: Event| {
        event.prevent_default();
        perseus::spawn_local(cloned!((new_action) => async move {
            http_service::post_html_form(&format!("{}/{}",API_BASE_URL,API_ACTION_NEW_ROUTE), &vec![
                (String::from("action"), new_action.get().as_ref().to_string()),
            ]).await;
            new_action.set(String::new());
        }));
    };

    let section_classes: &str = "border rounded border-success p-2 m-2";

    view! {
        Container(ContainerProperties{title: String::from("You"), children: view!{
            div(class="d-flex flex-column flex-grow-1 p-4 align-items-center") {
                div() {
                    h1(class="display-3") { "You" }
                    p() { "This is a page dedicated to you." }
                }
                div(class="d-flex") {
                    form(on:submit=save_handler, class=(section_classes)) {
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
                     form(on:submit=save_action_handler, class=(section_classes)) {
                        div(class="mb-3") {
                            label(class="form-label") {"New Action"}
                            input(
                                type="text",
                                class="form-control",
                                name="action",
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
                }
            }
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
        alias: String::new(),
        new_action: String::new(),
        action_list: Vec::new(),
    })
}

#[perseus::head]
pub fn head(_props: YouPageState) -> View<SsrNode> {
    view! {
        title { "You | Loremaster" }
    }
}
