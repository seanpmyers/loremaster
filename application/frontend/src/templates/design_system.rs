use perseus::{engine_only_fn, template::Template, web_log};
use sycamore::{
    prelude::*,
    reactive::{create_selector, create_signal, BoundedScope, ReadSignal, Scope, Signal},
};
use uuid::Uuid;
use web_sys::Event;

use crate::{
    components::{
        accordion::{Accordion, AccordionItem},
        combobox::{ComboBox, ComboBoxDatum, ComboBoxOption},
        navigation::tab::tab_panel::{TabIndex, TabPanel},
        navigation::tab::{tab_button::TabButton, tab_section::TabSection},
        switch::Switch,
        widget::theme_toggle::ThemeToggle,
    },
    data::entity::action::Action,
};

const PAGE_ROUTE_PATH: &str = "design-system";
const PAGE_TITLE: &str = "Design System | Chronilore";
const MAIN_HEADER: &str = "Chronilore Design System";

pub fn design_system_page<'page, G: Html>(context: BoundedScope<'_, 'page>) -> View<G> {
    let fake_actions: Vec<Action> = vec![
        Action {
            id: Uuid::nil(),
            name: String::from("Run"),
        },
        Action {
            id: Uuid::nil(),
            name: String::from("Sleep"),
        },
        Action {
            id: Uuid::nil(),
            name: String::from("Code"),
        },
        Action {
            id: Uuid::nil(),
            name: String::from("Play game"),
        },
    ];

    let empty_class: &Signal<String> = create_signal(context, String::from(""));

    let first_item: &Signal<String> = create_signal(context, String::from("First"));
    let second_item: &Signal<String> = create_signal(context, String::from("Second"));

    let active_tab: &Signal<TabIndex> = create_signal(context, 0_u32);
    let tab_panel_classes: &Signal<String> = create_signal(context, String::from("tab-panel"));
    let tab_button_classes: &Signal<String> = create_signal(context, String::from("tab-button"));
    let tab_section_classes: &Signal<String> = create_signal(context, String::from("tab-section"));

    let switch_on: &Signal<bool> = create_signal(context, false);
    let switch_classes: &Signal<String> = create_signal(context, String::from("switch"));

    let demo_container_classes: &Signal<String> = create_signal(context, String::from("card"));

    let selected: &Signal<Option<Uuid>> = create_signal(context, None);
    let query: &Signal<String> = create_signal(context, String::new());
    let other = create_signal(
        context,
        fake_actions
            .iter()
            .map(|action| action.clone().to_combobox_option().display_text)
            .collect::<Vec<_>>(),
    );
    let options: &ReadSignal<Vec<RcSignal<ComboBoxOption>>> =
        create_memo(context, move || query.get()).map(context, move |query| {
            let query: String = query.trim().to_lowercase();
            let result = fake_actions
                .iter()
                .map(|action| create_rc_signal(action.clone().to_combobox_option()))
                .filter(|datum| datum.get().display_text.to_lowercase().contains(&query))
                .collect::<Vec<_>>();
            let first = match result.first() {
                Some(value) => value.get().display_text.clone(),
                None => String::new(),
            };
            web_log!("{} {} {}", query, first, result.len());
            other.set(vec![first]);
            result
        });

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
                    div(class=demo_container_classes) {
                        ThemeToggle()
                    }
                }
                 div() {
                    h3() { "Accordion" }
                    div(class=demo_container_classes) {
                        Accordion(id=create_signal(context, String::from(""))) {
                            AccordionItem(title=first_item) {
                                div() { "Example body for first item"}
                            }
                            AccordionItem(title=second_item) {
                                div() { "Example body for second item"}
                            }
                        }
                    }
                }
                div() {
                    h3() { "Tab Panels" }
                    div(class=demo_container_classes) {
                        TabPanel(active_tab=active_tab, classes=tab_panel_classes) {
                            div(class="tab-button-group") {
                                TabButton(title=String::from("First"), index=0_u32, classes=tab_button_classes)
                                TabButton(title=String::from("Second"), index=1_u32, classes=tab_button_classes)
                                TabButton(title=String::from("Third"), index=2_u32, classes=tab_button_classes)
                            }
                            TabSection(title=String::from("tab1"), index=0_u32, classes=tab_section_classes){
                                div() {"First"}
                            }
                            TabSection(title=String::from("tab2"), index=1_u32, classes=tab_section_classes){
                                div() {"Second"}
                            }
                            TabSection(title=String::from("tab3"), index=2_u32, classes=tab_section_classes){
                                div() {"Third"}
                            }
                        }
                    }
                }
                div() {
                    h3() { "Switch" }
                    div(class=demo_container_classes) {
                        Switch(toggled=switch_on, classes=switch_classes)
                    }
                }
                div() {
                    h3() { "Popover" }
                    div(class=demo_container_classes) {

                    }
                }
                div() {
                    h3() { "Combobox" }
                    div(class=demo_container_classes) {
                        div() {
                            Indexed(
                                iterable=other,
                                view= |context, option| view! { context,
                                    div() { (option) }
                                },
                            )
                         }
                        ComboBox(query=query, selected=selected, options=options, classes=empty_class)
                    }
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
