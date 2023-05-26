use perseus::{engine_only_fn, template::Template};
use sycamore::{
    prelude::{view, Html, SsrNode, View},
    reactive::{create_signal, BoundedScope, Scope, Signal},
};

use crate::components::{
    accordion::{Accordion, AccordionItem},
    widget::theme_toggle::ThemeToggle,
};

const PAGE_ROUTE_PATH: &str = "design-system";
const PAGE_TITLE: &str = "Design System | Chronilore";
const MAIN_HEADER: &str = "Chronilore Design System";

pub fn design_system_page<'page, G: Html>(context: BoundedScope<'_, 'page>) -> View<G> {
    let first_item: &Signal<String> = create_signal(context, String::from("First"));
    let second_item: &Signal<String> = create_signal(context, String::from("Second"));
    view! {context,
        div(class="") {
            h1(class="") { (MAIN_HEADER) }
            p() {
                "Chronilore's set of design standards intended to manage and guide all design work done by Chronilore."
                br() {}
                "Test"
             }
             div() {
                h3() { "Theme Toggle" }
                ThemeToggle()
             }
             div() {
                h3() { "Accordion" }
                Accordion(id=create_signal(context, String::from(""))) {
                    AccordionItem(title=first_item) {
                        div() { "Example body for first item"}
                    }
                    AccordionItem(title=second_item) {
                        div() { "Example body for second item"}
                    }
                }
            }
            div() {
                h3() { "Tab Panels" }
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
