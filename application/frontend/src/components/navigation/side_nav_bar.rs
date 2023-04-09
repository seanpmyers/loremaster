use sycamore::prelude::*;

use crate::components::{
    icon::{GLOBE_SVG_HTML, SEARCH_SVG_HTML},
    widget::{date_time::DateTime, theme_toggle::ThemeToggle},
};

use super::{get_navigation_links, NavigationLink};

#[component]
pub fn SideNavBar<G: Html>(context: Scope) -> View<G> {
    let nav_classes: &str = "side-nav";
    let nav_ul_classes: &str = "side-nav-container";
    let nav_li_classes: &str = "side-nav-item";
    let a_class: &str = "big-nav-button side-nav-link";
    let quick_function_classes: &str = "quick-functions";

    let links: &Signal<Vec<NavigationLink>> = create_signal(context, get_navigation_links());

    view! {context,
        nav(class=nav_classes) {
            DateTime()
            ul(class=nav_ul_classes) {
                Indexed(IndexedProps{
                    iterable: links,
                    view: |context, link| view! { context,
                        li(class=nav_li_classes) {
                            a(
                                class=a_class,
                                id=link.html_id,
                                href=link.html_href
                            ) {
                                span(class="big-nav-button-icon",dangerously_set_inner_html=link.svg_html) {}
                                span(class="big-nav-button-text") {(link.display_text)}

                            }
                        }
                    }
                })
            }
            section(class=quick_function_classes) {
                div() {
                    ThemeToggle()
                }
                div() {
                    button(
                        dangerously_set_inner_html=SEARCH_SVG_HTML,
                        title="Search"
                    ) {}
                }
                div() {
                    button(
                        dangerously_set_inner_html=GLOBE_SVG_HTML,
                        title="Glossary"
                    ) {}
                }
            }
        }
    }
}
