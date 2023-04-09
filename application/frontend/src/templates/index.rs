use perseus::{
    engine_only_fn, prelude::Html, state::StateGeneratorInfo, template::Template, ReactiveState,
};
use serde::{Deserialize, Serialize};
use sycamore::{
    prelude::{view, View},
    reactive::{create_signal, BoundedScope, Scope, Signal},
    web::SsrNode,
};
use web_sys::Event;

use crate::{
    components::container::{Container, ContainerProperties},
    components::widget::notification::toast::{Toast, ToastProperties, ToastVariant},
};

const PAGE_ROUTE_PATH: &str = "index";
const PAGE_TITLE: &str = "Welcome | Loremaster";

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
pub struct IndexPageState {
    pub greeting: String,
    pub current_tab: ToastVariant,
}

pub fn index_page<'page, G: Html>(
    context: BoundedScope<'_, 'page>,
    state: &'page IndexPageStateRx,
) -> View<G> {
    let click_first = |current_tab, event: Event| {
        event.prevent_default();
        current_tab.set(ToastVariant::Default);
    };
    let click_second = |current_tab| {
        current_tab.set(ToastVariant::Success);
    };
    let click_third = |current_tab| {
        current_tab.set(ToastVariant::Error);
    };
    let click_fourth = |current_tab| {
        current_tab.set(ToastVariant::Warning);
    };

    let content_input: Signal<String> = create_signal(context, String::new());
    let content = content_input.clone();

    let button_classes: &str = "glow-button";
    view! { context,
        Container(ContainerProperties{title: String::from("Loremaster"), children: view!{ context,
            div(class="d-flex flex-column flex-grow-1 p-4 align-items-center") {
                h1(class="display-3") { "Loremaster" }
                p() { (state.greeting.get()) }
                div(class="d-flex", id="lm-tab-test") {
                    button(class=button_classes, on:click=click_first) { "Default" }
                    button(class=button_classes, on:click=click_second) { "Success" }
                    button(class=button_classes, on:click=click_third) { "Error" }
                    button(class=button_classes, on:click=click_fourth) { "Warning" }
                }
                (match *state.current_tab.get() {
                    ToastVariant::Default => view! {context, div() {"Default"}},
                    ToastVariant::Success => view! {context, div() {"Success"}},
                    ToastVariant::Error => view! {context, div() {"Error"}},
                    ToastVariant::Warning => view! {context, div() {"Warning"}},
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
                    variant: state.current_tab
                })
            }
        }})
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build(PAGE_ROUTE_PATH)
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head_with_state(head)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    IndexPageState {
        greeting: "Welcome!".to_string(),
        current_tab: ToastVariant::Default,
    }
}

#[engine_only_fn]
fn head(context: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { context,
        title { (PAGE_TITLE) }
    }
}
