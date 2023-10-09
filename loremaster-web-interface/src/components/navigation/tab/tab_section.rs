use sycamore::prelude::*;

use crate::components::navigation::tab::tab_panel::TabIndex;

#[derive(Prop)]
pub struct TabSectionProperties<'tab, G: Html> {
    pub title: String,
    pub index: TabIndex,
    pub classes: &'tab ReadSignal<String>,
    pub children: Children<'tab, G>,
}

#[component]
pub fn TabSection<'tab, G: Html>(
    context: Scope<'tab>,
    TabSectionProperties {
        title,
        index,
        classes,
        children,
    }: TabSectionProperties<'tab, G>,
) -> View<G> {
    let active_tab: &Signal<TabIndex> = use_context::<Signal<TabIndex>>(context);

    let children: View<G> = children.call(context);
    view! {context,
        (match active_tab.get().as_ref() == &index {
            true => {
                view! {context, div(class=(format!("tab-section {}", classes.get())), id="") {
                    (children)
                }}
            },
            false => view! {context, ""},
        })
    }
}
