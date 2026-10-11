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

    EdgeInfo { ip, country, asn }
}
