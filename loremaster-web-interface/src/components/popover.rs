use sycamore::prelude::*;

#[derive(Prop)]
pub struct PopoverProperties<'popover, G: Html> {
    pub classes: &'popover ReadSignal<String>,
    pub children: Children<'popover, G>,
}

#[component]
pub fn Popover<'popover, G: Html>(
    context: Scope<'popover>,
    PopoverProperties { children, classes }: PopoverProperties<'popover, G>,
) -> View<G> {
    let children = children.call(context);
    view! {context,
        div(class=(format!("{} popover", classes))) {
            (children)
        }
    }
}
