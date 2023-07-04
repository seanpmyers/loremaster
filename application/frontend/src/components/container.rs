use sycamore::prelude::*;

use crate::components::navigation::{side_nav_bar::SideNavBar, top_nav_bar::TopNavBar};

#[derive(Prop)]
pub struct ContainerProperties<'a, G: Html> {
    pub title: &'a str,
    pub children: Children<'a, G>,
}

#[component]
pub fn Container<'a, G: Html>(
    context: Scope<'a>,
    ContainerProperties { title, children }: ContainerProperties<'a, G>,
) -> View<G> {
    let children: View<G> = children.call(context);
    view! {context,
        TopNavBar()
        SideNavBar()
        div(class="", id="loremaster-main", data-title=title) {
            (children)
        }
    }
}
