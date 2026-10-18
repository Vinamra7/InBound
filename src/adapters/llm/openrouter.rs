use serde::Deserialize;
use serde_json::json;
use worker::console_log;
use worker::wasm_bindgen::JsValue;
use worker::{Fetch, Headers, Method, Request, RequestInit};

use crate::constants::{
    CHAT_MODEL, CLASSIFIER_MAX_TOKENS, CLASSIFIER_MODEL, OPENROUTER_URL, REPLY_MAX_TOKENS,
};
use crate::domain::message::{Message, Role, UsageDto};
use crate::error::AppError;

const CLASSIFIER_SYSTEM: &str = "You are the scope gate for a personal portfolio chatbot about Vinamra Mishra. \
Reply with exactly one word: \"yes\" if the user's message is about Vinamra — his work, projects, skills, \
experience, or contacting him — or a natural conversational reply in that context. Reply \"no\" otherwise.";

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
    usage: Option<ApiUsage>,
}

#[derive(Deserialize)]
struct Choice {
    message: RespMessage,
}

#[derive(Deserialize)]
struct RespMessage {
    content: Option<String>,
}

#[derive(Deserialize)]
struct ApiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

async fn post_chat(api_key: &str, body: serde_json::Value) -> Result<String, AppError> {
    let mut init = RequestInit::new();
    init.with_method(Method::Post);

    let headers = Headers::new();
    headers
        .append("Content-Type", "application/json")
        .map_err(|_| AppError::Upstream)?;
    headers
        .append("Authorization", &format!("Bearer {api_key}"))
        .map_err(|_| AppError::Upstream)?;
    headers
        .append("HTTP-Referer", "https://vinamramishra.dev")
        .map_err(|_| AppError::Upstream)?;
    headers
        .append("X-Title", "InScope")
        .map_err(|_| AppError::Upstream)?;
    init.with_headers(headers);

    init.with_body(Some(JsValue::from_str(&body.to_string())));

    let req = Request::new_with_init(OPENROUTER_URL, &init).map_err(|_| AppError::Upstream)?;
    let mut resp = Fetch::Request(req)
        .send()
        .await
        .map_err(|_| AppError::Upstream)?;

    if resp.status_code() != 200 {
        return Err(AppError::Upstream);
    }

    resp.text().await.map_err(|_| AppError::Upstream)
}

fn parse_choice(text: &str) -> Result<(String, Option<UsageDto>), AppError> {
    let parsed: ChatCompletionResponse =
        serde_json::from_str(text).map_err(|_| AppError::Upstream)?;

    let content = parsed
        .choices
        .first()
        .and_then(|c| c.message.content.clone())
        .ok_or(AppError::Upstream)?;

    Ok((
        content,
        parsed.usage.map(|u| UsageDto {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        }),
    ))
}

pub async fn complete(
    api_key: &str,
    system_prompt: &str,
    messages: &[Message],
) -> Result<(String, Option<UsageDto>), AppError> {
    let mut llm_messages = vec![json!({"role": "system", "content": system_prompt})];
    for m in messages {
        let role = match m.role {
            Role::User => "user",
            Role::Assistant | Role::System => "assistant",
        };
        llm_messages.push(json!({"role": role, "content": m.content}));
    }

    let body = json!({
        "model": CHAT_MODEL,
        "messages": llm_messages,
        "max_tokens": REPLY_MAX_TOKENS,
    });

    let text = post_chat(api_key, body).await?;
    parse_choice(&text)
}

pub async fn classify(api_key: &str, message: &str) -> Result<bool, AppError> {
    let body = json!({
        "model": CLASSIFIER_MODEL,
        "messages": [
            {"role": "system", "content": CLASSIFIER_SYSTEM},
            {"role": "user", "content": message},
        ],
        "max_tokens": CLASSIFIER_MAX_TOKENS,
        "temperature": 0,
    });

    let text = post_chat(api_key, body).await?;
    let (content, _) = match parse_choice(&text) {
        Ok(pair) => pair,
        Err(e) => {
            console_log!("classifier unparseable, failing open: {text}");
            return Err(e);
        }
    };

    // unparseable output → fail-open (§3) — only absence of a clear "yes" blocks
    Ok(content.to_lowercase().contains("yes"))
}
