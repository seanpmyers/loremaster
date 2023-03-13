use sycamore::prelude::*;
use web_sys::{window, Event};

use crate::components::icon::{MOON_SVG_HTML, SUN_SVG_HTML};

#[component(ThemeToggle<G>)]
pub fn theme_toggle() -> View<G> {
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
        button(
            on:click=theme_switch_handler,
            title="Switch color theme"
        ) {
            span(dangerously_set_inner_html=MOON_SVG_HTML) {}
            span(dangerously_set_inner_html=SUN_SVG_HTML) {}
        }
    }
}
