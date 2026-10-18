use serde::Deserialize;

use crate::constants::{SITE_HOST, SITEVERIFY_URL, TURNSTILE_ACTION};
use crate::error::AppError;
use worker::wasm_bindgen::JsValue;
use worker::{Fetch, Headers, Method, Request, RequestInit};

#[derive(Debug, Deserialize)]
pub struct SiteverifyResponse {
    pub success: bool,
    pub hostname: Option<String>,
    pub action: Option<String>,
}

pub fn ok(resp: &SiteverifyResponse) -> bool {
    resp.success
        && resp.hostname.as_deref() == Some(SITE_HOST)
        && resp.action.as_deref() == Some(TURNSTILE_ACTION)
}

pub async fn verify(secret: &str, widget_token: &str, ip: &str) -> Result<bool, AppError> {
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

    let mut resp = Fetch::Request(req).send().await?;

    if resp.status_code() != 200 {
        return Err(AppError::Upstream);
    }

    let text = resp.text().await.map_err(|_| AppError::Upstream)?;
    let parsed: SiteverifyResponse = serde_json::from_str(&text).map_err(|_| AppError::Upstream)?;

    Ok(ok(&parsed))
}
