use sycamore::prelude::*;

#[component(NavigationLinks<G>)]
pub fn navigation_links() -> View<G> {
    view! {
        nav(class="navbar position-fixed") {
            div(class="nav") {
                li(class = "m-3 p-1") {
                    a(href = "/", id="home-link", class = "px-2") { "Home" }
                }
                li(class = "m-3 p-1") {
                    a(href = "/about", id="about-link", class = "px-2") { "About" }
                }
                li(class = "m-3 p-1") {
                    a(href = "/chronicle", id="chronicle-link", class = "px-2") { "Chronicle" }
                }
                li(class = "m-3 p-1") {
                    a(href = "/registration", id="registration-link", class = "px-2") { "Registration" }
                }
                li(class = "m-3 p-1") {
                    a(href = "/login", id="login-link", class = "px-2") { "Login" }
                }
            }
        }
    }
}
