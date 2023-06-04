use sycamore::prelude::*;
use uuid::Uuid;
use web_sys::Event;

pub trait ComboBoxDatum {
    fn to_combobox_option(self) -> ComboBoxOption;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComboBoxOption {
    pub id: Uuid,
    pub display_text: String,
    pub description: String,
}

#[derive(Prop)]
pub struct ComboBoxProperties<'combobox> {
    pub query: &'combobox Signal<String>,
    pub selected: &'combobox Signal<Option<Uuid>>,
    pub options: &'combobox ReadSignal<Vec<RcSignal<ComboBoxOption>>>,
    pub classes: &'combobox ReadSignal<String>,
}

#[component]
pub fn ComboBox<'combobox, G: Html>(
    context: Scope<'combobox>,
    ComboBoxProperties {
        query,
        selected,
        options,
        classes,
    }: ComboBoxProperties<'combobox>,
) -> View<G> {
    view! {context,
        div(class=classes) {
            input(type="text", bind:value=query) {}
            input(type="hidden", value=(match selected.get().as_ref() {
                Some(option) => option.to_string(),
                None => String::new(),
            }), name="actionId")
            // select() {
            //     Keyed(
            //         iterable=options,
            //         view= |context, option| view! { context,
            //             option(
            //                 // on:click=move |event: Event| {
            //                 //     event.prevent_default();
            //                 //     selected.set(Some(option.get().id))
            //                 // },
            //                 value=option.get().id
            //             ) { (option.get().display_text) }
            //         },
            //         key=|option| option.get().id
            //     )
            // }
        }
    }
}
