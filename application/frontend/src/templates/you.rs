use perseus::{RenderFnResultWithCause, Template};
use sycamore::prelude::{cloned, view, Html, Signal, SsrNode, View};

use crate::{
    components::container::{Container, ContainerProperties},
    utility::{constants::API_USER_DATA_URL, http_service},
};

#[perseus::make_rx(YouPageStateRx)]
pub struct YouPageState {
    pub email_address: String,
}

#[perseus::template_rx]
pub fn you_page(state: YouPageStateRx) -> View<G> {
    let email_address: Signal<String> = state.email_address;
    let email_address_input: Signal<String> = email_address.clone();
    if G::IS_BROWSER {
        perseus::spawn_local(cloned!((email_address) => async move {

            let query_response = http_service::get_endpoint(API_USER_DATA_URL, None).await;
            match query_response {
                Some(response) => todo!(),
                None => todo!(),
            }

        }));
    }
    view! {
        Container(ContainerProperties{title: String::from("You"), children: view!{
            div(class="d-flex flex-column flex-grow-1 p-4 align-items-center") {
                div() {
                    h1(class="display-3") { "You" }
                    p() { "This is a page dedicated to you." }
                }
                div(class="mb-3") {
                    label(class="form-label") {"Email Address"}
                    input(class="form-control") {}
                }
            }
        }})
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("you")
        .build_state_fn(get_build_state)
        .template(you_page)
        .head(head)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<YouPageState> {
    Ok(YouPageState {
        email_address: String::new(),
    })
}

#[perseus::head]
pub fn head(_props: YouPageState) -> View<SsrNode> {
    view! {
        title { "You | Loremaster" }
    }
}
