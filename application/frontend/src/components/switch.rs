use sycamore::prelude::*;
use web_sys::Event;

#[derive(Prop)]
pub struct SwitchProperties<'switch> {
    pub on: &'switch Signal<bool>,
    pub classes: &'switch ReadSignal<String>,
}

#[component]
pub fn Switch<'switch, G: Html>(
    context: Scope<'switch>,
    SwitchProperties { on, classes }: SwitchProperties<'switch>,
) -> View<G> {
    let on_click = move |_: Event| match on.get().as_ref() {
        true => on.set(false),
        false => on.set(true),
    };

    view! {context,
        label(class=classes) {
            input(
                bind:checked=on,
                type="checkbox",
                id="",
                on:click=on_click
            ) {}
            span() {}
        }
    }
}
