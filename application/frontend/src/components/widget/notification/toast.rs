use sycamore::prelude::*;

use crate::components::{
    icon::{ALERT_OCTAGON_SVG_HTML, ALERT_TRIANGLE_SVG_HTML, CHECK_SVG_HTML, INFO_SVG_HTML},
    state::message_type::MessageType,
};

#[derive(Prop)]
pub struct ToastProperties<'a> {
    pub content: &'a ReadSignal<String>,
    pub message_type: &'a ReadSignal<MessageType>,
}

#[component]
pub fn Toast<'a, 'b: 'a, G: Html>(context: Scope<'a>, properties: ToastProperties<'b>) -> View<G> {
    view! { context,
        div(
            class=(match *properties.message_type.get() {
                MessageType::Information => "toast",
                MessageType::Success => "toast toast--success",
                MessageType::Error => "toast toast--error",
                MessageType::Warning => "toast toast--warning"
            })
        ) {
            span(
                class="toast-icon",
                dangerously_set_inner_html=match *properties.message_type.get() {
                    MessageType::Information => INFO_SVG_HTML,
                    MessageType::Success => CHECK_SVG_HTML,
                    MessageType::Error => ALERT_OCTAGON_SVG_HTML,
                    MessageType::Warning => ALERT_TRIANGLE_SVG_HTML
                }
            ) {

            }
            span(class="toast-content") { (properties.content.get()) }
        }
    }
}
