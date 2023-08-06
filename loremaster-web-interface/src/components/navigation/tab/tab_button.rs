use sycamore::prelude::*;
use web_sys::Event;

use crate::components::{icon::SvgIcon, navigation::tab::tab_panel::TabIndex};

#[derive(Prop)]
pub struct TabButtonProperties<'tab> {
    pub title: String,
    pub index: TabIndex,
    pub classes: &'tab Signal<String>,
    pub icon: Option<SvgIcon>,
}

#[component]
pub fn TabButton<'tab, G: Html>(
    context: Scope<'tab>,
    TabButtonProperties {
        title,
        index,
        classes,
        icon,
    }: TabButtonProperties<'tab>,
) -> View<G> {
    let active_tab: &Signal<TabIndex> = use_context::<Signal<TabIndex>>(context);
    let clicked = move |event: Event| {
        event.prevent_default();
        create_effect(context, move || active_tab.set(index));
    };

    view! {context,
        button(class=(match active_tab.get().as_ref() == &index {
            true => format!("tab-button active-tab {}", classes.get()),
            false => format!("tab-button {}", classes),
        }), id="", on:click=clicked) {
            (match icon {
                Some(icon) => view!{context, span(dangerously_set_inner_html=icon) {} },
                None => view!{context, },
            })
            (title)
        }
    }
}
