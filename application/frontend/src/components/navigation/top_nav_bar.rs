use perseus::reactor::Reactor;
use sycamore::prelude::*;

use crate::components::widget::date_time::DateTime;
use crate::components::{
    icon::{GIT_MERGE_SVG_HTML, GLOBE_SVG_HTML, SEARCH_SVG_HTML, USER_CIRCLE_SVH_HTML},
    widget::theme_toggle::ThemeToggle,
};
use crate::global_state::ApplicationStateRx;

use super::{get_home_link, NavigationLink};

#[component]
pub fn TopNavBar<G: Html>(context: Scope) -> View<G> {
    let user_authentication =
        Reactor::<G>::from_cx(context).get_global_state::<ApplicationStateRx>(context);

    let nav_classes: &str = "top-nav";

    let home_link: NavigationLink = get_home_link();

    view! {context,
        nav(class=nav_classes) {
            div(class="loremaster-banner") {
                a(href=home_link.html_href, id=home_link.html_id, class="loremaster-banner-link") { (home_link.display_text)}
            }
            div(id="top-nav-version") {
                div(class="glass-blur") {
                    span(dangerously_set_inner_html=GIT_MERGE_SVG_HTML) {}
                    span() { "2023.7.7"}
                }
            }
            ThemeToggle()
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
            DateTime()
            div(id="top-nav-user-icon") {
                button(
                    title="You"
                ) {
                    span(dangerously_set_inner_html=USER_CIRCLE_SVH_HTML) { }
                    span() { (user_authentication.authentication.user_alias.get()) }
                }

            }
        }
    }
}
