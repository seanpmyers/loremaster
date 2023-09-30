use perseus::{engine_only_fn, template::Template};
use sycamore::{
    prelude::*,
    reactive::{create_signal, BoundedScope, Scope, Signal},
};
use uuid::Uuid;

pub trait ComboBoxDatum {
    fn to_combobox_option(self) -> ComboBoxOption;
}

use crate::{
    components::{
        accordion::{Accordion, AccordionItem},
        combobox::{ComboBox, ComboBoxOption},
        container::Container,
        icon::{PASSWORD_SVG_HTML, YUBIKEY_SVG_HTML},
        modal::{Modal, ModalType},
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
const SYCAMORE_GITHUB_URL: &str = "https://github.com/sycamore-rs/sycamore";
const PERSEUS_GITHUB_URL: &str = "https://github.com/framesurge/perseus";

pub fn design_system_page<'page, G: Html>(context: BoundedScope<'_, 'page>) -> View<G> {
    let first_item: &Signal<String> = create_signal(context, String::from("First"));
    let second_item: &Signal<String> = create_signal(context, String::from("Second"));

    let active_tab: &Signal<TabIndex> = create_signal(context, 0_u32);
    let tab_panel_classes: &Signal<String> = create_signal(context, String::from("tab-panel"));
    let tab_button_classes: &Signal<String> = create_signal(context, String::from("tab-button"));
    let tab_section_classes: &Signal<String> = create_signal(context, String::from("tab-section"));

    let switch_on: &Signal<bool> = create_signal(context, false);
    let switch_classes: &Signal<String> = create_signal(context, String::from("switch"));
    let empty_class: &Signal<String> = create_signal(context, String::new());

    let demo_container_classes: &Signal<String> = create_signal(context, String::from("card"));

    let combobox_selected: &Signal<Option<Uuid>> = create_signal(context, None);
    let combobox_query: &Signal<String> = create_signal(context, String::new());

    let combobox_action_options: Vec<ComboBoxOption> = vec![
        Action {
            id: Uuid::nil(),
            name: String::from("Run"),
        }
        .to_combobox_option(),
        Action {
            id: Uuid::nil(),
            name: String::from("Sleep"),
        }
        .to_combobox_option(),
        Action {
            id: Uuid::nil(),
            name: String::from("Code"),
        }
        .to_combobox_option(),
        Action {
            id: Uuid::nil(),
            name: String::from("Play game"),
        }
        .to_combobox_option(),
    ];

    let modal_type: &Signal<ModalType> = create_signal(context, ModalType::Default);
    let click_to_close: &Signal<bool> = create_signal(context, false);

    view! {context,
        Container(title="Design System") {
            div(class="design-system") {
                div(class="design-system-header") {
                    h2(class="") { (MAIN_HEADER) }
                    p() {
                        (format!("All components are built with "))
                        a(href=SYCAMORE_GITHUB_URL, target="_NEW_TAB") {"sycamore"}
                        " and "
                        a(href=PERSEUS_GITHUB_URL, target="_NEW_TAB") {"perseus"}
                        "."
                     }
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
                                    TabButton(title=String::from("First"), index=0_u32, classes=tab_button_classes, icon=Some(PASSWORD_SVG_HTML))
                                    TabButton(title=String::from("Second"), index=1_u32, classes=tab_button_classes, icon=Some(YUBIKEY_SVG_HTML))
                                    TabButton(title=String::from("Third"), index=2_u32, classes=tab_button_classes, icon=None)
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
                            div() {
                                label() { "Hover" }
                            }
                            div() {
                                label() { "Button" }
                            }
                        }
                    }
                    div() {
                        h3() { "Combobox" }
                        div(class=demo_container_classes) {
                            ComboBox(
                                label=String::from("Example ComboBox"),
                                query=combobox_query,
                                selected=combobox_selected,
                                options=combobox_action_options,
                                classes=empty_class,
                                selected_html_input_name=String::from("actionId")
                            )
                        }
                    }
                    div() {
                        h3() { "Modal" }
                        div(class=demo_container_classes) {
                            div() {
                                button(on:click=move |_| { modal_type.set(ModalType::Default); }) { "Default" }
                                button(on:click=move |_| { modal_type.set(ModalType::SidePanelRight); }) { "Side Panel - Right"}
                                button(on:click=move |_| { modal_type.set(ModalType::SidePanelLeft); }) { "Side Panel - Left"}
                            }
                            div() {
                                label() { "Current Modal Type"}
                                input(disabled=true, type="text", value=(match modal_type.get().as_ref() {
                                    ModalType::Default => "Default".to_string(),
                                    ModalType::SidePanelRight => "Side Panel - Right".to_string(),
                                    ModalType::SidePanelLeft => "Side Panel - Left".to_string(),
                                })) {  }
                                div() {
                                    Switch(toggled=click_to_close, classes=empty_class)
                                    label() { "Click to Close"}
                                }
                            }
                            Modal(
                                html_class=empty_class,
                                button_label="Open modal",
                                modal_type=modal_type,
                                close_on_click_outside=click_to_close
                            ) { div() { "Test" } }
                        }
                    }
                    div() {
                        h3() { "Information" }
                        div(class=demo_container_classes) {

                        }
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
