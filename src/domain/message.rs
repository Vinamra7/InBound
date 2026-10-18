use serde::{Deserialize, Serialize};

pub const MAX_USER_WORDS: usize = 80;
pub const MAX_USER_BYTES: usize = 1536;
pub const MAX_AI_WORDS: usize = 300;
pub const MAX_AI_BYTES: usize = 3072;

// ---------- UI -> POST /api/chat ----------
// auth is the `Authorization: Bearer` header — the body carries no token.
// Turnstile is verified only at /api/session, never here.
#[derive(Debug, Deserialize)]
pub struct ChatRequestDto {
    pub model: ChatModelSlot,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum ChatModelSlot {
    #[serde(rename = "llm-1")]
    One,
    #[serde(rename = "llm-2")]
    Two,
    #[serde(rename = "llm-3")]
    Three,
}

impl ChatModelSlot {
    pub fn index(self) -> usize {
        match self {
            Self::One => 0,
            Self::Two => 1,
            Self::Three => 2,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MessageError {
    Empty,
    TooManyWords,
    TooLongBytes,
    SystemRole,
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
    match role {
        Role::System => Err(MessageError::SystemRole),
        Role::User => check_length(text, MAX_USER_WORDS, MAX_USER_BYTES),
        Role::Assistant => check_length(text, MAX_AI_WORDS, MAX_AI_BYTES),
    }
}

fn check_length(text: &str, max_words: usize, max_bytes: usize) -> Result<(), MessageError> {
    if count_words(text) == 0 {
        return Err(MessageError::Empty);
    }
    if text.len() > max_bytes {
        return Err(MessageError::TooLongBytes);
    }
    if count_words(text) > max_words {
        return Err(MessageError::TooManyWords);
    }
    Ok(())
}
