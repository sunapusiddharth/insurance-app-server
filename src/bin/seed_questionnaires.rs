use mongodb::{Client, Collection};
use serde_json::json;
use std::env;
use chrono::Utc;

#[derive(serde::Serialize)]
struct QuestionnaireConfig {
    insurance_type: String,
    version: String,
    questions: Vec<serde_json::Value>,
    created_at: chrono::DateTime<chrono::Utc>,
    created_by: String,
    is_active: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mongo_uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let client = Client::with_uri_str(mongo_uri).await?;
    let db = client.database("insurance_app");
    let collection: Collection<QuestionnaireConfig> = db.collection("questionnaire_configs");

    // List of configs — paste your JSONs here as serde_json::json!({})
    let configs = vec![
        json!({
            "insurance_type": "health",
            "version": "1.3",
            "questions": [ /* ... your health config questions ... */ ],
            "created_at": Utc::now(),
            "created_by": "system",
            "is_active": true
        }),
        json!({
            "insurance_type": "term",
            "version": "1.1",
            "questions": [ /* ... term config ... */ ],
            "created_at": Utc::now(),
            "created_by": "system",
            "is_active": true
        }),
        // Add others: motor_car, motor_bike, travel, home, ulip_sip
    ];

    for config in configs {
        let config_typed: QuestionnaireConfig = serde_json::from_value(config)?;
        collection.insert_one(config_typed, None).await?;
    }

    println!("✅ Seeded {} questionnaire configs", configs.len());
    Ok(())
}