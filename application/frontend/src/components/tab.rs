use sycamore::prelude::*;
use web_sys::Event;

pub struct TabProperties<G: Html> {
    pub title: String,
    pub id: String,
    pub classes: String,
    pub parent_id: String,
    pub children: View<G>,
}

#[component]
pub fn Tab<G: Html>(
    context: Scope,
    TabProperties {
        title,
        id,
        classes,
        parent_id,
        children,
    }: TabProperties<G>,
) -> View<G> {
    let clicked = |context: Scope, event: Event| {
        event.prevent_default();
        // classes.set(format!("{classes} active"));
    };
    view! {context,
        div(class=(classes), id=(id)) {
            (children.clone())
        }
    }
}
