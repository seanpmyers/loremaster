use routing::{switch, ApplicationRoute};
use sycamore::prelude::*;
use sycamore_router::{
    HistoryIntegration, Route, Router, RouterProps, StaticRouter, StaticRouterProps,
};

mod components;
mod routing;
mod utility;

#[component(App<G>)]
pub fn app(possible_path: Option<String>) -> View<G> {
    match possible_path {
        Some(path) => {
            let route = ApplicationRoute::match_path(&path);
            view! {
                StaticRouter(StaticRouterProps::new(route, |route: ApplicationRoute| switch(Signal::new(route).handle())))
            }
        }
        None => view! {
            Router(RouterProps::new(HistoryIntegration::new(), switch))
        },
    }
}
