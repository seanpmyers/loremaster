use perseus::reactor::Reactor;
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::navigation::{side_nav_bar::SideNavBar, top_nav_bar::TopNavBar},
    global_state::ApplicationStateRx,
};

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
    let user_authentication =
        Reactor::<G>::from_cx(context).get_global_state::<ApplicationStateRx>(context);
    let children: View<G> = children.call(context);
    if G::IS_BROWSER {
        spawn_local_scoped(context, async {
            user_authentication.authentication.detect_state().await;
        });
    }
    view! {context,
        div(class="glass container") {
            TopNavBar()
            SideNavBar()
            div(class="", id="loremaster-main", data-title=title) {
                (children)
            }
        }
    }
}
