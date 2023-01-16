use sycamore::prelude::*;

use crate::{
    data::entity::goal::Goal,
    utility::{
        constants::{API_BASE_URL, API_GOAL_LIST_ROUTE},
        http_service,
    },
};

pub struct ValueListProperties {
    pub values: Signal<Vec<Value>>,
}

#[component(ValueList<G>)]
pub fn value_list(ValueListProperties { values }: ValueListProperties) -> View<G> {
    if G::IS_BROWSER {
        perseus::spawn_local(cloned!((values) => async move {
            if let Some(value_list) = get_values().await {
                values.set(value_list);
            }
        }));
    }
    view! {
        (if goals.get().len() > 0 {
            view! {
                ul(class="value_list", id="") {
                    Keyed( KeyedProps {
                            iterable: values.handle(),
                            template: move |value: Value| {
                                view!{
                                    li() { (value.name) }
                                }
                            },
                            key: |value| value.id
                        })
                }
            }
        } else {
            view! {
                div() {
                    "No values available."
                }
            }
        })
    }
}

pub async fn get_values() -> Option<Vec<Value>> {
    let query_response = http_service::get_endpoint(
        format!("{}/{}", API_BASE_URL, API_GOAL_LIST_ROUTE).as_str(),
        None,
    )
    .await;
    match query_response {
        Some(response) => {
            let goal_list_data: Vec<Goal> = serde_json::from_str(&response).unwrap();
            Some(goal_list_data)
        }
        None => None,
    }
}
