use sycamore::prelude::*;

#[component(Index<G>)]
pub fn index() -> View<G> {
    view! {
        div {
            h1 {"Hi"}
        }
    }
}
