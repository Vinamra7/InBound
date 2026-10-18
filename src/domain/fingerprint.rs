use serde::Deserialize;

use crate::domain::edge::EdgeInfo;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct FingerprintPayload {
    pub device_id: String,
    pub tz: String,   //timezone
    pub lang: String, //language
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FpVerdict {
    Ok,
    Suspicious,
}

// §4 strike scoring: one honest mismatch (VPN / mobile geo drift) must not restrict
// a user — only two or more strikes do. A malformed device_id is a hard fail.
pub fn validate_fingerprint(payload: &FingerprintPayload, edge: &EdgeInfo) -> FpVerdict {
    if !validate_device_id(&payload.device_id) {
        return FpVerdict::Suspicious;
    }

    let mut strikes = 0;

    match &edge.tz {
        None => strikes += 1, // claim can't be verified against the edge
        Some(cf_tz) => {
            if payload.tz != *cf_tz {
                strikes += 1;
            }
        }
    }

    match &edge.lang {
        None => strikes += 1, // no Accept-Language header — curl/Postman
        Some(header) => {
            let claimed = payload.lang.to_lowercase();
            let in_header = header.split(',').any(|entry| {
                let primary = entry.split(';').next().unwrap_or("").trim().to_lowercase();
                primary == claimed
            });
            if !in_header {
                strikes += 1;
            }
        }
    }

    // Cloudflare-set, unforgeable; browsers speak h2/h3, quick scripts often HTTP/1.1
    if edge.http_protocol.as_deref() == Some("HTTP/1.1") {
        strikes += 1;
    }

    if strikes >= 2 {
        FpVerdict::Suspicious
    } else {
        FpVerdict::Ok
    }
}

fn validate_device_id(device_id: &str) -> bool {
    device_id.len() == 32
        && device_id
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}

pub fn is_valid_device_id(device_id: &str) -> bool {
    validate_device_id(device_id)
}
