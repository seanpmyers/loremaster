use sycamore::prelude::*;

use crate::components::navigation::NavigationLinks;

pub struct ContainerProperties<G: Html> {
    pub title: String,
    pub children: View<G>,
}

#[component(Container<G>)]
pub fn container(properties: ContainerProperties<G>) -> View<G> {
    view! {
        NavigationLinks()
        div(class="", id="loremaster-main") {
            (properties.children.clone())
        }
    }
}
