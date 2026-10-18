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

pub async fn fetch_body(req: Request, timeout_ms: u32, cap: usize) -> Result<Vec<u8>, AppError> {
    let signal: AbortSignal = worker::web_sys::AbortSignal::timeout_with_u32(timeout_ms).into();
    let mut response: Response = Fetch::Request(req)
        .send_with_signal(&signal)
        .await
        .map_err(|_| AppError::Upstream)?;
    if response.status_code() != 200 {
        return Err(AppError::Upstream);
    }
    check_length(response.headers(), cap).map_err(|_| AppError::Upstream)?;
    read_stream(response.stream().map_err(|_| AppError::Upstream)?, cap)
        .await
        .map_err(|_| AppError::Upstream)
}
