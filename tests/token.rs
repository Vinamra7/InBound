use inscope::domain::token::Q_NORMAL;
use inscope::domain::token::Q_RESTRICTED;
use inscope::domain::token::SESSION_TTL_SECS;
use inscope::domain::token::TokenError;
use inscope::domain::token::TokenRaw;
use inscope::domain::token::sign;
use inscope::domain::token::verify;

const KEY: &[u8] = b"test-key-must-be-at-least-32-characters!!";
const NOW: u64 = 1_000_000;

fn raw(device_id: &str) -> TokenRaw {
    TokenRaw::new(device_id.to_string(), Q_NORMAL, NOW)
}

fn parts(token: &str) -> (&str, &str) {
    token.split_once('.').unwrap()
}

#[test]
fn sign_then_verify_roundtrips() {
    let token = sign(&raw("abc123"), KEY).unwrap();
    assert_eq!(verify(&token, KEY, NOW + 100).unwrap(), raw("abc123"));
}

#[test]
fn tampered_payload_rejected() {
    // attacker swaps the payload but keeps a signature from another token
    let victim = sign(&raw("abc123"), KEY).unwrap();
    let evil_token = sign(&raw("evil"), KEY).unwrap();
    let forged = format!("{}.{}", parts(&evil_token).0, parts(&victim).1);

    assert_eq!(verify(&forged, KEY, NOW), Err(TokenError::BadSignature));
}

#[test]
fn wrong_key_rejected() {
    let token = sign(&raw("abc123"), KEY).unwrap();
    assert_eq!(
        verify(&token, b"another-key-at-least-32-characters!!!!", NOW),
        Err(TokenError::BadSignature)
    );
}

#[test]
fn expired_rejected() {
    let token = sign(&raw("abc123"), KEY).unwrap();

    assert_eq!(
        verify(&token, KEY, NOW + SESSION_TTL_SECS),
        Err(TokenError::Expired)
    ); // exactly at exp — already too late
    assert_eq!(
        verify(&token, KEY, NOW + SESSION_TTL_SECS + 1),
        Err(TokenError::Expired)
    );
}

#[test]
fn restricted_tier_survives_roundtrip() {
    let t = TokenRaw::new("evil".to_string(), Q_RESTRICTED, NOW);
    let token = sign(&t, KEY).unwrap();
    assert_eq!(verify(&token, KEY, NOW + 10).unwrap(), t);
}

#[test]
fn malformed_inputs_rejected() {
    assert_eq!(verify("nodot", KEY, NOW), Err(TokenError::Malformed));
    assert_eq!(verify("!!!.abc", KEY, NOW), Err(TokenError::Malformed)); // payload not base64
    assert_eq!(verify("e30.abc", KEY, NOW), Err(TokenError::Malformed)); // "e30" = "{}" — no token fields
}

#[test]
fn contains_two_parts() {
    let token = sign(&raw("abc123"), KEY).unwrap();
    let (payload, sig) = parts(&token);
    assert!(!payload.is_empty() && !sig.is_empty());
    assert_eq!(token.matches('.').count(), 1);
}
