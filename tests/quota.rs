use inscope::config::{USER_QUOTA_NORMAL, USER_QUOTA_RESTRICTED};
use inscope::domain::token::{Q_NORMAL, Q_RESTRICTED};
use inscope::pipeline::limits::user_limit_for;

#[test]
fn test_user_limit_for() {
    assert_eq!(user_limit_for(Q_NORMAL), USER_QUOTA_NORMAL);
    assert_eq!(user_limit_for(Q_RESTRICTED), USER_QUOTA_RESTRICTED);
    assert_eq!(user_limit_for(99), USER_QUOTA_NORMAL); // unknown tier → full quota
}
