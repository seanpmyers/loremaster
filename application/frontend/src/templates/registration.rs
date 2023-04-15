use gloo_timers::future::TimeoutFuture;
use perseus::prelude::{navigate, spawn_local_scoped, Html};
use perseus::state::StateGeneratorInfo;
use perseus::template::Template;
use perseus::{engine_only_fn, ReactiveState};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{view, Signal, View};
use sycamore::reactive::{create_signal, BoundedScope, Scope};
use sycamore::web::SsrNode;
use web_sys::Event;

use crate::components::container::Container;
use crate::components::form::input_validation::InputValidation;
use crate::components::state::message_type::MessageType;
use crate::components::state::validation::Validation;
use crate::components::state::visibility::Visibility;
use crate::components::widget::data::form::security_key_authentication::SecurityKeyAuthentication;
use crate::utility::constants::{ACCEPTED_HTTP_STATUS_CODE, API_REGISTER_URL, OK_HTTP_STATUS_CODE};
use crate::utility::http_service;

const PAGE_ROUTE_PATH: &str = "registration";
const PAGE_TITLE: &str = "Registration - Loremaster";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FormMessageState {
    Hidden,
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "RegistrationPageStateRx")]
pub struct RegistrationPageState {
    pub email_address: String,
    pub password: String,
}

pub fn registration_page<'page, G: Html>(
    context: BoundedScope<'_, 'page>,
    state: &'page RegistrationPageStateRx,
) -> View<G> {
    let loading: &Signal<bool> = create_signal(context, false);
    let email_address: &Signal<String> = &state.email_address;
    let password: &Signal<String> = &state.password;
    let form_message: &Signal<FormMessageState> = create_signal(context, FormMessageState::Hidden);

    let email_address_validation_content: &Signal<String> = create_signal(context, String::new());
    let email_address_validation_visibility: &Signal<Visibility> =
        create_signal(context, Visibility::Hidden);
    let email_address_validity: &Signal<Validation> = create_signal(context, Validation::Valid);
    let email_address_message_type: &Signal<MessageType> =
        create_signal(context, MessageType::Information);

    let registration_handler = move |event: Event| {
        event.prevent_default();
        spawn_local_scoped(context, async move {
            if loading.get().as_ref() == &true {
                return;
            }

            if email_address.get().is_empty() {
                email_address_validation_content
                    .set(String::from("Email address cannot be empty."));
                email_address_validation_visibility.set(Visibility::Visible);
                email_address_message_type.set(MessageType::Error);
                email_address_validity.set(Validation::Invalid);
                loading.set(false);
                return;
            }

            loading.set(true);
            let potential_response = http_service::post_html_form(
                &String::from(API_REGISTER_URL),
                &vec![
                    (
                        String::from("email_address"),
                        email_address.get().as_ref().to_string(),
                    ),
                    (
                        String::from("password"),
                        password.get().as_ref().to_string(),
                    ),
                ],
            )
            .await;

            match potential_response {
                Some(response) => match response.status() {
                    OK_HTTP_STATUS_CODE | ACCEPTED_HTTP_STATUS_CODE => {
                        form_message.set(FormMessageState::Success);
                        email_address.set(String::new());
                        password.set(String::new());
                        TimeoutFuture::new(4000_u32).await;
                        navigate("/login/");
                    }
                    _ => form_message.set(FormMessageState::Failure),
                },
                None => form_message.set(FormMessageState::Failure),
            }
            loading.set(false);
        });
    };

    view! {context,
        Container(title="Registration") {
            div(class="card registration-form") {
                div(class="card-body") {
                    h3(class="card-title display-6") {"Registration"}
                    form(on:submit=registration_handler) {
                        div(class="input-row") {
                            label(
                                name="email_address",
                                class="form-label") { "Email Address" }
                            input(
                                type="email",
                                class="form-control",
                                bind:value= email_address,
                                placeholder = "Enter your email address",
                                disabled=*loading.get()
                            ) {}
                            InputValidation(
                                content= email_address_validation_content,
                                visibility= email_address_validation_visibility,
                                validity= email_address_validity,
                                message_type= email_address_message_type)
                            }
                        div(class="input-row") {
                            label(
                                name="password",
                                class="form-label"
                            ) { "Password" }
                            input(
                                type="password",
                                class="form-control",
                                bind:value= password,
                                placeholder = "Enter your password",
                                disabled=*loading.get()
                            ) {}
                        }
                        button(class="btn btn-primary", type="submit", disabled=*loading.get()){ "Submit"}
                    }
                    (match *form_message.get() {
                        FormMessageState::Hidden => view!{context,  div() {}},
                        FormMessageState::Success => view!{context, div(class="badge bg-success rounded") {"Successfully registered."}},
                        FormMessageState::Failure => view!{context, div(class="badge bg-danger rounded") {"Unable to register with the provided credentials."}}
                    })
                    SecurityKeyAuthentication()
                }
            }
            }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build(PAGE_ROUTE_PATH)
        .build_state_fn(get_build_state)
        .view_with_state(registration_page)
        .head_with_state(head)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> RegistrationPageState {
    RegistrationPageState {
        email_address: String::new(),
        password: String::new(),
    }
}

#[engine_only_fn]
fn head(context: Scope, _props: RegistrationPageState) -> View<SsrNode> {
    view! { context,
        title { (PAGE_TITLE) }
    }
}
