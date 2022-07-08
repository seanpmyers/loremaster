use routing::{switch, ApplicationRoute};
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, StaticRouter};

pub mod components;
pub mod routing;
pub mod utility;

#[component]
pub fn App<G: Html>(context: Scope, possible_path: Option<String>) -> View<G> {
    match possible_path {
        Some(path) => {
            let route = ApplicationRoute::match_path(&ApplicationRoute::Index, &path);
            view! { context,
                StaticRouter {
                    view: switch,
                    route: route,
                }
            }
        }
        None => view! { context,
            Router {
                view: switch,
                integration: HistoryIntegration::new(),
            }
        },
    }
}
