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
    #[to("/login")]
    Login,
    #[not_found]
    NotFound,
}

pub fn switch<G: Html>(route: ReadSignal<ApplicationRoute>) -> View<G> {
    view! {
        div(class="min-vh-100") {
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
                ApplicationRoute::Login => view! {
                    components::hello_world::HelloWorld()
                },
                ApplicationRoute::NotFound => view! {
                        "404 Not Found"
                },
            })
        }
    }
}
