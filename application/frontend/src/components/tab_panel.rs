use std::collections::HashMap;

use perseus::prelude::spawn_local_scoped;
use sycamore::prelude::*;
use web_sys::Event;

#[derive(Prop)]
pub struct TabPanelProperties<'a, G: Html> {
    pub tabs: &'a ReadSignal<HashMap<String, Children<'a, G>>>,
}

#[component]
pub fn TabPanel<'a, G: Html>(
    context: Scope<'a>,
    TabPanelProperties { tabs }: TabPanelProperties<'a, G>,
) -> View<G> {
    let result: HashMap<String, View<G>> = HashMap::new();

    view! {context,

    }
}
