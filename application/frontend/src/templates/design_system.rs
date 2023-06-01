use perseus::{engine_only_fn, template::Template};
use sycamore::{
    prelude::{view, Html, SsrNode, View},
    reactive::{create_signal, BoundedScope, Scope, Signal},
};

use crate::components::{
    accordion::{Accordion, AccordionItem},
    navigation::tab::tab_panel::{TabIndex, TabPanel},
    navigation::tab::{tab_button::TabButton, tab_section::TabSection},
    switch::Switch,
    widget::theme_toggle::ThemeToggle,
};

const PAGE_ROUTE_PATH: &str = "design-system";
const PAGE_TITLE: &str = "Design System | Chronilore";
const MAIN_HEADER: &str = "Chronilore Design System";

pub fn design_system_page<'page, G: Html>(context: BoundedScope<'_, 'page>) -> View<G> {
    let first_item: &Signal<String> = create_signal(context, String::from("First"));
    let second_item: &Signal<String> = create_signal(context, String::from("Second"));

    let active_tab: &Signal<TabIndex> = create_signal(context, TabIndex(0));
    let tab_panel_classes: &Signal<String> = create_signal(context, String::from("tab-panel"));
    let tab_button_classes: &Signal<String> = create_signal(context, String::from("tab-button"));
    let tab_section_classes: &Signal<String> = create_signal(context, String::from("tab-section"));

    let switch_on: &Signal<bool> = create_signal(context, false);
    let switch_classes: &Signal<String> = create_signal(context, String::from("switch"));

    view! {context,
        div(class="design-system") {
            div(class="design-system-header") {
                h2(class="design-system-header") { (MAIN_HEADER) }
            }
            div(class="design-system-nav") {
                "nav"
            }
            div(class="design-system-content") {
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
                    div() {
                        TabPanel(active_tab=active_tab, classes=tab_panel_classes) {
                            div(class="tab-button-group") {
                                TabButton(title=String::from("First"), index=TabIndex(0), classes=tab_button_classes)
                                TabButton(title=String::from("Second"), index=TabIndex(1), classes=tab_button_classes)
                                TabButton(title=String::from("Third"), index=TabIndex(2), classes=tab_button_classes)
                            }
                            TabSection(title=String::from("tab1"), index=TabIndex(0), classes=tab_section_classes){
                                div() {"First"}
                            }
                            TabSection(title=String::from("tab2"), index=TabIndex(1), classes=tab_section_classes){
                                div() {"Second"}
                            }
                            TabSection(title=String::from("tab3"), index=TabIndex(2), classes=tab_section_classes){
                                div() {"Third"}
                            }
                        }
                    }
                }
                div() {
                    h3() { "Switch" }
                    Switch(on=switch_on, classes=switch_classes)
                }
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
