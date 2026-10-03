use inscope::is_allowed_model;

#[test]
fn test_is_allowed_model() {
    let allowed = ["claude-opus-5.5".to_owned(), "gpt-6.1-sol".to_owned()];
    let model1 = "claude-opus-5.5";
    let model2 = "claude-opus-5.4";
    assert!(is_allowed_model(model1, &allowed));
    assert!(!is_allowed_model(model2, &allowed));
}
