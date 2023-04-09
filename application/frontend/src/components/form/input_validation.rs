use sycamore::prelude::*;

use crate::components::state::{
    message_type::{get_message_type_icon, MessageType},
    validation::Validation,
    visibility::Visibility,
};

#[derive(Prop)]
pub struct InputValidationProperties<'a> {
    pub content: &'a ReadSignal<String>,
    pub visibility: &'a ReadSignal<Visibility>,
    pub validity: &'a ReadSignal<Validation>,
    pub message_type: &'a ReadSignal<MessageType>,
}

#[component]
pub fn InputValidation<G: Html>(context: Scope, properties: InputValidationProperties) -> View<G> {
    let visibility: &ReadSignal<Visibility> = properties.visibility.clone();
    view! {context,
        (match *visibility.get(){
            Visibility::Visible => {
                let icon = properties.message_type.clone();
                let message_type = properties.message_type.clone();
                let content = properties.content.clone();
                view!{ context,
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
            Visibility::Hidden => view!{context,},
        })
    }
}
