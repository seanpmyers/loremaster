use sycamore::prelude::*;

#[component]
pub fn Index<G: Html>(context: Scope<'_>) -> View<G> {
    view! { context,
        div {
            h1 {"Hi"}
        }
    }
}
