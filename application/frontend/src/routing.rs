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

pub fn switch<G: Html>(route: ReadSignal<ApplicationRoute>) -> View<G> {
    view! {
        div {
        components::navigation::Navigation()
            (match route.get().as_ref() {
                    ApplicationRoute::Index => view! {
                            components::index::Index()
                    },
                    ApplicationRoute::HelloWorld => view! {
                            components::hello_world::HelloWorld()
                    },
                    ApplicationRoute::Registration => view! {
                            components::registration::RegistrationForm()
                    },
                    ApplicationRoute::NotFound => view! {
                            "404 Not Found"
                    },
            })
        }
    }
}
