use sycamore::prelude::*;

use crate::components::state::{
    message_type::{get_message_type_icon, MessageType},
    validation::Validation,
    visibility::Visibility,
};

pub struct InputValidationProperties {
    pub content: Signal<String>,
    pub visibility: Signal<Visibility>,
    pub validity: Signal<Validation>,
    pub message_type: Signal<MessageType>,
}

#[component(InputValidation<G>)]
pub fn input_validation(properties: InputValidationProperties) -> View<G> {
    let visibility: Signal<Visibility> = properties.visibility.clone();
    view! {
        (match *visibility.get(){
            Visibility::Visible => {
                let icon = properties.message_type.clone();
                let message_type = properties.message_type.clone();
                let content = properties.content.clone();
                view!{
                    div(
                        class=(match *message_type.get() {
                            MessageType::Information => "input-validation",
                            MessageType::Success => "input-validation input-validation--success",
                            MessageType::Error => "input-validation input-validation--error",
                            MessageType::Warning => "input-validation input-validation--warning"
                        })
                    ) {
                        span(
                            class="input-validation-icon",
                            dangerously_set_inner_html=get_message_type_icon(&icon.get())
                        ) {

                        }
                        output(class="input-validation-content") { (content.get()) }
                    }
                }
            },
            Visibility::Hidden => view!{},
        })
    }
}
