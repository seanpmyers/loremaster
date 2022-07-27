use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use sycamore::prelude::{view, View};

use crate::components::container::{Container, ContainerProperties};

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
}

#[perseus::template_rx]
pub fn index_page(state: IndexPageStateRx) -> View<G> {
    view! {
        Container(ContainerProperties{title: String::from("Home"), children: view!{
            h1() { "Loremaster" }
            p() { "Welcome!" }
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
        greeting: String::from(""),
    })
}

#[perseus::head]
pub fn head(_props: IndexPageState) -> View<SsrNode> {
    view! {
        title { "Welcome | Loremaster" }
    }
}