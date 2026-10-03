use inscope::MessageError;
use inscope::count_words;
use inscope::validate_message;

use inscope::{MAX_MESSAGE_BYTES, MAX_WORDS};

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
    assert_eq!(validate_message("hello"), Ok(()));
    assert_eq!(validate_message("hello world"), Ok(()));
    assert_eq!(validate_message("hello world!"), Ok(()));
    assert_eq!(validate_message(" "), Err(MessageError::Empty));
    let long_message = "a".repeat(MAX_MESSAGE_BYTES + 1);
    assert_eq!(validate_message(&long_message), Err(MessageError::TooLong));
    let too_many_words = "a ".repeat(MAX_WORDS + 1);
    assert_eq!(
        validate_message(&too_many_words),
        Err(MessageError::TooManyWords)
    );
}
