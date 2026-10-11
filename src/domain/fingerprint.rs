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

pub fn validate_fingerprint(payload: &FingerprintPayload, edge: &EdgeInfo) -> FpVerdict {
    let Some(cf_tz) = &edge.tz else {
        return FpVerdict::Suspicious;
    };

    if payload.tz != *cf_tz {
        return FpVerdict::Suspicious;
    }

    let Some(cf_lang) = &edge.lang else {
        return FpVerdict::Suspicious;
    };

    let claimed = payload.lang.to_lowercase();
    let matches = cf_lang.split(',').any(|entry| {
        let primary = entry.split(';').next().unwrap_or("").trim().to_lowercase();
        primary == claimed
    });

    if !matches {
        return FpVerdict::Suspicious;
    }

    match validate_device_id(&payload.device_id) {
        true => FpVerdict::Ok,
        false => FpVerdict::Suspicious,
    }
}

fn validate_device_id(device_id: &str) -> bool {
    device_id.len() == 32
        && device_id
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
