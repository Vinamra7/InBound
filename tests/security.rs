use inscope::adapters::turnstile::{SiteverifyResponse, ok, ok_with_localhost};

fn resp(success: bool, hostname: Option<&str>, action: Option<&str>) -> SiteverifyResponse {
    SiteverifyResponse {
        success,
        hostname: hostname.map(|h| h.to_string()),
        action: action.map(|a| a.to_string()),
    }
}

#[test]
fn valid_turnstile_accepted() {
    assert!(ok(&resp(true, Some("vinamramishra.dev"), Some("session"))));
}

#[test]
fn www_host_accepted() {
    assert!(ok(&resp(
        true,
        Some("www.vinamramishra.dev"),
        Some("session")
    )));
}

#[test]
fn vercel_fallback_host_accepted() {
    assert!(ok(&resp(
        true,
        Some("vinamra-portfolio-beta.vercel.app"),
        Some("session")
    )));
}

#[test]
fn localhost_dev_host_accepted() {
    assert!(ok_with_localhost(
        &resp(true, Some("localhost"), Some("session")),
        true
    ));
    assert!(!ok(&resp(true, Some("localhost"), Some("session"))));
}

#[test]
fn unknown_subdomain_rejected() {
    assert!(!ok(&resp(
        true,
        Some("evil.vinamramishra.dev"),
        Some("session")
    )));
}

#[test]
fn success_false_rejected() {
    assert!(!ok(&resp(
        false,
        Some("vinamramishra.dev"),
        Some("session")
    )));
}

#[test]
fn wrong_hostname_rejected() {
    assert!(!ok(&resp(true, Some("evil.com"), Some("session"))));
}

#[test]
fn missing_hostname_rejected() {
    assert!(!ok(&resp(true, None, Some("session"))));
}

#[test]
fn wrong_action_rejected() {
    assert!(!ok(&resp(true, Some("vinamramishra.dev"), Some("chat"))));
}

#[test]
fn missing_action_rejected() {
    assert!(!ok(&resp(true, Some("vinamramishra.dev"), None)));
}

#[test]
fn real_siteverify_json_shape_parses_kebab() {
    // include extra unknown field "error-codes" — serde must skip it silently
    let json = r#"{"success":true,"challenge_ts":"2022-02-28T15:14:30.096Z","hostname":"vinamramishra.dev","error-codes":[],"action":"session","cdata":"x"}"#;
    let parsed: SiteverifyResponse = serde_json::from_str(json).unwrap();
    assert!(ok(&parsed));
}
