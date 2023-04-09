use perseus::{engine_only_fn, template::Template};
use sycamore::{
    prelude::{view, Html, SsrNode, View},
    reactive::{BoundedScope, Scope},
};

const PAGE_ROUTE_PATH: &str = "design-system";
const PAGE_TITLE: &str = "Design System | Chronilore";
const MAIN_HEADER: &str = "Chronilore Design System";

pub fn design_system_page<'page, G: Html>(context: BoundedScope<'_, 'page>) -> View<G> {
    view! {context,
        div(class="") {
            h1(class="") { (MAIN_HEADER) }
            p() {
                "Chronilore's set of design standards intended to manage and guide all design work done by Chronilore."
                br() {}
                "Test"
             }

        }
    }
}

#[engine_only_fn]
fn head(context: Scope) -> View<SsrNode> {
    view! { context,
        title { (PAGE_TITLE) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build(PAGE_ROUTE_PATH)
        .view(design_system_page)
        .head(head)
        .build()
}
