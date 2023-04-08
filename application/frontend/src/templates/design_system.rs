use perseus::Template;
use sycamore::prelude::{view, Html, SsrNode, View};

const ROUTE_PATH: &str = "design-system";
const PAGE_TITLE: &str = "Design System | Chronilore";
const MAIN_HEADER: &str = "Chronilore Design System";

#[perseus::template_rx]
pub fn design_system_page() -> View<G> {
    view! {
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

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { (PAGE_TITLE) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new(ROUTE_PATH)
        .template(design_system_page)
        .head(head)
}
