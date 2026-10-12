use inscope::config::ConfigError;
use inscope::domain::message::MessageError;
use inscope::domain::token::TokenError;
use inscope::error::AppError;

#[test]
fn status_codes_match_the_doc_table() {
    assert_eq!(AppError::BadRequest.status(), 400);
    assert_eq!(AppError::Unauthorized.status(), 401);
    assert_eq!(AppError::HumanCheckFailed.status(), 403);
    assert_eq!(AppError::RateLimited.status(), 429);
    assert_eq!(AppError::BudgetExceeded.status(), 503);
    assert_eq!(AppError::Upstream.status(), 502);
    assert_eq!(AppError::Internal.status(), 500);
}

#[test]
fn codes_are_stable_strings() {
    assert_eq!(AppError::RateLimited.code(), "rate_limited");
    assert_eq!(AppError::Internal.code(), "internal");
}

#[test]
fn error_families_map_to_app_error() {
    assert_eq!(AppError::from(TokenError::Expired), AppError::Unauthorized);
    assert_eq!(
        AppError::from(TokenError::BadSignature),
        AppError::Unauthorized
    );
    assert_eq!(
        AppError::from(MessageError::TooManyWords),
        AppError::BadRequest
    );
    assert_eq!(AppError::from(ConfigError::WeakHmacKey), AppError::Internal);
}
