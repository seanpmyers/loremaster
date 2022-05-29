use crate::components::home::Home;
use crate::components::registration::Registration;

use dioxus::router::{Link, Route, Router};
use dioxus::{events::FormEvent, prelude::*};

pub fn Navigation(context: Scope) -> Element {
    context.render(rsx! {
        Router {
            nav { class: "navbar border-bottom navbar-expand-lg",
                div { class: "container-fluid",
                    Link { class: "navbar-brand", to: "/",  "Loremaster" }
                    div { class: "me-2 navbar-collapse",
                        ul { class: "navbar-nav",
                            li { class: "nav-item", Link { class: "nav-link", to: "/",  "Home" } }
                            li { class: "nav-item", Link { class: "nav-link", to: "/registration", li { "Registration" } } } 
                        }
                    }
                    
                }
            }
            div {
                Route { to: "/", Home {} }
                Route { to: "/registration", Registration {} }
                Route { to: "", "Err 404 Route Not Found" }
            }
            
        }
        
    })
}
