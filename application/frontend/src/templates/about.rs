use perseus::Template;
use sycamore::prelude::{view, Html, SsrNode, View};

use crate::components::navigation::NavigationLinks;

#[perseus::template_rx]
pub fn about_page() -> View<G> {
    view! {
        NavigationLinks()
        p { "About." }
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
