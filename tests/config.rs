use inscope::config::AssistantConfig;
use inscope::config::load_default;

#[test]
fn default_config_is_valid() {
    assert!(load_default().validate().is_ok());
}

#[test]
fn empty_models_is_invalid() {
    let mut c = load_default();
    c.allowed_models.clear();
    assert!(c.validate().is_err());
}

#[test]
fn parses_json_config() {
    let json = r#"{"allowed_models":["example/alias-a"],"max_words":250,"max_bytes":4096}"#;
    let c: AssistantConfig = serde_json::from_str(json).unwrap();
    assert!(c.validate().is_ok());
}
