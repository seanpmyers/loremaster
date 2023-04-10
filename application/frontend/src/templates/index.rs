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
    components::state::message_type::MessageType,
};

const PAGE_ROUTE_PATH: &str = "index";
const PAGE_TITLE: &str = "Welcome | Loremaster";

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
pub struct IndexPageState {
    pub greeting: String,
    pub current_tab: MessageType,
}

pub fn index_page<'page, G: Html>(
    context: BoundedScope<'_, 'page>,
    state: &'page IndexPageStateRx,
) -> View<G> {
    let current_tab: &Signal<MessageType> = create_signal(context, MessageType::Information);
    let click_first = |event: Event| {
        event.prevent_default();
        current_tab.set(MessageType::Information);
    };
    let click_second = |event: Event| {
        event.prevent_default();
        current_tab.set(MessageType::Success);
    };
    let click_third = |event: Event| {
        event.prevent_default();
        current_tab.set(MessageType::Error);
    };
    let click_fourth = |event: Event| {
        event.prevent_default();
        current_tab.set(MessageType::Warning);
    };

    let content_input: &Signal<String> = create_signal(context, String::new());

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
                    MessageType::Information => view! {context, div() {"Default"}},
                    MessageType::Success => view! {context, div() {"Success"}},
                    MessageType::Error => view! {context, div() {"Error"}},
                    MessageType::Warning => view! {context, div() {"Warning"}},
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
        current_tab: MessageType::Information,
    }
}

#[engine_only_fn]
fn head(context: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { context,
        title { (PAGE_TITLE) }
    }
}
