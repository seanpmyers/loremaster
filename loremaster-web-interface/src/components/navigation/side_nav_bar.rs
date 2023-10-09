use perseus::reactor::Reactor;
use sycamore::prelude::*;

use crate::{components::navigation::DESIGN_SYSTEM_LINK, global_state::ApplicationStateRx};

use super::{get_navigation_links, NavigationLink};

const NAV_CLASSES: &str = "side-nav";
const NAV_UL_CLASSES: &str = "side-nav-container";
const NAV_LI_CLASSES: &str = "side-nav-item";
const A_CLASS: &str = "big-nav-button side-nav-link";
const NAV_BUTTON_ICON_CLASS: &str = "big-nav-button-icon";
const NAV_BUTTON_TEXT_CLASS: &str = "big-nav-button-text";

#[component]
pub fn SideNavBar<G: Html>(context: Scope) -> View<G> {
    let user_authentication =
        Reactor::<G>::from_cx(context).get_global_state::<ApplicationStateRx>(context);
    let links: &Signal<Vec<NavigationLink>> = create_signal(context, get_navigation_links());

    view! {context,
        nav(class=NAV_CLASSES) {
            ul(class=NAV_UL_CLASSES) {
                Indexed(
                    iterable=links,
                    view= move |context, link|
                    {
                        if link.html_href.eq("/you/") {
                            return view! { context,
                                li(class=NAV_LI_CLASSES) {
                                    a(
                                        class=A_CLASS,
                                        id=link.html_id,
                                        href=link.html_href
                                    ) {
                                        span(class=NAV_BUTTON_ICON_CLASS,dangerously_set_inner_html=link.svg_html) {}
                                        span(class=NAV_BUTTON_TEXT_CLASS) {(user_authentication.authentication.user_alias.get())}

                                    }
                                }
                            };
                        }
                    view! { context,
                        li(class=NAV_LI_CLASSES) {
                            a(
                                class=A_CLASS,
                                id=link.html_id,
                                href=link.html_href
                            ) {
                                span(class=NAV_BUTTON_ICON_CLASS,dangerously_set_inner_html=link.svg_html) {}
                                span(class=NAV_BUTTON_TEXT_CLASS) {(link.display_text)}

                            }
                        }
                    }}
                )
                DesignSystemLink()
            }
        }
    }
}

#[component]
pub fn DesignSystemLink<G: Html>(context: Scope) -> View<G> {
    view! { context,
       li(class=NAV_LI_CLASSES) {
           a(
               class=A_CLASS,
               id=DESIGN_SYSTEM_LINK.html_id,
               href=DESIGN_SYSTEM_LINK.html_href
           ) {
               span(class=NAV_BUTTON_ICON_CLASS,dangerously_set_inner_html=DESIGN_SYSTEM_LINK.svg_html) {}
               span(class=NAV_BUTTON_TEXT_CLASS) {(DESIGN_SYSTEM_LINK.display_text)}

           }
       }
    }
}
