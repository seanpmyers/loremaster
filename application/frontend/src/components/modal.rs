use sycamore::builder::prelude::*;
use sycamore::prelude::*;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlDialogElement};

#[derive(Prop)]
pub struct ModalProperties<'modal, G: Html> {
    pub html_class: &'modal ReadSignal<String>,
    pub children: Children<'modal, G>,
    pub button_label: &'modal str,
}

#[component]
pub fn Modal<'modal, G: Html>(
    context: Scope<'modal>,
    ModalProperties {
        children,
        html_class,
        button_label,
    }: ModalProperties<'modal, G>,
) -> View<G> {
    let children = children.call(context);

    let dialog_id = Uuid::new_v4();
    let dialog = dialog::<G>()
        .id(dialog_id.to_string())
        .class("modal")
        .c(fragment([
            div().c(children).view(context),
            button()
                .on("click", move |_: Event| {
                    let dialog_node = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id(&dialog_id.to_string());
                    if let Some(element) = dialog_node {
                        element.unchecked_into::<HtmlDialogElement>().close();
                    };
                })
                .class("modal-close")
                .t("Ok")
                .view(context),
        ]))
        .view(context);

    let button = button::<G>()
        .on("click", move |_: Event| {
            let dialog_node = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id(&dialog_id.to_string());
            if let Some(element) = dialog_node {
                element.unchecked_into::<HtmlDialogElement>().show_modal();
            };
        })
        .t(button_label)
        .view(context);

    fragment([button, dialog])
}
