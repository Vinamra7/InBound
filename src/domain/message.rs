use serde::{Deserialize, Serialize};

pub const MAX_USER_WORDS: usize = 80;
pub const MAX_USER_BYTES: usize = 1536;
pub const MAX_AI_WORDS: usize = 300;
pub const MAX_AI_BYTES: usize = 3072;

// ---------- UI -> POST /api/chat ----------
// token: the HMAC session token from /api/session — None = "first visit", goes 401 → client then calls /api/session
// (turnstile NOT here: it's verified only at /api/session; fingerprint deferred, §11)
#[derive(Debug, Deserialize)]
pub struct ChatRequestDto {
    pub token: Option<String>,
    pub messages: Vec<Message>,
}

#[derive(Debug, PartialEq)]
pub enum MessageError {
    Empty,
    TooManyWords,
    TooLongBytes,
}

#[derive(Debug, Serialize)]
pub struct ChatResponseDto {
    pub reply: String,
    pub usage: Option<UsageDto>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct UsageDto {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    #[serde(alias = "agent")]
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn validate_message(text: &str, role: Role) -> Result<(), MessageError> {
    let count = count_words(text);
    let max_words = match role {
        Role::User => MAX_USER_WORDS,
        Role::Assistant => MAX_AI_WORDS,
        Role::System => 0,
    };
    let max_bytes = match role {
        Role::User => MAX_USER_BYTES,
        Role::Assistant => MAX_AI_BYTES,
        Role::System => 0,
    };
    if count == 0 {
        return Err(MessageError::Empty);
    }
    if text.len() > max_bytes {
        return Err(MessageError::TooLongBytes);
    }
    if count > max_words {
        return Err(MessageError::TooManyWords);
    }
    Ok(())
}
