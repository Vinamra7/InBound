use inscope::remaining_quota;

#[test]
fn test_remaining_quota() {
    assert_eq!(remaining_quota(100, 50), 50);
    assert_eq!(remaining_quota(100, 100), 0);
    assert_eq!(remaining_quota(100, 150), 0);
}
