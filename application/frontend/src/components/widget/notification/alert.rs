use sycamore::prelude::*;
use time::Duration;

#[derive(Prop)]
pub struct AlertProperties<'a> {
    pub message_title: &'a ReadSignal<String>,
    pub message_body: &'a ReadSignal<String>,
    pub display_time: &'a ReadSignal<Option<Duration>>,
}

#[component]
pub fn Alert<G: Html>(
    context: Scope,
    AlertProperties {
        message_title,
        message_body,
        display_time,
    }: AlertProperties,
) -> View<G> {
    view! { context,
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
