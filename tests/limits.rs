use inscope::MessageError;
use inscope::count_words;
use inscope::validate_message;

use inscope::utf16_len;
use inscope::{MAX_USER_BYTES, MAX_USER_UTF16, MAX_USER_WORDS};

#[test]
fn test_count_words() {
    assert_eq!(count_words(""), 0);
    assert_eq!(count_words("hello"), 1);
    assert_eq!(count_words("hello world"), 2);
    assert_eq!(count_words("hello world! "), 2);
    assert_eq!(count_words(" "), 0);
}

#[test]
fn test_validate_message() {
    assert_eq!(validate_message(""), Err(MessageError::Empty));
    assert_eq!(validate_message(" "), Err(MessageError::Empty));
    assert_eq!(validate_message("hello"), Ok(()));

    let too_long_bytes = "a".repeat(MAX_USER_BYTES + 1);
    assert_eq!(
        validate_message(&too_long_bytes),
        Err(MessageError::TooLongBytes)
    );

    let too_many_units = "🙂".repeat(MAX_USER_UTF16 / 2 + 1); // 502 units, 1004 bytes
    assert_eq!(
        validate_message(&too_many_units),
        Err(MessageError::TooManyUnits)
    );

    let too_many_words = "a ".repeat(MAX_USER_WORDS + 1);
    assert_eq!(
        validate_message(&too_many_words),
        Err(MessageError::TooManyWords)
    );
}

#[test]
fn test_utf16_len() {
    assert_eq!(utf16_len("hello"), 5);
    assert_eq!(utf16_len("🙂"), 2);
    assert_eq!("🙂".len(), 4);
    assert_eq!(utf16_len(""), 0);
    assert_eq!(MAX_USER_UTF16, 500);
}
