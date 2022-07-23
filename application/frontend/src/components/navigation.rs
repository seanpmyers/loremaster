use sycamore::prelude::*;

#[component(NavigationLinks<G>)]
pub fn navigation_links() -> View<G> {
    view! {
        li(class = "m-3 p-1") {
            a(href = "/", id="home-link", class = "px-2") { "Home" }
        }
        li(class = "m-3 p-1") {
            a(href = "/about", id="about-link", class = "px-2") { "About" }
        }
        li(class = "m-3 p-1") {
            a(href = "/login", id="login-link", class = "px-2") { "Login" }
        }
    }
}
