pub const MAX_USER_WORDS: usize = 80;
pub const MAX_USER_UTF16: usize = 500;
pub const MAX_USER_BYTES: usize = 2048;

#[derive(Debug, PartialEq)]
pub enum MessageError {
    Empty,
    TooManyWords,
    TooManyUnits,
    TooLongBytes,
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
