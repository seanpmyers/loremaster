use crate::components::home::Home;
use crate::components::registration::Registration;
use crate::components::login::Login;
use crate::components::chronicle::Chronicle;

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
                        li { class: "nav-item", Link { class: "nav-link", to: "/chronicle",  "Chronicle" } }
                        li { class: "nav-item", Link { class: "nav-link", to: "/registration", li { "Registration" } } } 
                            li { class: "nav-item", Link { class: "nav-link", to: "/login", li { "Login" } } } 
                        }
                    }
                    
                }
            }
            div {
                Route { to: "/", Home {} }
                Route { to: "/chronicle", Chronicle {} }
                Route { to: "/registration", Registration {} }
                Route { to: "/login", Login {} }
                Route { to: "", "Err 404 Route Not Found" }
            }
            
        }
        
    })
}
