use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use sycamore::{
    prelude::{view, View},
    reactive::{cloned, Signal},
};
use web_sys::Event;

use crate::{
    components::container::{Container, ContainerProperties},
    components::widget::notification::toast::{Toast, ToastProperties, ToastVariant},
    global_state::AppStateRx,
};

const ROUTE_PATH: &str = "index";
const PAGE_TITLE: &str = "Welcome | Loremaster";

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
    pub current_tab: ToastVariant,
}

#[perseus::template_rx]
pub fn index_page(
    IndexPageStateRx {
        greeting,
        current_tab,
    }: IndexPageStateRx,
    global_state: AppStateRx,
) -> View<G> {
    let click_first = cloned!((current_tab) => move |event: Event| {
        event.prevent_default();
        current_tab.set(ToastVariant::Default);
    });
    let click_second = cloned!((current_tab) => move |_| {
        current_tab.set(ToastVariant::Success);
    });
    let click_third = cloned!((current_tab) => move |_| {
        current_tab.set(ToastVariant::Error);
    });
    let click_fourth = cloned!((current_tab) => move |_| {
        current_tab.set(ToastVariant::Warning);
    });

    let content_input: Signal<String> = Signal::new(String::new());
    let content = content_input.clone();

    let variant = current_tab.clone();

    let button_classes: &str = "glow-button";
    view! {
        Container(ContainerProperties{title: String::from("Loremaster"), children: view!{
            div(class="d-flex flex-column flex-grow-1 p-4 align-items-center") {
                h1(class="display-3") { "Loremaster" }
                p() { (greeting.get()) }
                div(class="d-flex", id="lm-tab-test") {
                    button(class=button_classes, on:click=click_first) { "Default" }
                    button(class=button_classes, on:click=click_second) { "Success" }
                    button(class=button_classes, on:click=click_third) { "Error" }
                    button(class=button_classes, on:click=click_fourth) { "Warning" }
                }
                (match *current_tab.get() {
                    ToastVariant::Default => view! { div() {"Default"}},
                    ToastVariant::Success => view! { div() {"Success"}},
                    ToastVariant::Error => view! { div() {"Error"}},
                    ToastVariant::Warning => view! { div() {"Warning"}},
                })
                div() {
                    button(class="popover-button glow-button") { "Test" }
                    div(class="popover") {
                        div() {
                            "Content"
                        }
                     }
                }
                input(bind:value=content_input) {}
                Toast(ToastProperties {
                    content,
                    variant
                })
            }
        }})
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new(ROUTE_PATH)
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        greeting: String::from("Welcome!"),
        current_tab: ToastVariant::Default,
    })
}

#[perseus::head]
pub fn head(_props: IndexPageState) -> View<SsrNode> {
    view! {
        title { (PAGE_TITLE) }
    }
}
