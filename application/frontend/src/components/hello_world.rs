use sycamore::prelude::*;

#[component(HelloWorld<G>)]
pub fn hello_world() -> View<G> {
    let name = Signal::new(String::new());
    let name2 = name.clone();

    view! {
        div {
            h1 {
                "Hello "
                (if *create_selector(cloned!((name) => move || !name.get().is_empty())).get() {
                    cloned!((name) => view! {
                        span { (name.get()) }
                    })
                } else {
                    view! { span { "World" } }
                })
                "!"
            }

            input(bind:value=name2)
        }
    }
}
