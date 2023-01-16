use sycamore::prelude::*;

use crate::{
    data::entity::frequency::Frequency,
    utility::{
        constants::{API_BASE_URL, API_FREQUENCY_LIST_ROUTE},
        http_service,
    },
};

pub struct FrequencyOptionsProperties {}

#[component(FrequencyOptions<G>)]
pub fn frequency_options(FrequencyOptionsProperties {}: FrequencyOptionsProperties) -> View<G> {
    let frequencies: Signal<Vec<Frequency>> = Signal::new(Vec::new());

    if G::IS_BROWSER {
        perseus::spawn_local(cloned!((frequencies) => async move {
            if let Some(data) = get_frequencies().await {
                frequencies.set(data);
            }
        }));
    }

    view! {
        select(name="frequency", class="form-select") {
            option(selected=true, disabled=true) { "Select the frequency" }
            Keyed( KeyedProps {
                    iterable: frequencies.handle(),
                    template: move |frequency: Frequency| {
                        view!{
                            option(value=(frequency.id)) { (frequency.unit) }
                        }
                    },
                    key: |frequency| frequency.id
                })
        }
    }
}

pub async fn get_frequencies() -> Option<Vec<Frequency>> {
    let query_response = http_service::get_endpoint(
        format!("{}/{}", API_BASE_URL, API_FREQUENCY_LIST_ROUTE).as_str(),
        None,
    )
    .await;

    match query_response {
        Some(response) => {
            let data: Vec<Frequency> = serde_json::from_str(&response).unwrap();
            Some(data)
        }
        None => None,
    }
}
