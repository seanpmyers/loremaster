use sycamore::prelude::*;
use time::Duration;

pub struct AlertProperties {
    pub message_title: Signal<String>,
    pub message_body: Signal<String>,
    pub display_time: Signal<Option<Duration>>,
}

#[component(Alert<G>)]
pub fn alert(
    AlertProperties {
        message_title,
        message_body,
        display_time,
    }: AlertProperties,
) -> View<G> {
    view! {
        div(class="notification-toast-alert", id="") {
            image(class="fi-check-circle") {}
            div() {
                h5(class="") { (message_title.get()) }
                p(class="") { (message_body.get()) }
            }
            button() { image(class="fi-x") {} }

        }
    }
}
