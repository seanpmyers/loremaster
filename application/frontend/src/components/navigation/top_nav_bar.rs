use sycamore::prelude::*;

use crate::components::widget::date_time::DateTime;
use crate::components::{
    icon::{GLOBE_SVG_HTML, SEARCH_SVG_HTML},
    widget::theme_toggle::ThemeToggle,
};

use super::{get_home_link, NavigationLink};

#[component]
pub fn TopNavBar<G: Html>(context: Scope) -> View<G> {
    let nav_classes: &str = "top-nav";

    let home_link: NavigationLink = get_home_link();

    view! {context,
        nav(class=nav_classes) {
            div(class="loremaster-banner") {
                a(href=home_link.html_href, id=home_link.html_id, class="loremaster-banner-link") { (home_link.display_text)}
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
        }
    }
}
