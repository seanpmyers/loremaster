use perseus::Template;
use sycamore::prelude::{view, Html, SsrNode, View};

use crate::components::container::{Container, ContainerProperties};

#[perseus::template_rx]
pub fn about_page() -> View<G> {
    view! {
        Container(ContainerProperties{title: String::from("About"), children: view!{
            h1() { "About" }
            p { "This is a website." }
        }})
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "About | Loremaster" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}
