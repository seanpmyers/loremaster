use perseus::Template;
use sycamore::prelude::{view, Html, SsrNode, View};

use crate::components::container::{Container, ContainerProperties};

#[perseus::template_rx]
pub fn you_page() -> View<G> {
    view! {
        Container(ContainerProperties{title: String::from("You"), children: view!{
            div(class="d-flex flex-column flex-grow-1 p-4 align-items-center") {
                h1(class="display-3") { "You" }
                p() { "This is a page dedicated to you." }
            }
        }})
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "You | Loremaster" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("you").template(you_page).head(head)
}
