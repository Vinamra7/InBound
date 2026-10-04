use worker::{Context, Env, Method, Request, Response, event};

pub const MAX_USER_WORDS: usize = 80;
pub const MAX_USER_UTF16: usize = 500;
pub const MAX_USER_BYTES: usize = 2048;

pub mod config;
pub mod security;

#[derive(Debug, PartialEq)]
pub enum MessageError {
    Empty,
    TooManyWords,
    TooManyUnits,
    TooLongBytes,
}

#[event(fetch)]
pub async fn health(req: Request, _env: Env, _ctx: Context) -> worker::Result<Response> {
    if matches!(req.method(), Method::Get) && req.path() == "/health" {
        Response::from_json(&serde_json::json!({"ok": true}))
    } else {
        Response::error("Not Found", 404)
    }
}

pub fn is_allowed_model(model: &str, allowed: &[String]) -> bool {
    allowed.iter().any(|m| m == model)
}

pub fn remaining_quota(limit: u32, used: u32) -> u32 {
    if used >= limit { 0 } else { limit - used }
}

pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn utf16_len(text: &str) -> usize {
    text.encode_utf16().count()
}

pub fn validate_message(text: &str) -> Result<(), MessageError> {
    let count = count_words(text);
    if count == 0 {
        return Err(MessageError::Empty);
    }
    if text.len() > MAX_USER_BYTES {
        return Err(MessageError::TooLongBytes);
    }
    if utf16_len(text) > MAX_USER_UTF16 {
        return Err(MessageError::TooManyUnits);
    }
    if count > MAX_USER_WORDS {
        return Err(MessageError::TooManyWords);
    }
    Ok(())
}
