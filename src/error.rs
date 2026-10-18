use serde_json::json;
use worker::Response;

use crate::config::ConfigError;
use crate::domain::message::MessageError;
use crate::domain::token::TokenError;

#[derive(Debug, PartialEq)]
pub enum AppError {
    BadRequest,       // shape / length / turn-cap — 400
    Unauthorized,     // token missing, malformed, tampered, expired — 401
    HumanCheckFailed, // Turnstile siteverify said no — 403
    RateLimited,      // sess/fine/net quota hit — 429
    BudgetExceeded,   // global daily cap — 503
    Upstream,         // siteverify / OpenRouter network failure — 502
    Internal,         // config bad, impossible states — 500
}

// ---------- impls ----------

impl AppError {
    pub fn status(&self) -> u16 {
        match self {
            AppError::BadRequest => 400,
            AppError::Unauthorized => 401,
            AppError::HumanCheckFailed => 403,
            AppError::RateLimited => 429,
            AppError::BudgetExceeded => 503,
            AppError::Upstream => 502,
            AppError::Internal => 500,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            AppError::BadRequest => "bad_request",
            AppError::Unauthorized => "unauthorized",
            AppError::HumanCheckFailed => "human_check_failed",
            AppError::RateLimited => "rate_limited",
            AppError::BudgetExceeded => "budget_exhausted",
            AppError::Upstream => "upstream",
            AppError::Internal => "internal",
        }
    }

    /// HTTP-shaped error. Body is {"error": code} only — never internals
    /// (§10: no which-limit / which-key leakage to the client).
    pub fn response(&self) -> Response {
        Response::from_json(&json!({"error": self.code()}))
            .unwrap_or_else(|_| {
                // fallback can only run if serde fails on a &'static str — i.e. never
                Response::error("Internal Server Error", 500).expect("fallback response")
            })
            .with_status(self.status())
    }
}

// ---------- conversions from the domain/adapter error families ----------

impl From<TokenError> for AppError {
    fn from(_: TokenError) -> Self {
        AppError::Unauthorized // malformed, bad signature, expired → all 401
    }
}

impl From<MessageError> for AppError {
    fn from(_: MessageError) -> Self {
        AppError::BadRequest
    }
}

impl From<ConfigError> for AppError {
    fn from(_: ConfigError) -> Self {
        AppError::Internal
    }
}

impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> Self {
        AppError::BadRequest // invalid JSON body
    }
}

impl From<worker::Error> for AppError {
    fn from(_: worker::Error) -> Self {
        AppError::Internal // generic worker failure; adapters map deliberate fetch failures to Upstream
    }
}
