use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    #[serde(rename = "type")]
    pub field_type: String, // "text", "number", "radio", "checkbox", "select", "date", "toggle"
    pub label: String,
    pub options: Option<Vec<QuestionOption>>,
    pub mandatory: bool,
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub placeholder: Option<String>,
    pub ui: UIConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionOption {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UIConfig {
    pub order: i32,
    pub depends_on: Option<Dependency>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub question_id: String,
    pub value: Vec<String>, // supports ["yes"], or ["car", "bike"]
}

// ===== Questionnaire Config =====
#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionnaireConfig {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub insurance_type: String,
    pub version: String,
    pub questions: Vec<Question>,
    pub created_at: DateTime<Utc>,
    pub created_by: String, // admin/analyst ID
    pub is_active: bool,
}

// ===== User Response =====
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub insurance_type: String,
    pub config_version: String,
    pub responses: serde_json::Value, // { "age": 30, "smoker": "yes", ... }
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
    pub session_id: Option<String>, // for anonymous pre-login
}