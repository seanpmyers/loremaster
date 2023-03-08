use sycamore::prelude::*;

use super::{get_home_link, NavigationLink};

#[component(TopNavBar<G>)]
pub fn top_nav_bar() -> View<G> {
    let nav_classes: &str = "top-nav";

    let home_link: NavigationLink = get_home_link();

    view! {
        nav(class=nav_classes) {
            div(class="loremaster-banner") {
                a(href=home_link.html_href, id=home_link.html_id, class="loremaster-banner-link") { (home_link.display_text)}
            }
        }
    }
}
