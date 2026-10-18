use worker::Request;

use crate::domain::token::{TokenRaw, verify};
use crate::error::AppError;

pub fn authenticate(req: &Request, key: &[u8], now_secs: u64) -> Result<TokenRaw, AppError> {
    let header = req
        .headers()
        .get("Authorization")
        .ok()
        .flatten()
        .ok_or(AppError::Unauthorized)?;

    let token = header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?
        .trim();

    verify(token, key, now_secs).map_err(AppError::from)
}
