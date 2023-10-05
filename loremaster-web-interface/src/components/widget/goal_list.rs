use perseus::prelude::spawn_local_scoped;
use sycamore::prelude::*;

use crate::{
    data::entity::goal::Goal,
    utility::{
        constants::{API_BASE_URL, API_GOAL_LIST_ROUTE},
        http_service,
    },
};

#[derive(Prop)]
pub struct GoalListProperties<'a> {
    pub goals: &'a Signal<Vec<Goal>>,
}

#[component]
pub fn GoalList<'a, 'b: 'a, G: Html>(
    context: Scope<'a>,
    GoalListProperties { goals }: GoalListProperties<'b>,
) -> View<G> {
    if G::IS_BROWSER {
        spawn_local_scoped(context, async move {
            if let Some(goal_list) = get_goals().await {
                goals.set(goal_list);
            }
        });
    }
    view! { context,
        (if !goals.get().is_empty() {
            view! { context,
                ul(class=" goal_list", id="") {
                    Keyed(
                        iterable= goals,
                        view = |context, goal: Goal| view!{ context,
                            li() { (goal.name) }
                        },
                        key= |goal| goal.id
                    )
                }
            }
        } else {
            view! { context,
                "No goals available."
            }
        })
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
