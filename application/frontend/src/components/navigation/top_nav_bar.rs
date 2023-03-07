use sycamore::prelude::*;
use web_sys::{window, Event};

use super::{get_home_link, NavigationLink};

#[component(TopNavBar<G>)]
pub fn top_nav_bar() -> View<G> {
    let nav_classes: &str = "top-nav";

    let home_link: NavigationLink = get_home_link();

    let theme_switch_handler = move |event: Event| {
        event.prevent_default();
        let root = window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector(":root")
            .unwrap();
        match root {
            Some(element) => element.class_list().toggle("light").unwrap(),
            None => false,
        };
    };

    view! {
        nav(class=nav_classes) {
            div(class="loremaster-banner") {
                a(href=home_link.html_href, id=home_link.html_id, class="loremaster-banner-link") { (home_link.display_text)}
            }
            div() {
                button(on:click=theme_switch_handler) {
                    "Toggle Theme"
                }
            }
        }
    }
}
