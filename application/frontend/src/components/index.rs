use sycamore::prelude::*;

#[component(Index<G>)]
pub fn index() -> View<G> {
    let main_content_class: &str =
        "col-10 bg-white border-1 rounded-top rounded-end border-top border-end overflow-auto shadow p-3";
    let main_content_style: &str = "height: calc(100vh - 56px);";
    view! {
        div(class="container-fluid") {
            div(class="row") {
                div(class=main_content_class, style=main_content_style) {
                    h1 {"Hi"}
                }
                div(class="col-2") {
                    h1 {"Sean"}
                }
            }
        }
    }
}
