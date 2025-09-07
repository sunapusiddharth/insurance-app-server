// Add these imports
mod models;
mod handlers;

use handlers::questionnaire::{get_questionnaire_config, submit_questionnaire};

// Inside main() where you set up router:
let app = Router::new()
    .route("/api/questionnaire/:insurance_type", get(get_questionnaire_config))
    .route("/api/questionnaire/submit", post(submit_questionnaire))
    // ... your existing routes
    .with_state(app_state);

    .route("/api/policies/filter", post(policy::filter_policies))
    .route("/api/questionnaire/responses/:user_id/:insurance_type", get(get_last_active_responses))