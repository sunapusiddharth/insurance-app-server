use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use chrono::DateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Policy {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub policy_name: String,
    pub insurance_type: String, // "health", "term", etc.
    pub company_id: ObjectId,   // reference to insurance company
    pub premium: f64,           // monthly/yearly premium
    pub coverage_amount: f64,   // sum insured
    pub eligibility: serde_json::Value, // { "min_age": 18, "max_age": 65, "smoker": ["yes", "no"], ... }
    pub benefits: Vec<String>,
    pub exclusions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}