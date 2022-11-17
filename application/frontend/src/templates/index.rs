use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use serde::{Deserialize, Serialize};
use sycamore::{
    prelude::{view, View},
    reactive::cloned,
};

use crate::components::container::{Container, ContainerProperties};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Tab {
    First,
    Second,
    Third,
    Fourth,
}

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
    pub current_tab: Tab,
}

#[perseus::template_rx]
pub fn index_page(
    IndexPageStateRx {
        greeting,
        current_tab,
    }: IndexPageStateRx,
) -> View<G> {
    let click_first = cloned!((current_tab) => move |_| {
        current_tab.set(Tab::First);
    });
    let click_second = cloned!((current_tab) => move |_| {
        current_tab.set(Tab::Second);
    });
    let click_third = cloned!((current_tab) => move |_| {
        current_tab.set(Tab::Third);
    });
    let click_fourth = cloned!((current_tab) => move |_| {
        current_tab.set(Tab::Fourth);
    });
    view! {
        Container(ContainerProperties{title: String::from("Loremaster"), children: view!{
            div(class="d-flex flex-column flex-grow-1 p-4 align-items-center") {
                h1(class="display-3") { "Loremaster" }
                p() { (greeting.get()) }
                div(class="d-flex") {
                    button(class="p-1 btn btn-primary", on:click=click_first) { "First" }
                    button(class="p-1 btn btn-primary", on:click=click_second) { "Second" }
                    button(class="p-1 btn btn-primary", on:click=click_third) { "Third" }
                    button(class="p-1 btn btn-primary", on:click=click_fourth) { "Fourth" }
                }
                (match *current_tab.get() {
                    Tab::First => view! { div() {"First"}},
                    Tab::Second => view! { div() {"Second"}},
                    Tab::Third => view! { div() {"Third"}},
                    Tab::Fourth => view! { div() {"Fourth"}},
                })
            }
        }})
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
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
        current_tab: Tab::First,
    })
}

#[perseus::head]
pub fn head(_props: IndexPageState) -> View<SsrNode> {
    view! {
        title { "Welcome | Loremaster" }
    }
}
