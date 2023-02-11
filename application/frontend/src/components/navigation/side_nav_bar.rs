use sycamore::prelude::*;

use super::{get_navigation_links, NavigationLink};

#[component(SideNavBar<G>)]
pub fn side_nav_bar() -> View<G> {
    let nav_classes: &str = "side-nav";
    let nav_ul_classes: &str = "side-nav-container";
    let nav_li_classes: &str = "side-nav-item";
    let a_class: &str = "big-nav-button side-nav-link";

    let links: Signal<Vec<NavigationLink>> = Signal::new(get_navigation_links());

    view! {
        nav(class=nav_classes) {
            ul(class=nav_ul_classes) {
                Indexed(IndexedProps{
                    iterable:links.handle(),
                    template: move |link| view! {
                        li(class=nav_li_classes) {
                            a(class=a_class, id=link.html_id, href=link.html_href) { (link.display_text) }
                        }
                    }
                })
            }
        }
    }
}
