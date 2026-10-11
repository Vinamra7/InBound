use serde::{Deserialize, Serialize};

pub const SESSION_TTL_SECS: u64 = 24 * 60 * 60; // 24 hours in seconds

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
    pub sid: String,
    pub ip_prefix: String,
    pub iat: u64,
    pub exp: u64,
}
