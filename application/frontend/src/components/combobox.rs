use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use uuid::Uuid;
use web_sys::Event;

pub const COMBOBOX_OPTION_CSS_CLASSES: &str = "";
pub const COMBOBOX_QUERY_INPUT_CSS_CLASSES: &str = "";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComboBoxOption {
    pub id: Uuid,
    pub display_text: String,
    pub description: String,
}

#[derive(Prop)]
pub struct ComboBoxProperties<'combobox> {
    pub classes: &'combobox ReadSignal<String>,
    pub options: Vec<ComboBoxOption>,
    pub query: &'combobox Signal<String>,
    pub selected: &'combobox Signal<Option<Uuid>>,
    pub selected_html_input_name: String,
}

#[component]
pub fn ComboBox<'combobox, G: Html>(
    context: Scope<'combobox>,
    ComboBoxProperties {
        classes,
        selected_html_input_name,
        options,
        query,
        selected,
    }: ComboBoxProperties<'combobox>,
) -> View<G> {
    let filtered_options: &Signal<Vec<ComboBoxOption>> = create_signal(context, options.clone());

    create_effect(context, move || {
        let query: String = query.get().trim().to_lowercase();
        filtered_options.modify().clear();
        filtered_options.set(
            options
                .iter()
                .cloned()
                .filter(|option| option.display_text.to_lowercase().contains(&query))
                .collect::<Vec<_>>(),
        );
        filtered_options.modify().sort();
    });

    view! {context,
        div(class=classes) {
            input(type="text", bind:value=query) {}
            input(type="hidden", value=(match selected.get().as_ref() {
                Some(option) => option.to_string(),
                None => String::new(),
            }), name=selected_html_input_name)
            select() {
                Keyed(
                    iterable=filtered_options,
                    view= move |context, option| view! { context,
                        option(
                            class=COMBOBOX_OPTION_CSS_CLASSES,
                            value=option.id.to_string(),
                            title=option.description,
                            on:click=move |event: Event| {
                                event.prevent_default();
                                selected.set(Some(option.id))
                            }
                        ) { (option.display_text) }
                    },
                    key=|option| option.id
                )
            }
        }
    }
}
