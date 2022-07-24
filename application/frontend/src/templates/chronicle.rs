use perseus::Template;
use sycamore::prelude::{view, Html, SsrNode, View};

use crate::components::container::{Container, ContainerProperties};

#[perseus::template_rx]
pub fn chronicle_page() -> View<G> {
    view! {
            Container(ContainerProperties {
                title: String::from("Chronicle"),
                children: view! {
                    h1 { "Chronicle" }
            },
        })
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "Chronicle | Loremaster" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("chronicle")
        .template(chronicle_page)
        .head(head)
}
