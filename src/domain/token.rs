use serde::{Deserialize, Serialize};

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as B64;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub const SESSION_TTL_SECS: u64 = 24 * 60 * 60; // 24 hours in seconds

// quota tier carried inside the token (§11 fp/ASN risk collapsed to one claim)
pub const Q_NORMAL: u8 = 1; // full session quota
pub const Q_RESTRICTED: u8 = 0; // incoherent fingerprint / datacenter ASN

#[derive(Debug, Deserialize)]
pub struct SessionRequestDto {
    pub turnstile: String,
}

#[derive(Debug, Serialize)]
pub struct SessionResponseDto {
    pub token: String,
    pub expires_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenRaw {
    pub device_id: String, // fingerprint id — the per-user identity; clearing cookies does NOT rotate it
    pub q: u8,             // quota tier at issuance — Q_NORMAL | Q_RESTRICTED
    pub iat: u64,          // "issued at" — unix seconds when the token was minted
    pub exp: u64,          // "expires" — iat + SESSION_TTL_SECS (24h); server rejects past this
}

#[derive(Debug, PartialEq)]
pub enum TokenError {
    Malformed,    // no dot / bad base64 / bad json
    BadSignature, // HMAC mismatch — tampered or foreign key
    Expired,      // exp <= now
}

impl TokenRaw {
    pub fn new(device_id: String, q: u8, now_secs: u64) -> Self {
        Self {
            device_id,
            q,
            iat: now_secs,
            exp: now_secs + SESSION_TTL_SECS,
        }
    }
    pub fn is_expired(&self, now_secs: u64) -> bool {
        self.exp <= now_secs
    }
}

pub fn sign(raw: &TokenRaw, key: &[u8]) -> Result<String, TokenError> {
    let payload_json = serde_json::to_vec(raw).map_err(|_| TokenError::Malformed)?;
    let payload = B64.encode(&payload_json);

    let mut mac = Hmac::<Sha256>::new_from_slice(key).map_err(|_| TokenError::Malformed)?;
    mac.update(payload.as_bytes());
    let sig = B64.encode(mac.finalize().into_bytes());

    Ok(format!("{payload}.{sig}"))
}

pub fn verify(token: &str, key: &[u8], now_secs: u64) -> Result<TokenRaw, TokenError> {
    let (payload, sig) = token.split_once('.').ok_or(TokenError::Malformed)?;

    let payload_bytes = B64.decode(payload).map_err(|_| TokenError::Malformed)?;
    let raw: TokenRaw =
        serde_json::from_slice(&payload_bytes).map_err(|_| TokenError::Malformed)?;
    let sig_bytes = B64.decode(sig).map_err(|_| TokenError::Malformed)?;

    let mut mac = Hmac::<Sha256>::new_from_slice(key).map_err(|_| TokenError::Malformed)?;
    mac.update(payload.as_bytes());
    mac.verify_slice(&sig_bytes)
        .map_err(|_| TokenError::BadSignature)?;

    if raw.is_expired(now_secs) {
        return Err(TokenError::Expired);
    }
    Ok(raw)
}
