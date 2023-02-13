use gloo_timers::future::TimeoutFuture;
use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{cloned, view, Signal, View};
use web_sys::Event;

use crate::components::container::{Container, ContainerProperties};
use crate::utility::constants::API_REGISTER_URL;
use crate::utility::http_service;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FormMessageState {
    Hidden,
    Success,
    Failure,
}

#[perseus::make_rx(RegistrationPageStateRx)]
pub struct RegistrationPageState {
    pub email_address: String,
    pub password: String,
}

#[perseus::template_rx]
pub fn registration_page(state: RegistrationPageStateRx) -> View<G> {
    let loading: Signal<bool> = Signal::new(false);
    let loading_email: Signal<bool> = loading.clone();
    let loading_password: Signal<bool> = loading.clone();
    let loading_submit: Signal<bool> = loading.clone();
    let email_address: Signal<String> = state.email_address;
    let email_address_input: Signal<String> = email_address.clone();

    let password: Signal<String> = state.password;
    let password_input: Signal<String> = password.clone();

    let form_message: Signal<FormMessageState> = Signal::new(FormMessageState::Hidden);
    let form_message_display: Signal<FormMessageState> = form_message.clone();

    let registration_handler = move |event: Event| {
        event.prevent_default();
        perseus::spawn_local(
            cloned!((email_address, password, loading, form_message) => async move {
                if loading.get().as_ref() == &true { return; }
                loading.set(true);
                let potential_response = http_service::post_html_form(&String::from(API_REGISTER_URL), &vec![
                    (String::from("email_address"), email_address.get().as_ref().to_string()),
                    (String::from("password"), password.get().as_ref().to_string()),
                ]).await;

                match potential_response {
                    Some(response) => {
                        match response.status() {
                            200 => {
                                form_message.set(FormMessageState::Success);
                                email_address.set(String::new());
                                password.set(String::new());
                                TimeoutFuture::new(4000_u32).await;
                                perseus::navigate("/login/");
                            },
                            _ => form_message.set(FormMessageState::Failure),
                        }

                    },
                    None => form_message.set(FormMessageState::Failure),
                }
                loading.set(false);
            }),
        );
    };

    view! {
        Container(ContainerProperties{
            title: String::from("Registration"),
            children: view! {
                div(class="container shadow card p-4 border-0 rounded text-black") {
                    div(class="card-body") {
                        h3(class="card-title display-6") {"Registration"}
                        form(on:submit=registration_handler) {
                            div(class="mb-3") {
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
                            }
                            div(class="mb-3") {
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
                            button(class="btn btn-primary", type="submit", disabled=loading_submit.get().as_ref().to_owned()){ "Submit"}
                        }
                        (match *form_message_display.get() {
                            FormMessageState::Hidden => view!{ div() {}},
                            FormMessageState::Success => view!{ div(class="badge bg-success rounded") {"Successfully registered."}},
                            FormMessageState::Failure => view!{ div(class="badge bg-danger rounded") {"Unable to register with the provided credentials."}}
                        })
                    }
                }
            }
        })
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("registration")
        .build_state_fn(get_build_state)
        .template(registration_page)
        .head(head)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<RegistrationPageState> {
    Ok(RegistrationPageState {
        email_address: String::new(),
        password: String::new(),
    })
}

#[perseus::head]
pub fn head(_props: RegistrationPageState) -> View<SsrNode> {
    view! {
        title { "Registration - Loremaster " }
    }
}
