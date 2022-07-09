use sycamore::prelude::*;

#[component(Navigation<G>)]
pub fn navigation() -> View<G> {
    view! {
        nav(class="navbar") {
            a(href="/", class="nav-item") { "Home" }
            a(href="/hello_world", class="nav-item") { "Hello World"}
            a(href="/registration") { "Register" }
            a(href="/login") { "Login" }
        }
    }
}
