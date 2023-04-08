use perseus::Template;
use sycamore::prelude::{view, Html, SsrNode, View};

use crate::components::container::{Container, ContainerProperties};

const PAGE_TITLE: &str = "About | Loremaster";
const PAGE_ROUTE: &str = "about";

#[perseus::template_rx]
pub fn about_page() -> View<G> {
    view! {
        Container(ContainerProperties{title: String::from("About"), children: view!{
            div(class="d-flex flex-column flex-grow-1 p-4 align-items-center") {
                h1(class="display-3") { "About" }
                p() { "This is a website." }
            }
        }})
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { (PAGE_TITLE) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new(PAGE_ROUTE).template(about_page).head(head)
}
