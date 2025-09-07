use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use bson::doc;
use crate::{
    models::policy::Policy,
    utils::policy_filter::filter_and_rank_policies,
    AppState,
};

pub async fn filter_policies(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let user_id = payload["user_id"].as_str().unwrap_or("").to_string();
    let insurance_type = payload["insurance_type"].as_str().unwrap_or("").to_string();
    let responses = &payload["responses"];

    if user_id.is_empty() || insurance_type.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Missing user_id or insurance_type" })),
        )
            .into_response();
    }

    let policy_collection = state.db.collection::<Policy>("policies");

    let policies = match policy_collection
        .find(
            doc! {
                "insurance_type": &insurance_type,
                "is_active": true
            },
            None,
        )
        .await
    {
        Ok(cursor) => cursor.try_collect().await.unwrap_or_default(),
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to fetch policies" })),
        )
            .into_response(),
    };

    let filtered_policies = filter_and_rank_policies(policies, responses);

    (
        StatusCode::OK,
        Json(json!({
            "count": filtered_policies.len(),
            "policies": filtered_policies
        })),
    )
        .into_response()
}