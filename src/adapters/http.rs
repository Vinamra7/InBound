use futures_util::StreamExt;
use worker::{AbortSignal, ByteStream, Fetch, Request, Response};

use crate::error::AppError;

async fn read_stream(mut stream: ByteStream, cap: usize) -> worker::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        if chunk.len() > cap.saturating_sub(bytes.len()) {
            return Err(worker::Error::RustError("body limit exceeded".into()));
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

fn check_length(headers: &worker::Headers, cap: usize) -> worker::Result<()> {
    if let Some(length) = headers.get("Content-Length")? {
        let length = length
            .parse::<usize>()
            .map_err(|_| worker::Error::RustError("invalid body length".into()))?;
        if length > cap {
            return Err(worker::Error::RustError("body limit exceeded".into()));
        }
    }
    Ok(())
}

pub async fn request_body(req: &mut Request, cap: usize) -> Result<Vec<u8>, AppError> {
    check_length(req.headers(), cap).map_err(|_| AppError::BadRequest)?;
    read_stream(req.stream().map_err(|_| AppError::BadRequest)?, cap)
        .await
        .map_err(|_| AppError::BadRequest)
}

fn provider_error_code(bytes: &[u8]) -> Option<&'static str> {
    let value: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    let code = value.pointer("/error/type")?.as_str()?;
    [
        "FreeTierError",
        "AuthError",
        "AuthenticationError",
        "ModelError",
        "ModelAccessError",
        "ModelNotFoundError",
        "ModelDisabledError",
        "BalanceError",
        "InsufficientBalanceError",
        "BudgetExceededError",
        "RateLimitError",
        "ProviderError",
        "api_error",
    ]
    .into_iter()
    .find(|known| *known == code)
}

fn upstream_error(
    service: &'static str,
    stage: &str,
    status: Option<u16>,
    code: Option<&'static str>,
) -> AppError {
    worker::console_error!(
        "{}",
        serde_json::json!({
            "event": "upstream_failure",
            "service": service,
            "stage": stage,
            "status": status,
            "code": code,
        })
    );
    AppError::Upstream
}

pub async fn fetch_body(
    req: Request,
    timeout_ms: u32,
    cap: usize,
    service: &'static str,
) -> Result<Vec<u8>, AppError> {
    let signal: AbortSignal = worker::web_sys::AbortSignal::timeout_with_u32(timeout_ms).into();
    let mut response: Response = Fetch::Request(req)
        .send_with_signal(&signal)
        .await
        .map_err(|_| upstream_error(service, "fetch", None, None))?;
    if response.status_code() != 200 {
        let status = response.status_code();
        let mut code = None;
        if check_length(response.headers(), 4096).is_ok() {
            if let Ok(stream) = response.stream() {
                if let Ok(bytes) = read_stream(stream, 4096).await {
                    code = provider_error_code(&bytes);
                }
            }
        }
        return Err(upstream_error(service, "http_status", Some(status), code));
    }
    check_length(response.headers(), cap)
        .map_err(|_| upstream_error(service, "body_length", Some(200), None))?;
    let stream = response
        .stream()
        .map_err(|_| upstream_error(service, "body_stream", Some(200), None))?;
    read_stream(stream, cap)
        .await
        .map_err(|_| upstream_error(service, "body_read", Some(200), None))
}

#[cfg(test)]
mod tests {
    use super::provider_error_code;

    #[test]
    fn provider_diagnostics_only_allow_known_codes() {
        assert_eq!(
            provider_error_code(br#"{"error":{"type":"FreeTierError","message":"private"}}"#),
            Some("FreeTierError")
        );
        for body in [
            br#"{"error":{"type":"secret value","message":"private"}}"#.as_slice(),
            br#"{"error":"private"}"#,
            b"not json",
        ] {
            assert_eq!(provider_error_code(body), None);
        }
    }
}
