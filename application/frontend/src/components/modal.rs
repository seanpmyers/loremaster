use sycamore::prelude::*;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlDialogElement};

use crate::components::icon::CLOSE_X_SVG_HTML;

#[derive(PartialEq, Clone)]
pub enum ModalType {
    Default,
    SidePanelRight,
    SidePanelLeft,
}

#[derive(Prop)]
pub struct ModalProperties<'modal, G: Html> {
    pub html_class: &'modal ReadSignal<String>,
    pub children: Children<'modal, G>,
    pub button_label: &'static str,
    pub modal_type: &'modal ReadSignal<ModalType>,
}

#[component]
pub fn Modal<'modal, G: Html>(
    context: Scope<'modal>,
    ModalProperties {
        children,
        html_class,
        button_label,
        modal_type,
    }: ModalProperties<'modal, G>,
) -> View<G> {
    let children = children.call(context);
    let dialog_id = Uuid::new_v4();

    let open_click_handler = move |_: Event| {
        open_dialog(&dialog_id.to_string());
    };

    let close_click_handler = move |_: Event| {
        close_dialog(&dialog_id.to_string());
    };

    view! {context,
        button(on:click=open_click_handler, class=html_class) { (button_label) }
        dialog(
            id=dialog_id.to_string(),
            class=(match modal_type.get().as_ref() {
                ModalType::Default => "modal",
                ModalType::SidePanelRight => "modal-side-panel-right",
                ModalType::SidePanelLeft => "modal-side-panel-left",
            })
        ) {
            button(title="close",on:click=close_click_handler, class="modal-close", dangerously_set_inner_html=CLOSE_X_SVG_HTML) { }
            div() {
                (children)
            }
        }
    }
}

//TODO: fix error handling
pub fn close_dialog(dialog_html_id: &str) {
    let dialog_node = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(dialog_html_id);
    if let Some(element) = dialog_node {
        element.unchecked_into::<HtmlDialogElement>().close();
    };
}

//TODO: fix error handling
pub fn open_dialog(dialog_html_id: &str) {
    let dialog_node = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(dialog_html_id);
    if let Some(element) = dialog_node {
        element
            .unchecked_into::<HtmlDialogElement>()
            .show_modal()
            .unwrap();
    };
}
