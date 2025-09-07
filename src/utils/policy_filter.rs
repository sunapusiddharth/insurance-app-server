use crate::models::policy::Policy;
use serde_json::Value;

pub fn filter_and_rank_policies(
    policies: Vec<Policy>,
    user_responses: &Value,
) -> Vec<Policy> {
    let mut scored_policies: Vec<(Policy, i32)> = policies
        .into_iter()
        .map(|policy| {
            let score = calculate_match_score(&policy, user_responses);
            (policy, score)
        })
        .collect();

    // Sort by score descending
    scored_policies.sort_by(|a, b| b.1.cmp(&a.1));

    // Return only policies with score > 0
    scored_policies
        .into_iter()
        .filter(|(_, score)| *score > 0)
        .map(|(policy, _)| policy)
        .collect()
}

fn calculate_match_score(policy: &Policy, user_responses: &Value) -> i32 {
    let mut score = 0;

    if let Some(eligibility) = policy.eligibility.as_object() {
        for (key, required_value) in eligibility {
            if let Some(user_value) = user_responses.get(key) {
                if is_match(user_value, required_value) {
                    score += 10;
                } else {
                    // Hard reject if doesn't match mandatory criteria
                    if is_mandatory_rejection(key, user_value, required_value) {
                        return 0;
                    }
                }
            }
        }
    }

    // Bonus for coverage amount match
    if let Some(user_sum_insured) = user_responses.get("sum_insured").and_then(|v| v.as_str()).and_then(|s| s.parse::<f64>().ok()) {
        let diff = (policy.coverage_amount - user_sum_insured).abs();
        if diff <= policy.coverage_amount * 0.2 {
            score += 5;
        }
    }

    score
}

fn is_match(user_value: &Value, required_value: &Value) -> bool {
    match (user_value, required_value) {
        (Value::String(user_str), Value::Array(arr)) => {
            arr.iter().any(|v| v == user_str)
        }
        (Value::Number(user_num), Value::Array(arr)) => {
            arr.iter().any(|v| v == user_num)
        }
        (Value::Array(user_arr), Value::Array(req_arr)) => {
            user_arr.iter().any(|uv| req_arr.contains(uv))
        }
        _ => user_value == required_value,
    }
}

fn is_mandatory_rejection(key: &str, user_value: &Value, required_value: &Value) -> bool {
    let hard_constraints = ["age", "gender", "smoker", "pre_existing"];
    if hard_constraints.contains(&key) {
        !is_match(user_value, required_value)
    } else {
        false
    }
}