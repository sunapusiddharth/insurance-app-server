use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use bson::doc;
use crate::{
    models::questionnaire::{QuestionnaireConfig, UserResponse},
    AppState,
};

// ===== GET: Fetch active questionnaire config by type =====
pub async fn get_questionnaire_config(
    Path(insurance_type): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let collection = state.db.collection::<QuestionnaireConfig>("questionnaire_configs");

    match collection
        .find_one(
            doc! {
                "insurance_type": &insurance_type,
                "is_active": true
            },
            None,
        )
        .await
    {
        Ok(Some(config)) => (StatusCode::OK, Json(config)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "No active config found for this insurance type" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("DB Error: {}", e) })),
        )
            .into_response(),
    }
}

// ===== POST: Submit user responses =====
pub async fn submit_questionnaire(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    // Expected payload:
    // {
    //   "user_id": "usr_123",
    //   "insurance_type": "health",
    //   "config_version": "1.3",
    //   "responses": { ...answers... },
    //   "session_id": "optional"
    // }

    let user_id = payload["user_id"].as_str().unwrap_or("").to_string();
    let insurance_type = payload["insurance_type"].as_str().unwrap_or("").to_string();
    let config_version = payload["config_version"].as_str().unwrap_or("").to_string();
    let responses = payload["responses"].clone();
    let session_id = payload["session_id"].as_str().map(|s| s.to_string());

    if user_id.is_empty() || insurance_type.is_empty() || responses.is_null() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Missing required fields" })),
        )
            .into_response();
    }

    let user_response = UserResponse {
        id: None,
        user_id: user_id.clone(),
        insurance_type: insurance_type.clone(),
        config_version,
        responses,
        created_at: chrono::Utc::now(),
        is_active: true,
        session_id,
    };

    let response_collection = state.db.collection::<UserResponse>("user_questionnaire_responses");

    // Step 1: Deactivate previous active responses for this user + insurance_type
    let _ = response_collection
        .update_many(
            doc! {
                "user_id": &user_id,
                "insurance_type": &insurance_type,
                "is_active": true
            },
            doc! { "$set": { "is_active": false } },
            None,
        )
        .await;

    // Step 2: Insert new active response
    match response_collection.insert_one(user_response, None).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "message": "Responses saved. Fetching matching policies...",
                "next_step": "/api/policies/filter" // stub endpoint
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to save responses: {}", e) })),
        )
            .into_response(),
    }



    pub async fn get_last_active_responses(
    Path((user_id, insurance_type)): Path<(String, String)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let collection = state.db.collection::<UserResponse>("user_questionnaire_responses");

    match collection
        .find_one(
            doc! {
                "user_id": &user_id,
                "insurance_type": &insurance_type,
                "is_active": true
            },
            None,
        )
        .await
    {
        Ok(Some(response)) => (StatusCode::OK, Json(response)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "No saved responses found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("DB Error: {}", e) })),
        )
            .into_response(),
    }
}
}