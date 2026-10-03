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
