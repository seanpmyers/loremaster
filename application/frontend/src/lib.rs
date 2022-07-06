use components::{hello_world::HelloWorld, registration::RegistrationForm};
use sycamore::prelude::*;

pub mod components;
pub mod data;
pub mod utility;

#[component]
pub fn App<G: Html>(context: Scope) -> View<G> {
    view! { context,
        HelloWorld {}
        RegistrationForm {}
    }
}
