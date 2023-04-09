use sycamore::prelude::*;

use crate::components::navigation::{side_nav_bar::SideNavBar, top_nav_bar::TopNavBar};

pub struct ContainerProperties<G: Html> {
    pub title: String,
    pub children: View<G>,
}

#[component]
pub fn Container<G: Html>(context: Scope, properties: ContainerProperties<G>) -> View<G> {
    view! {context,
        TopNavBar()
        div(class="", id="loremaster-main") {
            SideNavBar()
            (properties.children.clone())
        }
    }
}
