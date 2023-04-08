use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::components::icon::{
    ALERT_OCTAGON_SVG_HTML, ALERT_TRIANGLE_SVG_HTML, CHECK_SVG_HTML, INFO_SVG_HTML,
};

pub struct ToastProperties {
    pub content: Signal<String>,
    pub variant: Signal<ToastVariant>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ToastVariant {
    Default,
    Success,
    Error,
    Warning,
}

#[component(Toast<G>)]
pub fn toast(properties: ToastProperties) -> View<G> {
    let icon = properties.variant.clone();
    view! {
        div(
            class=(match *properties.variant.get() {
                ToastVariant::Default => "toast",
                ToastVariant::Success => "toast toast--success",
                ToastVariant::Error => "toast toast--error",
                ToastVariant::Warning => "toast toast--warning"
            })
        ) {
            span(
                class="toast-icon",
                dangerously_set_inner_html=match *icon.get() {
                    ToastVariant::Default => INFO_SVG_HTML,
                    ToastVariant::Success => CHECK_SVG_HTML,
                    ToastVariant::Error => ALERT_OCTAGON_SVG_HTML,
                    ToastVariant::Warning => ALERT_TRIANGLE_SVG_HTML
                }
            ) {

            }
            span(class="toast-content") { (properties.content.get()) }
        }
    }
}
