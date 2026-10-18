use inscope::config::ChatApi;
use inscope::domain::message::{ChatModelSlot, ChatRequestDto};
use serde_json::json;

#[test]
fn only_server_defined_slots_are_accepted() {
    for (slot, expected) in [
        ("llm-1", ChatModelSlot::One),
        ("llm-2", ChatModelSlot::Two),
        ("llm-3", ChatModelSlot::Three),
    ] {
        let dto: ChatRequestDto =
            serde_json::from_value(json!({"model":slot,"messages":[]})).unwrap();
        assert_eq!(dto.model, expected);
    }
    for slot in [
        "llm-0",
        "llm-4",
        "deepseek-v4.1-flash-free",
        "gpt-6-luna",
        "",
        "LLM-1",
    ] {
        assert!(
            serde_json::from_value::<ChatRequestDto>(json!({"model":slot,"messages":[]})).is_err()
        );
    }
    assert!(serde_json::from_value::<ChatRequestDto>(json!({"messages":[]})).is_err());
}

#[test]
fn unsupported_api_formats_fail_configuration() {
    assert_eq!(
        ChatApi::parse("chat_completions").unwrap(),
        ChatApi::ChatCompletions
    );
    assert_eq!(ChatApi::parse("responses").unwrap(), ChatApi::Responses);
    assert!(ChatApi::parse("https://attacker.invalid").is_err());
}
