use sycamore::prelude::*;

#[component(Navigation<G>)]
pub fn navigation() -> View<G> {
    let list_item_class: &str = "acrylic-link navbar-item rounded";
    let link_class: &str = "nav-link";
    view! {
        nav(class="navbar navbar-expand-lg") {
            div(class="container-fluid") {
                a(class="navbar-brand", href="/") {
                    img(
                        class="d-inline-block align-text-top",
                        src="logo.svg",
                        alt="Loremaster's sheep logo",
                        width="30",
                        height="24"
                    ) { "Loremaster" }
                }
                div(class="navbar-collapse") {
                    ul(class="navbar-nav") {
                        li(class=list_item_class) { a(class=link_class, href="/") { "Home" } }
                        li(class=list_item_class) { a(class=link_class, href="/hello_world", ) { "Hello World"}  }
                        li(class=list_item_class) { a(class=link_class, href="/registration") { "Register" } }
                        li(class=list_item_class) { a(class=link_class, href="/login") { "Login" } }
                    }
                }
            }

        }
    }
}
