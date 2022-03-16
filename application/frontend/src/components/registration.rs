use sycamore::prelude::*;

#[component(RegistrationForm<G>)]
pub fn registration_form() -> View<G> {
    view! {
        div {
            h1 {
                "Register"
            }
            form {
                div {
                    label {"Email Address"}
                    input(placeholder="you@loremaster.xyz", type="text")
                }
            }
        }
    }
}
