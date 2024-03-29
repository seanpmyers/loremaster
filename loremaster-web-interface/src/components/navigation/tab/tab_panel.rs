use sycamore::prelude::*;

pub type TabIndex = u32;

#[derive(Prop)]
pub struct TabPanelProperties<'tab_panel, G: Html> {
    pub active_tab: &'tab_panel Signal<TabIndex>,
    pub classes: &'tab_panel ReadSignal<String>,
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
        div(class=(format!("tab-panel {}", properties.classes.get()))) {
            (children)
        }
    }
}
