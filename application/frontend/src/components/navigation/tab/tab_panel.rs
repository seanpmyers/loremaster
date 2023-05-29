use sycamore::prelude::*;

#[derive(Clone, Copy)]
pub struct TabIndex(pub u32);

#[derive(Prop)]
pub struct TabPanelProperties<'tab_panel, G: Html> {
    pub active_tab: &'tab_panel Signal<TabIndex>,
    pub children: Children<'tab_panel, G>,
}

#[component]
pub fn TabPanel<'tab_panel, G: Html>(
    context: Scope<'tab_panel>,
    properties: TabPanelProperties<'tab_panel, G>,
) -> View<G> {
    provide_context_ref(context, properties.active_tab);
    let children: View<G> = properties.children.call(context);

    view! {context,
        div() {
            (children)
        }
    }
}
