use worker::{Headers, Response};

use crate::constants::ALLOWED_ORIGINS;
use crate::error::AppError;

pub fn headers_for(origin: Option<&str>) -> Headers {
    let headers = Headers::new();
    if let Some(o) = origin {
        if ALLOWED_ORIGINS.contains(&o) {
            let _ = headers.append("Access-Control-Allow-Origin", o);
            let _ = headers.append("Access-Control-Allow-Methods", "GET, POST, OPTIONS");
            let _ = headers.append(
                "Access-Control-Allow-Headers",
                "Content-Type, Authorization",
            );
            let _ = headers.append("Access-Control-Max-Age", "86400");
            let _ = headers.append("Vary", "Origin");
        }
    }
    headers
}

pub fn wrap(origin: Option<String>, resp: Response) -> Response {
    resp.with_headers(headers_for(origin.as_deref()))
}

pub fn preflight(origin: Option<String>) -> Response {
    Response::empty()
        .unwrap_or_else(|_| AppError::Internal.response())
        .with_status(204)
        .with_headers(headers_for(origin.as_deref()))
}
