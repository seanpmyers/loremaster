use sycamore::prelude::*;
use web_sys::Event;

#[derive(Prop)]
pub struct SwitchProperties<'switch> {
    pub toggled: &'switch Signal<bool>,
    pub classes: &'switch ReadSignal<String>,
}

#[component]
pub fn Switch<'switch, G: Html>(
    context: Scope<'switch>,
    SwitchProperties { toggled, classes }: SwitchProperties<'switch>,
) -> View<G> {
    let on_click = move |_: Event| match toggled.get().as_ref() {
        true => toggled.set(false),
        false => toggled.set(true),
    };

    view! {context,
        label(class=classes) {
            input(
                bind:checked=toggled,
                type="checkbox",
                id="",
                on:click=on_click
            ) {}
            span() {}
        }
    }
}
