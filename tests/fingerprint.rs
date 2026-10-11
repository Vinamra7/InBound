use inscope::domain::edge::EdgeInfo;
use inscope::domain::fingerprint::FingerprintPayload;
use inscope::domain::fingerprint::FpVerdict;
use inscope::domain::fingerprint::validate_fingerprint;

const DEVICE: &str = "3a7f2b9c8d41e5f6a0b3c2d1e4f7a8b6"; // 32 lowercase hex — FingerprintJS OSS shape

fn payload(device_id: &str, tz: &str, lang: &str) -> FingerprintPayload {
    FingerprintPayload {
        device_id: device_id.to_string(),
        tz: tz.to_string(),
        lang: lang.to_string(),
    }
}

fn edge(tz: Option<&str>, lang: Option<&str>) -> EdgeInfo {
    EdgeInfo {
        ip: "49.36.x.x".to_string(),
        country: Some("IN".to_string()),
        asn: Some(55836),
        tz: tz.map(|t| t.to_string()),
        lang: lang.map(|l| l.to_string()),
    }
}

#[test]
fn coherent_browser_is_ok() {
    let e = edge(Some("Asia/Kolkata"), Some("en-IN,en;q=0.9,hi;q=0.8"));
    assert_eq!(
        validate_fingerprint(&payload(DEVICE, "Asia/Kolkata", "en-IN"), &e),
        FpVerdict::Ok
    );
}

#[test]
fn claimed_lang_can_sit_anywhere_in_header() {
    // 2nd entry carries a q-weight and the claimed lang is only 3rd — still Ok
    let e = edge(
        Some("Asia/Kolkata"),
        Some("hi-IN,q=1.0,en-IN;q=0.9,ta;q=0.3"),
    );
    assert_eq!(
        validate_fingerprint(&payload(DEVICE, "Asia/Kolkata", "en-IN"), &e),
        FpVerdict::Ok
    );
}

#[test]
fn case_folding_forgives_safari_style() {
    // Safari-style header casing vs browser-API casing — one honest human, zero suspicion
    let e = edge(Some("Asia/Kolkata"), Some("en-in,en;q=0.9"));
    assert_eq!(
        validate_fingerprint(&payload(DEVICE, "Asia/Kolkata", "en-IN"), &e),
        FpVerdict::Ok
    );
}

#[test]
fn wrong_tz_is_suspicious() {
    let e = edge(Some("Asia/Kolkata"), Some("en-IN,en;q=0.9"));
    let p = payload(DEVICE, "Europe/London", "en-IN");
    assert_eq!(validate_fingerprint(&p, &e), FpVerdict::Suspicious); // VPN / replayed payload
}

#[test]
fn absent_lang_is_suspicious() {
    let e = edge(Some("Asia/Kolkata"), None);
    assert_eq!(
        validate_fingerprint(&payload(DEVICE, "Asia/Kolkata", "en-IN"), &e),
        FpVerdict::Suspicious
    ); // curl/Postman send no Accept-Language — refusal avoids spoof-by-omission
}

#[test]
fn claimed_lang_not_in_header_is_suspicious() {
    let e = edge(Some("Asia/Kolkata"), Some("en-IN,en;q=0.9"));
    assert_eq!(
        validate_fingerprint(&payload(DEVICE, "Asia/Kolkata", "en-GB"), &e),
        FpVerdict::Suspicious
    );
}

#[test]
fn device_id_must_be_32_lowercase_hex() {
    let e = edge(Some("Asia/Kolkata"), Some("en-IN,en;q=0.9"));

    let short = payload(&DEVICE[..31], "Asia/Kolkata", "en-IN");
    assert_eq!(validate_fingerprint(&short, &e), FpVerdict::Suspicious);

    let mut upper = DEVICE.to_uppercase();
    upper.replace_range(0..1, &DEVICE[0..1]); // keep length 32, uppercase one HEX digit
    assert_eq!(
        validate_fingerprint(&payload(&upper, "Asia/Kolkata", "en-IN"), &e),
        FpVerdict::Suspicious
    );
}
