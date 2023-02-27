use sycamore::prelude::*;

use crate::components::navigation::{side_nav_bar::SideNavBar, top_nav_bar::TopNavBar};

pub struct ContainerProperties<G: Html> {
    pub title: String,
    pub children: View<G>,
}

#[component(Container<G>)]
pub fn container(properties: ContainerProperties<G>) -> View<G> {
    view! {
        TopNavBar()
        div(class="", id="loremaster-main") {
            SideNavBar()
            (properties.children.clone())
        }
    }
}
