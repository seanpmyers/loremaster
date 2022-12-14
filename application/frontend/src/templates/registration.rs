use gloo_timers::future::TimeoutFuture;
use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use sycamore::prelude::{cloned, view, Signal, View};
use web_sys::Event;

use crate::components::container::{Container, ContainerProperties};
use crate::components::widget::notification::alert::{Alert, AlertProperties};
use crate::utility::constants::API_REGISTER_URL;
use crate::utility::http_service;

#[perseus::make_rx(RegistrationPageStateRx)]
pub struct RegistrationPageState {
    pub email_address: String,
    pub password: String,
}

#[perseus::template_rx]
pub fn registration_page(state: RegistrationPageStateRx) -> View<G> {
    let registration_success: Signal<Option<bool>> = Signal::new(None);
    let registration_display: Signal<Option<bool>> = registration_success.clone();
    let email_address: Signal<String> = state.email_address;
    let email_address_input: Signal<String> = email_address.clone();

    let password: Signal<String> = state.password;
    let password_input: Signal<String> = password.clone();

    let registration_handler = move |event: Event| {
        event.prevent_default();
        perseus::spawn_local(
            cloned!((email_address, password, registration_success) => async move {

                http_service::post_html_form(&String::from(API_REGISTER_URL), &vec![
                    (String::from("email_address"), email_address.get().as_ref().to_string()),
                    (String::from("password"), password.get().as_ref().to_string()),
                ]).await;

                registration_success.set(Some(true));
                TimeoutFuture::new(10000_u32).await;
                registration_success.set(None);
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
                    }
                }
                (if registration_display.get().is_some() {
                    view! {
                        Alert(AlertProperties{
                            message_title: Signal::new(String::from("Success!")),
                            message_body: Signal::new(String::from("You have successfully registered.")),
                            display_time: Signal::new(None),
                        })
                    }
                }
                else {
                    view!{ div() {""}}
                })
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
