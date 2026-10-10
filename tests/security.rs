use inscope::adapters::turnstile::{parse_turnstile, turnstile_ok};

#[test]
fn test_valid_turnstile_accepted() {
    let json = r#"{"success":true,"hostname":"example.com","action":"chat"}"#;
    let decision = parse_turnstile(json).unwrap();
    assert!(turnstile_ok(&decision, "example.com", "chat"));
}

#[test]
fn test_success_false_rejected() {
    let json = r#"{"success":false,"hostname":"example.com","action":"chat"}"#;
    let decision = parse_turnstile(json).unwrap();
    assert!(!turnstile_ok(&decision, "example.com", "chat"));
}

#[test]
fn test_malformed_json_errors() {
    assert!(parse_turnstile("not json").is_err());
    assert!(parse_turnstile(r#"{"foo":1}"#).is_err()); // missing required `success`
}

#[test]
fn test_wrong_hostname_rejected() {
    let json = r#"{"success":true,"hostname":"evil.com","action":"chat"}"#;
    let decision = parse_turnstile(json).unwrap();
    assert!(!turnstile_ok(&decision, "example.com", "chat"));
}
