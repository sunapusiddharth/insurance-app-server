use mongodb::{Client, Collection};
use serde_json::json;
use std::env;
use chrono::Utc;

#[derive(serde::Serialize)]
struct Policy {
    policy_name: String,
    insurance_type: String,
    company_id: bson::oid::ObjectId,
    premium: f64,
    coverage_amount: f64,
    eligibility: serde_json::Value,
    benefits: Vec<String>,
    exclusions: Vec<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    is_active: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mongo_uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let client = Client::with_uri_str(mongo_uri).await?;
    let db = client.database("insurance_app");
    let collection: Collection<Policy> = db.collection("policies");

    // Create dummy company ID
    let company_id = bson::oid::ObjectId::new();

    let policies = vec![
        Policy {
            policy_name: "HealthPlus Gold".to_string(),
            insurance_type: "health".to_string(),
            company_id,
            premium: 899.0,
            coverage_amount: 1000000.0,
            eligibility: json!({
                "age": {"min": 18, "max": 65},
                "smoker": ["no"],
                "pre_existing": ["none"]
            }),
            benefits: vec!["Cashless hospitalization".to_string(), "Annual health checkup".to_string()],
            exclusions: vec!["Pre-existing conditions in first 2 years".to_string()],
            created_at: Utc::now(),
            is_active: true,
        },
        Policy {
            policy_name: "HealthPlus Silver".to_string(),
            insurance_type: "health".to_string(),
            company_id,
            premium: 599.0,
            coverage_amount: 500000.0,
            eligibility: json!({
                "age": {"min": 18, "max": 70},
                "smoker": ["yes", "no", "occasionally"]
            }),
            benefits: vec!["Cashless hospitalization".to_string()],
            exclusions: vec!["No maternity cover".to_string()],
            created_at: Utc::now(),
            is_active: true,
        },
    ];

    for policy in policies {
        collection.insert_one(policy, None).await?;
    }

    println!("✅ Seeded {} policies", policies.len());
    Ok(())
}