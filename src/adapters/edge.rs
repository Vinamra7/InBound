use crate::domain::edge::EdgeInfo;
use worker::Request;

/// Reads what Cloudflare's edge injected. Call BEFORE parsing the body —
/// JSON parsing consumes the request.
pub fn extract(req: &Request) -> EdgeInfo {
    let ip = req
        .headers()
        .get("CF-Connecting-IP")
        .ok()
        .flatten()
        .unwrap_or_else(|| "0.0.0.0".to_string());

    let country = req.headers().get("CF-IPCountry").ok().flatten();
    let asn = req.cf().and_then(|cf| cf.asn());

    let tz = req
        .cf()
        .filter(|cf| !cf.timezone_name().is_empty())
        .map(|cf| cf.timezone_name());

    let lang = req.headers().get("Accept-Language").ok().flatten();

    let http_protocol = req.cf().map(|cf| cf.http_protocol());

    EdgeInfo {
        ip,
        country,
        asn,
        tz,
        lang,
        http_protocol,
    }
}
