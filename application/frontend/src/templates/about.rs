use perseus::{engine_only_fn, template::Template};
use sycamore::{
    prelude::{view, Html, SsrNode, View},
    reactive::{BoundedScope, Scope},
};

use crate::components::container::{Container, ContainerProperties};

const PAGE_TITLE: &str = "About | Loremaster";
const PAGE_ROUTE_PATH: &str = "about";

pub fn about_page<'page, G: Html>(context: BoundedScope<'_, 'page>) -> View<G> {
    view! {context,
        Container(ContainerProperties{title: String::from("About"), children: view!{ context,
            div(class="d-flex flex-column flex-grow-1 p-4 align-items-center") {
                h1(class="display-3") { "About" }
                p() { "This is a website." }
            }
        }})
    }
}

#[engine_only_fn]
pub fn head(context: Scope) -> View<SsrNode> {
    view! {context,
        title { (PAGE_TITLE) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build(PAGE_ROUTE_PATH)
        .view(about_page)
        .head(head)
        .build()
}
