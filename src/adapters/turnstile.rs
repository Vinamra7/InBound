use serde::Deserialize;

use crate::adapters::http;
use crate::constants::{ALLOWED_HOSTS, SITEVERIFY_URL, TURNSTILE_ACTION};
use crate::error::AppError;
use worker::wasm_bindgen::JsValue;
use worker::{Headers, Method, Request, RequestInit};

#[derive(Debug, Deserialize)]
pub struct SiteverifyResponse {
    pub success: bool,
    pub hostname: Option<String>,
    pub action: Option<String>,
}

pub fn ok(resp: &SiteverifyResponse) -> bool {
    ok_with_localhost(resp, false)
}

pub fn ok_with_localhost(resp: &SiteverifyResponse, allow_localhost: bool) -> bool {
    resp.success
        && resp.hostname.as_deref().is_some_and(|h| {
            ALLOWED_HOSTS.contains(&h)
                || (allow_localhost && matches!(h, "localhost" | "127.0.0.1"))
        })
        && resp.action.as_deref() == Some(TURNSTILE_ACTION)
}

pub async fn verify(
    secret: &str,
    widget_token: &str,
    ip: &str,
    allow_localhost: bool,
) -> Result<bool, AppError> {
    let body = serde_json::json!({
        "secret": secret,
        "response": widget_token,
        "remoteip": ip,
    });

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    let headers = Headers::new();
    headers
        .append("Content-Type", "application/json")
        .map_err(AppError::from)?;
    init.with_headers(headers);
    init.with_body(Some(JsValue::from_str(&body.to_string())));

    let req = Request::new_with_init(SITEVERIFY_URL, &init).map_err(AppError::from)?;

    let bytes = http::fetch_body(req, 8_000, 16 * 1024).await?;
    let parsed: SiteverifyResponse =
        serde_json::from_slice(&bytes).map_err(|_| AppError::Upstream)?;

    Ok(ok_with_localhost(&parsed, allow_localhost))
}
