use inscope::domain::message::MessageError;
use inscope::domain::message::count_words;
use inscope::domain::message::validate_message;

use inscope::domain::message::Role;
use inscope::domain::message::{MAX_AI_BYTES, MAX_USER_BYTES, MAX_USER_WORDS};

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
    assert_eq!(validate_message("", Role::User), Err(MessageError::Empty));
    assert_eq!(validate_message(" ", Role::User), Err(MessageError::Empty));
    assert_eq!(validate_message("hello", Role::User), Ok(()));

    let too_long_bytes = "a".repeat(MAX_USER_BYTES + 1);
    assert_eq!(
        validate_message(&too_long_bytes, Role::User),
        Err(MessageError::TooLongBytes)
    );

    let too_many_words = "a ".repeat(MAX_USER_WORDS + 1);
    assert_eq!(
        validate_message(&too_many_words, Role::User),
        Err(MessageError::TooManyWords)
    );
}

#[test]
fn test_validate_message_role_limits() {
    // 150 words (450 bytes): exceeds User's 80-word cap, fits AI's 300w/3072b
    let long_content = "hi ".repeat(150);
    assert_eq!(
        validate_message(&long_content, Role::User),
        Err(MessageError::TooManyWords)
    );
    assert_eq!(validate_message(&long_content, Role::Assistant), Ok(()));

    // over AI's byte cap
    let huge_reply = "a".repeat(MAX_AI_BYTES + 1);
    assert_eq!(
        validate_message(&huge_reply, Role::Assistant),
        Err(MessageError::TooLongBytes)
    );

    // System has a 0-byte budget in validate_message — never a valid client-written message
    assert_eq!(
        validate_message("hi", Role::System),
        Err(MessageError::SystemRole)
    );
    assert_eq!(
        validate_message("", Role::System),
        Err(MessageError::SystemRole)
    );
}
