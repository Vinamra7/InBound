use inscope::config::AssistantConfig;
use inscope::config::load_default;

#[test]
fn default_config_has_models() {
    assert!(!load_default().allowed_models.is_empty());
}

#[test]
fn parses_json_config() {
    let json = r#"{"allowed_models":["example/alias-a"]}"#;
    let c: AssistantConfig = serde_json::from_str(json).unwrap();
    assert_eq!(c.allowed_models, vec!["example/alias-a".to_string()]);
}
