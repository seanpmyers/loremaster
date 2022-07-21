use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use sycamore::prelude::{view, View};

#[perseus::make_rx(LoginPageStateRx)]
pub struct LoginPageState {
    pub greeting: String,
}

#[perseus::template_rx]
pub fn login_page(state: LoginPageStateRx) -> View<G> {
    view! {
        p { (state.greeting.get()) }
        a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("login")
        .build_state_fn(get_build_state)
        .template(login_page)
        .head(head)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<LoginPageState> {
    Ok(LoginPageState {
        greeting: "Hello World!".to_string(),
    })
}

#[perseus::head]
pub fn head(_props: LoginPageState) -> View<SsrNode> {
    view! {
        title { "Login - Loremaster " }
    }
}
