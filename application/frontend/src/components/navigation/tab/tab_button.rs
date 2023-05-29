use sycamore::prelude::*;
use web_sys::Event;

use crate::components::navigation::tab::tab_panel::TabIndex;

#[derive(Prop)]
pub struct TabButtonProperties<'tab> {
    pub title: String,
    pub index: TabIndex,
    pub classes: &'tab ReadSignal<String>,
}

#[component]
pub fn TabButton<'tab, G: Html>(
    context: Scope<'tab>,
    TabButtonProperties {
        title,
        index,
        classes,
    }: TabButtonProperties<'tab>,
) -> View<G> {
    let active_tab: &Signal<TabIndex> = use_context::<Signal<TabIndex>>(context);
    let clicked = move |event: Event| {
        event.prevent_default();
        create_effect(context, move || active_tab.set(index));
    };

    view! {context,
        button(class=(classes.get()), id="", on:click=clicked) {
            (title)
        }
    }
}
