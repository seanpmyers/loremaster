use sycamore::prelude::*;

#[component(NavigationLinks<G>)]
pub fn navigation_links() -> View<G> {
    let app_name_class: &str = "nav-link loremaster-text fs-3";
    let a_class: &str = "nav-link loremaster-text fs-5";
    let li_class: &str = "nav-item hover-border-bottom";
    view! {
        nav(class="container-fluid border-bottom-1 rounded p-3") {
            ul(class="nav align-items-baseline") {
                li(class = (li_class)) {
                    a(href = "/", id="index-link", class = (app_name_class) ) { "Loremaster" }
                }
                li(class = (li_class)) {
                    a(href = "/about", id="about-link", class = (a_class) ) { "About" }
                }
                li(class = (li_class)) {
                    a(href = "/chronicle", id="chronicle-link", class = (a_class) ) { "Chronicle" }
                }
                li(class = (li_class)) {
                    a(href = "/registration", id="registration-link", class = (a_class) ) { "Registration" }
                }
                li(class = (li_class)) {
                    a(href = "/login", id="login-link", class = (a_class) ) { "Login" }
                }
            }
        }
    }
}
