use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TurnstileDecision {
    pub success: bool,
    pub hostname: Option<String>,
    pub action: Option<String>,
}

pub fn parse_turnstile(json: &str) -> Result<TurnstileDecision, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

pub fn turnstile_ok(
    decision: &TurnstileDecision,
    expected_hostname: &str,
    expected_action: &str,
) -> bool {
    decision.success
        && decision.hostname.as_deref() == Some(expected_hostname)
        && decision.action.as_deref() == Some(expected_action)
}
