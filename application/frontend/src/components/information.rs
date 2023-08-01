use sycamore::prelude::*;

#[derive(Prop)]
pub struct InformationProperties<'info, G: Html> {
    pub classes: &'info ReadSignal<String>,
    pub children: Children<'info, G>,
}

#[component]
pub fn Information<'info, G: Html>(
    context: Scope<'info>,
    InformationProperties { children, classes }: InformationProperties<'info, G>,
) -> View<G> {
    let children = children.call(context);
    view! {context,
        div(class=(format!("{} information", classes))) {
            (children)
        }
    }
}
