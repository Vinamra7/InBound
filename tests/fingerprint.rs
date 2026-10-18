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
        http_protocol: None,
    }
}

fn edge_http11(tz: Option<&str>, lang: Option<&str>) -> EdgeInfo {
    EdgeInfo {
        http_protocol: Some("HTTP/1.1".to_string()),
        ..edge(tz, lang)
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
fn single_tz_mismatch_is_ok() {
    // one strike only — VPN / mobile geo drift between browser tz and CF's geo-IP tz is honest
    let e = edge(Some("Asia/Kolkata"), Some("en-IN,en;q=0.9"));
    assert_eq!(
        validate_fingerprint(&payload(DEVICE, "Europe/London", "en-IN"), &e),
        FpVerdict::Ok
    );
}

#[test]
fn absent_lang_alone_is_ok() {
    // no Accept-Language = one strike; with a coherent tz that's not enough to restrict
    let e = edge(Some("Asia/Kolkata"), None);
    assert_eq!(
        validate_fingerprint(&payload(DEVICE, "Asia/Kolkata", "en-IN"), &e),
        FpVerdict::Ok
    );
}

#[test]
fn claimed_lang_not_in_header_alone_is_ok() {
    let e = edge(Some("Asia/Kolkata"), Some("en-IN,en;q=0.9"));
    assert_eq!(
        validate_fingerprint(&payload(DEVICE, "Asia/Kolkata", "en-GB"), &e),
        FpVerdict::Ok
    );
}

#[test]
fn http1_1_alone_is_ok() {
    let e = edge_http11(Some("Asia/Kolkata"), Some("en-IN,en;q=0.9"));
    assert_eq!(
        validate_fingerprint(&payload(DEVICE, "Asia/Kolkata", "en-IN"), &e),
        FpVerdict::Ok
    );
}

#[test]
fn two_strikes_are_suspicious() {
    // tz mismatch + lang mismatch — the classic replayed-payload shape
    let e = edge(Some("Asia/Kolkata"), Some("en-IN,en;q=0.9"));
    assert_eq!(
        validate_fingerprint(&payload(DEVICE, "Europe/London", "en-GB"), &e),
        FpVerdict::Suspicious
    );
}

#[test]
fn tz_mismatch_plus_http1_1_is_suspicious() {
    let e = edge_http11(Some("Asia/Kolkata"), Some("en-IN,en;q=0.9"));
    assert_eq!(
        validate_fingerprint(&payload(DEVICE, "Europe/London", "en-IN"), &e),
        FpVerdict::Suspicious
    );
}

#[test]
fn device_id_must_be_32_lowercase_hex() {
    // hard fail — a garbage identity is Suspicious even with zero other strikes
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
