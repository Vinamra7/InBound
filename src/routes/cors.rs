use worker::{Headers, Response};

use crate::constants::ALLOWED_ORIGINS;
use crate::error::AppError;

pub fn headers_for(origin: Option<&str>, allow_localhost: bool) -> Headers {
    let headers = Headers::new();
    if let Some(o) = origin {
        if ALLOWED_ORIGINS.contains(&o)
            || (allow_localhost && matches!(o, "http://localhost:3000" | "http://127.0.0.1:3000"))
        {
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

pub fn wrap(origin: Option<String>, mut resp: Response, allow_localhost: bool) -> Response {
    for (name, value) in headers_for(origin.as_deref(), allow_localhost).entries() {
        let _ = resp.headers_mut().set(&name, &value);
    }
    let _ = resp.headers_mut().set("Vary", "Origin");
    let _ = resp.headers_mut().set("Cache-Control", "no-store");
    resp
}

pub fn preflight(origin: Option<String>, allow_localhost: bool) -> Response {
    Response::empty()
        .unwrap_or_else(|_| AppError::Internal.response())
        .with_status(204)
        .with_headers(headers_for(origin.as_deref(), allow_localhost))
}
