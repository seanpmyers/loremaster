use sycamore::prelude::*;

#[component(NavigationLinks<G>)]
pub fn navigation_links() -> View<G> {
    let a_class: &str = "nav-link text-black fs-5";
    let li_class: &str = "nav-item hover-border-bottom";
    view! {
        nav(class="container-fluid border-bottom-1 rounded p-3") {
            ul(class="nav") {
                li(class = (li_class)) {
                    a(href = "/", id="home-link", class = (a_class) ) { "Home" }
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
