use sycamore::prelude::*;

use crate::components::navigation::DESIGN_SYSTEM_LINK;

use super::{get_navigation_links, NavigationLink};

#[component]
pub fn SideNavBar<G: Html>(context: Scope) -> View<G> {
    let nav_classes: &str = "side-nav";
    let nav_ul_classes: &str = "side-nav-container";
    let nav_li_classes: &str = "side-nav-item";
    let a_class: &str = "big-nav-button side-nav-link";
    let nav_button_icon_class: &str = "big-nav-button-icon";
    let nav_button_text_class: &str = "big-nav-button-text";

    let links: &Signal<Vec<NavigationLink>> = create_signal(context, get_navigation_links());

    view! {context,
        nav(class=nav_classes) {
            ul(class=nav_ul_classes) {
                Indexed(
                    iterable= links,
                    view= move |context, link| view! { context,
                        li(class=nav_li_classes) {
                            a(
                                class=a_class,
                                id=link.html_id,
                                href=link.html_href
                            ) {
                                span(class=nav_button_icon_class,dangerously_set_inner_html=link.svg_html) {}
                                span(class=nav_button_text_class) {(link.display_text)}

                            }
                        }
                    }
                )
                li(class=nav_li_classes) {
                    a(
                        class=a_class,
                        id=DESIGN_SYSTEM_LINK.html_id,
                        href=DESIGN_SYSTEM_LINK.html_href
                    ) {
                        span(class=nav_button_icon_class,dangerously_set_inner_html=DESIGN_SYSTEM_LINK.svg_html) {}
                        span(class=nav_button_text_class) {(DESIGN_SYSTEM_LINK.display_text)}

                    }
                }
            }
        }
    }
}
