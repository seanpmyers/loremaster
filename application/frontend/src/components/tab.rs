use sycamore::prelude::*;
use web_sys::Event;

pub struct TabProperties<G: Html> {
    pub title: String,
    pub id: String,
    pub classes: String,
    pub parent_id: String,
    pub children: View<G>,
}

#[component(Tab<G>)]
pub fn tab(
    TabProperties {
        title,
        id,
        classes,
        parent_id,
        children,
    }: TabProperties<G>,
) -> View<G> {
    let clicked = cloned!((classes, parent_id) => move |event: Event| {
            event.prevent_default();
            // classes.set(format!("{classes} active"));
    });
    view! {
        div(class=(classes), id=(id)) {
            (children.clone())
        }
    }
}
