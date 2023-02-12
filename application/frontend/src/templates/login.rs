use gloo_timers::future::TimeoutFuture;
use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{cloned, view, Signal, View};
use web_sys::Event;

use crate::components::container::{Container, ContainerProperties};

use crate::utility::constants::API_LOGIN_URL;
use crate::utility::http_service;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FormMessageState {
    Hidden,
    Success,
    Failure,
}

#[perseus::make_rx(LoginPageStateRx)]
pub struct LoginPageState {
    pub email_address: String,
    pub password: String,
}

#[perseus::template_rx]
pub fn login_page(state: LoginPageStateRx) -> View<G> {
    let loading: Signal<bool> = Signal::new(false);
    let loading_clone: Signal<bool> = loading.clone();
    let email_address: Signal<String> = state.email_address;
    let email_address_input: Signal<String> = email_address.clone();

    let password: Signal<String> = state.password;
    let password_input: Signal<String> = password.clone();

    let form_message: Signal<FormMessageState> = Signal::new(FormMessageState::Hidden);
    let form_message_display: Signal<FormMessageState> = form_message.clone();

    let login_handler = move |event: Event| {
        event.prevent_default();
        perseus::spawn_local(
            cloned!((email_address, password, form_message, loading) => async move {

                let potential_response: Option<reqwasm::http::Response> = http_service::post_html_form(&String::from(API_LOGIN_URL), &vec![
                    (String::from("email_address"), email_address.get().as_ref().to_string()),
                    (String::from("password"), password.get().as_ref().to_string()),
                ]).await;

                match potential_response {
                    Some(response) => {
                        match response.status() {
                            200 => {
                                form_message.set(FormMessageState::Success);
                                TimeoutFuture::new(4000_u32).await;
                                perseus::navigate("/chronicle/");
                            },
                            _ => form_message.set(FormMessageState::Failure),
                        }

                    },
                    None => form_message.set(FormMessageState::Failure),
                }
            }),
        );
    };

    view! {
        Container(ContainerProperties{
            title: String::from("Login"),
            children: view! {
                div(class="container shadow card p-4 border-0 rounded text-black") {
                    div(class="card-body") {
                        h3(class="card-title display-6") {"Login"}
                        form(on:submit=login_handler) {
                            div(class="mb-3") {
                                label(
                                    name="email_address",
                                    class="form-label") { "Email Address" }
                                input(
                                    type="email",
                                    class="form-control",
                                    bind:value= email_address_input,
                                    placeholder = "Enter your email address"
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
                                    placeholder = "Enter your password"
                                ) {}
                            }
                            button(class="btn btn-primary", type="submit"){ "Submit"}
                        }
                        (match *form_message_display.get() {
                            FormMessageState::Hidden => view!{ div() {}},
                            FormMessageState::Success => view!{ div(class="badge bg-success rounded") {"Successfully logged in."}},
                            FormMessageState::Failure => view!{ div(class="badge bg-danger rounded") {"Unable to login with the provided credentials."}}
                        })
                    }
                }
            }
        })
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("login")
        .build_state_fn(get_build_state)
        .template(login_page)
        .head(head)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<LoginPageState> {
    Ok(LoginPageState {
        email_address: String::new(),
        password: String::new(),
    })
}

#[perseus::head]
pub fn head(_props: LoginPageState) -> View<SsrNode> {
    view! {
        title { "Login - Loremaster " }
    }
}
