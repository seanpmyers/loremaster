use sycamore::prelude::*;
use sycamore_router::Route;

use crate::components;

#[derive(Route, Clone)]
pub enum ApplicationRoute {
    #[to("/")]
    Index,
    #[to("/registration")]
    Registration,
    #[to("/hello_world")]
    HelloWorld,
    #[not_found]
    NotFound,
}

pub fn switch<'a, G: Html>(context: Scope<'a>, route: &'a ReadSignal<ApplicationRoute>) -> View<G> {
    view! { context,
        div {
            components::navigation_bar::NavigationBar {}
            (match route.get().as_ref() {
                    ApplicationRoute::Index => view! { context,
                            components::index::Index()
                    },
                    ApplicationRoute::HelloWorld => view! { context,
                            components::hello_world::HelloWorld()
                    },
                    ApplicationRoute::Registration => view! { context,
                            components::registration::RegistrationForm()
                    },
                    ApplicationRoute::NotFound => view! { context,
                            "404 Not Found"
                    },
            })
        }
    }
}
