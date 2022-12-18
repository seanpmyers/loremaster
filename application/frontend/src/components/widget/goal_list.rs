use sycamore::prelude::*;

use crate::{
    data::entity::goal::Goal,
    utility::{
        constants::{API_BASE_URL, API_GOAL_LIST_ROUTE},
        http_service,
    },
};

pub struct GoalListProperties {
    pub goals: Signal<Vec<Goal>>,
}

#[component(GoalList<G>)]
pub fn goal_list(GoalListProperties { goals }: GoalListProperties) -> View<G> {
    if G::IS_BROWSER {
        perseus::spawn_local(cloned!((goals) => async move {
            if let Some(goal_list) = get_goals().await {
                goals.set(goal_list);
            }
        }));
    }
    view! {
        div(class="d-flex flex-row goal_list", id="") {
            Keyed( KeyedProps {
                    iterable: goals.handle(),
                    template: move |goal: Goal| {
                        view!{
                            div() { (goal.name) }
                        }
                    },
                    key: |goal| goal.id
                })
        }
    }
}

pub async fn get_goals() -> Option<Vec<Goal>> {
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
