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
pub fn InputValidation<'a, 'b: 'a, G: Html>(
    context: Scope<'a>,
    InputValidationProperties {
        content,
        visibility,
        validity,
        message_type,
    }: InputValidationProperties<'b>,
) -> View<G> {
    view! {context,
        (match *visibility.get(){
            Visibility::Visible => {
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
                            dangerously_set_inner_html=get_message_type_icon(&message_type.get())
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
