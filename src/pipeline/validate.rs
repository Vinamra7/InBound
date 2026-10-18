use crate::domain::message::{Message, Role, validate_message};
use crate::error::AppError;

pub const MAX_MESSAGES: usize = 21; // 10 history pairs + the current message (§2)

pub fn validate_chat_request(messages: &[Message]) -> Result<(), AppError> {
    if messages.is_empty() || messages.len() > MAX_MESSAGES {
        return Err(AppError::BadRequest);
    }

    if messages.last().map(|m| m.role) != Some(Role::User) {
        return Err(AppError::BadRequest);
    }

    for m in messages {
        validate_message(&m.content, m.role).map_err(|_| AppError::BadRequest)?;
    }

    Ok(())
}
