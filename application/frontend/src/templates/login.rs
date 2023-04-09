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

use crate::components::container::{Container, ContainerProperties};

use crate::components::form::input_validation::{InputValidation, InputValidationProperties};
use crate::components::state::message_type::MessageType;
use crate::components::state::validation::Validation;
use crate::components::state::visibility::Visibility;
use crate::components::widget::notification::toast::{Toast, ToastProperties, ToastVariant};
use crate::utility::constants::API_LOGIN_URL;
use crate::utility::http_service;

const PAGE_ROUTE_PATH: &str = "login";
const PAGE_TITLE: &str = "Login - Loremaster";

#[derive(Deserialize, Serialize, Debug, Clone)]
enum FormMessageState {
    Hidden,
    Visible,
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "LoginPageStateRx")]
pub struct LoginPageState {
    pub email_address: String,
    pub password: String,
}

pub fn login_page<'page, G: Html>(
    context: BoundedScope<'_, 'page>,
    state: &'page LoginPageStateRx,
) -> View<G> {
    let toast_variant: &Signal<ToastVariant> = create_signal(context, ToastVariant::Default);
    let toast_content: &Signal<String> = create_signal(context, String::new());

    let variant = toast_variant.clone();
    let content = toast_content.clone();

    let email_address_validation_content: &Signal<String> = create_signal(context, String::new());
    let email_address_validation_visibility: &Signal<Visibility> =
        create_signal(context, Visibility::Hidden);
    let email_address_validity: &Signal<Validation> = create_signal(context, Validation::Valid);
    let email_address_message_type: &Signal<MessageType> =
        create_signal(context, MessageType::Information);

    let display_email_address_validation_content: &Signal<String> =
        email_address_validation_content.clone();
    let display_email_address_validation_visibility: &Signal<Visibility> =
        email_address_validation_visibility.clone();
    let display_email_address_validity: &Signal<Validation> = email_address_validity.clone();
    let display_email_address_message_type: &Signal<MessageType> =
        email_address_message_type.clone();

    let loading: &Signal<bool> = create_signal(context, false);
    let loading_email: &Signal<bool> = loading.clone();
    let loading_password: &Signal<bool> = loading.clone();
    let loading_submit: &Signal<bool> = loading.clone();

    let email_address: &Signal<String> = &state.email_address;
    let email_address_input: &Signal<String> = email_address.clone();

    let password: &Signal<String> = &state.password;
    let password_input: &Signal<String> = password.clone();

    let form_message: &Signal<FormMessageState> = create_signal(context, FormMessageState::Hidden);
    let form_message_display: &Signal<FormMessageState> = form_message.clone();

    let login_handler = |event: Event| {
        event.prevent_default();
        spawn_local_scoped(context, async move {
            if loading.get().as_ref() == &true {
                return;
            }
            loading.set(true);

            if email_address.get().is_empty() || password.get().is_empty() {
                email_address_validation_content
                    .set(String::from("Email address cannot be empty."));
                email_address_validation_visibility.set(Visibility::Visible);
                email_address_message_type.set(MessageType::Error);
                email_address_validity.set(Validation::Invalid);
                loading.set(false);
                return;
            } else {
                email_address_validation_content.set(String::new());
                email_address_validation_visibility.set(Visibility::Hidden);
                email_address_message_type.set(MessageType::Information);
                email_address_validity.set(Validation::Valid);
            }

            let potential_response: Option<reqwasm::http::Response> = http_service::post_html_form(
                &String::from(API_LOGIN_URL),
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
                    200 => {
                        email_address.set(String::new());
                        password.set(String::new());

                        toast_variant.set(ToastVariant::Success);
                        toast_content.set(String::from("Successfully logged in."));
                        form_message.set(FormMessageState::Visible);

                        TimeoutFuture::new(4000_u32).await;
                        navigate("/chronicle/");
                    }
                    _ => {
                        toast_content.set(String::from(
                            "Unable to login with the provided credentials.",
                        ));
                        toast_variant.set(ToastVariant::Error);
                        form_message.set(FormMessageState::Visible)
                    }
                },
                None => {
                    toast_content.set(String::from(
                        "Unable to login with the provided credentials.",
                    ));
                    toast_variant.set(ToastVariant::Error);
                    form_message.set(FormMessageState::Visible)
                }
            }
            loading.set(false);
        });
    };

    view! {context,
        Container(ContainerProperties{
            title: String::from("Login"),
            children: view! {context,
                div(class="") {
                    div(class="card-body") {
                        h3(class="card-title display-6") {"Login"}
                        form(on:submit=login_handler) {
                            div(class="input-row") {
                                label(
                                    name="email_address",
                                    class="form-label") { "Email Address" }
                                input(
                                    type="email",
                                    class="form-control",
                                    bind:value= email_address_input,
                                    placeholder = "Enter your email address",
                                    disabled=loading_email.get().as_ref().to_owned()
                                ) {}
                                InputValidation(InputValidationProperties{
                                    content: display_email_address_validation_content,
                                    visibility: display_email_address_validation_visibility,
                                    validity: display_email_address_validity,
                                    message_type: display_email_address_message_type
                                })
                            }
                            div(class="input-row") {
                                label(
                                    name="password",
                                    class="form-label"
                                ) { "Password" }
                                input(
                                    type="password",
                                    class="form-control",
                                    bind:value= password_input,
                                    placeholder = "Enter your password",
                                    disabled=loading_password.get().as_ref().to_owned()
                                ) {}

                            }
                            button(
                                class="btn btn-primary",
                                type="submit",
                                disabled=loading_submit.get().as_ref().to_owned()
                            ) {
                                "Submit"
                            }
                            (match *form_message_display.get() {
                                FormMessageState::Hidden => view!{context, },
                                FormMessageState::Visible => {
                                    let content = content.clone();
                                    let variant = variant.clone();
                                    return view! {context, Toast(ToastProperties{content, variant})};
                                },
                            })
                        }
                    }
                }
            }
        })
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build(PAGE_ROUTE_PATH)
        .build_state_fn(get_build_state)
        .view_with_state(login_page)
        .head_with_state(head)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> LoginPageState {
    LoginPageState {
        email_address: String::new(),
        password: String::new(),
    }
}

#[engine_only_fn]
fn head(context: Scope, _props: LoginPageState) -> View<SsrNode> {
    view! { context,
        title { (PAGE_TITLE) }
    }
}
