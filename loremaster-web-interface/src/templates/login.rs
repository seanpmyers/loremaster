use gloo_timers::future::TimeoutFuture;
use perseus::prelude::{navigate, spawn_local_scoped, Html};
use perseus::reactor::Reactor;
use perseus::state::StateGeneratorInfo;
use perseus::template::Template;
use perseus::{engine_only_fn, ReactiveState};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{view, Signal, View};
use sycamore::reactive::{create_signal, BoundedScope, Scope};
use sycamore::web::SsrNode;
use web_sys::Event;

use crate::components::container::Container;

use crate::components::form::input_validation::{InputValidation, InputValidationProperties};
use crate::components::icon::{PASSWORD_SVG_HTML, YUBIKEY_SVG_HTML};
use crate::components::navigation::tab::tab_button::TabButton;
use crate::components::navigation::tab::tab_panel::{TabIndex, TabPanel};
use crate::components::navigation::tab::tab_section::TabSection;
use crate::components::state::message_type::MessageType;
use crate::components::state::validation::Validation;
use crate::components::state::visibility::Visibility;
use crate::components::widget::data::form::web_authentication_api_login::WebAuthenticationAPILogin;
use crate::components::widget::notification::toast::{Toast, ToastProperties};
use crate::data::entity::person_meta::PersonMeta;
use crate::global_state::ApplicationStateRx;
use crate::utility::constants::{API_BASE_URL, API_LOGIN_URL, API_PERSON_META_DATA_ROUTE};
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
    LoginPageStateRx {
        email_address,
        password,
    }: &'page LoginPageStateRx,
) -> View<G> {
    let ApplicationStateRx { authentication } =
        Reactor::<G>::from_cx(context).get_global_state::<ApplicationStateRx>(context);

    let tab_panel_classes = create_signal(context, String::new());
    let tab_button_classes = create_signal(context, String::new());
    let tab_section_classes = create_signal(context, String::new());
    let active_tab: &Signal<TabIndex> = create_signal(context, 0_u32);

    let message_type: &Signal<MessageType> = create_signal(context, MessageType::Information);
    let toast_content: &Signal<String> = create_signal(context, String::new());

    let email_address_validation_content: &Signal<String> = create_signal(context, String::new());
    let email_address_validation_visibility: &Signal<Visibility> =
        create_signal(context, Visibility::Hidden);
    let email_address_validity: &Signal<Validation> = create_signal(context, Validation::Valid);
    let email_address_message_type: &Signal<MessageType> =
        create_signal(context, MessageType::Information);

    let loading: &Signal<bool> = create_signal(context, false);
    let form_message: &Signal<FormMessageState> = create_signal(context, FormMessageState::Hidden);

    let login_handler = move |event: Event| {
        event.prevent_default();
        spawn_local_scoped(context, async {
            if *loading.get() {
                return;
            }
            loading.set(true);

            if email_address.get().is_empty() {
                email_address_validation_content
                    .set(String::from("Email address cannot be empty."));
                email_address_validation_visibility.set(Visibility::Visible);
                email_address_message_type.set(MessageType::Error);
                email_address_validity.set(Validation::Invalid);
                loading.set(false);
                return;
            }

            if password.get().is_empty() {
                email_address_validation_content
                    .set(String::from("Email address cannot be empty."));
                email_address_validation_visibility.set(Visibility::Visible);
                email_address_message_type.set(MessageType::Error);
                email_address_validity.set(Validation::Invalid);
                loading.set(false);
                return;
            }

            email_address_validation_content.set(String::new());
            email_address_validation_visibility.set(Visibility::Hidden);
            email_address_message_type.set(MessageType::Information);
            email_address_validity.set(Validation::Valid);

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

                        let query_response: Option<String> = http_service::get_endpoint(
                            format!("{}/{}", API_BASE_URL, API_PERSON_META_DATA_ROUTE).as_str(),
                            None,
                        )
                        .await;
                        match query_response {
                            Some(response) => {
                                let person_meta_data: PersonMeta =
                                    serde_json::from_str(&response).unwrap();
                                email_address.set(person_meta_data.email_address);
                                if let Some(existing_alias) = person_meta_data.alias {
                                    authentication.update_user_alias(&existing_alias);
                                }
                            }
                            None => todo!(),
                        }

                        authentication.update_authentication();

                        message_type.set(MessageType::Success);
                        toast_content.set(String::from("Successfully logged in."));
                        form_message.set(FormMessageState::Visible);

                        TimeoutFuture::new(4000_u32).await;
                        navigate("/chronicle/");
                    }
                    _ => {
                        toast_content.set(String::from(
                            "Unable to login with the provided credentials.",
                        ));
                        message_type.set(MessageType::Error);
                        form_message.set(FormMessageState::Visible)
                    }
                },
                None => {
                    toast_content.set(String::from(
                        "Unable to login with the provided credentials.",
                    ));
                    message_type.set(MessageType::Error);
                    form_message.set(FormMessageState::Visible)
                }
            }
            loading.set(false);
        });
    };

    view! {context,
        Container(title="Login") {
            div(class="login-page") {
                div(class="") {
                    h3(class="") { "Login" }
                    TabPanel(active_tab=active_tab, classes=tab_panel_classes) {
                        div(class="tab-button-group") {
                            TabButton(title=String::from("Password/OPAQUE"), index=0_u32, classes=tab_button_classes, icon=Some(PASSWORD_SVG_HTML))
                            TabButton(title=String::from("WebAuthn"), index=1_u32, classes=tab_button_classes, icon=Some(YUBIKEY_SVG_HTML))
                        }
                        TabSection(title=String::from("tab1"), index=0_u32, classes=tab_section_classes){
                            form(on:submit=login_handler) {
                                div(class="input-row") {
                                    label(
                                        name="email_address",
                                        class="form-label") { "Email Address" }
                                    input(
                                        type="email",
                                        class="",
                                        bind:value=email_address,
                                        placeholder = "Enter your email address",
                                        disabled=loading.get().as_ref().to_owned()
                                    ) {}
                                    InputValidation(InputValidationProperties{
                                        content: email_address_validation_content,
                                        visibility: email_address_validation_visibility,
                                        validity: email_address_validity,
                                        message_type: email_address_message_type
                                    })
                                }
                                div(class="input-row") {
                                    label(
                                        name="password",
                                        class="form-label"
                                    ) { "Password" }
                                    input(
                                        type="password",
                                        class="",
                                        bind:value=password,
                                        placeholder = "Enter your password",
                                        disabled=loading.get().as_ref().to_owned()
                                    ) {}

                                }
                                button(
                                    class="btn btn-primary",
                                    type="submit",
                                    disabled=loading.get().as_ref().to_owned()
                                ) {
                                    "Submit"
                                }
                                (match *form_message.get() {
                                    FormMessageState::Hidden => view!{context, },
                                    FormMessageState::Visible => {
                                        return view! {context, Toast(ToastProperties{content: toast_content, message_type})};
                                    },
                                })
                            }
                        }
                        TabSection(title=String::from("tab2"), index=1_u32, classes=tab_section_classes){
                            WebAuthenticationAPILogin()
                        }
                    }
                }
            }
        }
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
