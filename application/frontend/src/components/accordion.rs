use perseus::prelude::spawn_local_scoped;
use sycamore::prelude::*;
use web_sys::Event;

use crate::components::icon::{CHEVRON_DOWN_SVG_HTML, CHEVRON_UP_SVG_HTML};

#[derive(Prop)]
pub struct AccordionProperties<'a, G: Html> {
    pub id: &'a ReadSignal<String>,
    pub children: Children<'a, G>,
}

#[component]
pub fn Accordion<'a, G: Html>(
    context: Scope<'a>,
    AccordionProperties { id, children }: AccordionProperties<'a, G>,
) -> View<G> {
    let children = children.call(context);
    view! {context,
        div(
            id=id.get(),
            class="accordion"
        ) { (children) }
    }
}

#[derive(Prop)]
pub struct AccordionItemProperties<'a, G: Html> {
    pub title: &'a Signal<String>,
    pub children: Children<'a, G>,
}

#[component]
pub fn AccordionItem<'a, G: Html>(
    context: Scope<'a>,
    AccordionItemProperties { title, children }: AccordionItemProperties<'a, G>,
) -> View<G> {
    let children = children.call(context);
    let collapsed: &Signal<bool> = create_signal(context, true);
    let item_css_classes: &Signal<&str> = create_signal(context, "accordion-collapse collapse");
    let svg: &Signal<&str> = create_signal(context, CHEVRON_UP_SVG_HTML);
    let handle_click = move |event: Event| {
        event.prevent_default();
        if G::IS_BROWSER {
            spawn_local_scoped(context, async move {
                collapsed.set(!*collapsed.get());
                match *collapsed.get() {
                    true => {
                        item_css_classes.set("accordion-collapse collapse");
                        svg.set(CHEVRON_UP_SVG_HTML);
                    }
                    false => {
                        item_css_classes.set("accordion-collapse collapse show");
                        svg.set(CHEVRON_DOWN_SVG_HTML);
                    }
                }
            });
        }
    };

    view! {context,
        div(class="accordion-item") {
            h2(class="accordion-item-title") {
                button(on:click=handle_click, class="accordion-toggle") {
                    span() { (title.get()) }
                    span(class="accordion-shevron", dangerously_set_inner_html=*svg.get()) {  }
                 }
            }
            div(class=item_css_classes) {
                div(class="accordion-item-body") {
                    (children)
                }
            }
        }

    }
}
