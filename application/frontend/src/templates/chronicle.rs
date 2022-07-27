use perseus::Template;
use sycamore::prelude::{view, Html, SsrNode, View};

use crate::components::container::{Container, ContainerProperties};

#[perseus::template_rx]
pub fn chronicle_page() -> View<G> {
    view! {
            Container(ContainerProperties {
                title: String::from("Chronicle"),
                children: view! {
                    div(class="row flex-grow-1"){
                        div(class="col-10 bg-white p-4 shadow border-0 rounded") {
                            h1 { "Chronicle" }
                        }
                        div(class="col-2") {}
                    }
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
