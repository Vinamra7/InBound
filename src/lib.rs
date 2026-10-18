use worker::{Context, Env, Method, Request, Response, event};

use crate::error::AppError;

pub mod adapters;
pub mod config;
pub mod constants;
pub mod domain;
pub mod error;
pub mod pipeline;
pub mod routes;

#[event(fetch)]
pub async fn main(req: Request, env: Env, ctx: Context) -> worker::Result<Response> {
    console_error_panic_hook::set_once();

    let origin = req.headers().get("Origin").ok().flatten();

    if req.method() == Method::Options {
        return Ok(routes::cors::preflight(origin));
    }

    // health answers even with missing secrets — config must not gate it
    if req.method() == Method::Get && req.path() == "/health" {
        return Ok(routes::cors::wrap(
            origin,
            routes::health::handle_health(req)
                .await
                .unwrap_or_else(|_| AppError::Internal.response()),
        ));
    }

    let config = match config::Config::from_env(&env) {
        Ok(c) => c,
        Err(_) => {
            return Ok(routes::cors::wrap(origin, AppError::Internal.response()));
        }
    };

    let resp = match (req.method(), req.path().as_str()) {
        (Method::Get, "/health") => routes::health::handle_health(req).await,
        (Method::Post, "/api/session") => {
            routes::session::handle_session(req, config.session, env.clone(), ctx).await
        }
        (Method::Post, "/api/chat") => routes::chat::handle_chat(req, config.chat, env, ctx).await,
        _ => Ok(AppError::NotFound.response()),
    };

    let resp = match resp {
        Ok(r) => r,
        Err(_) => AppError::Internal.response(),
    };

    Ok(routes::cors::wrap(origin, resp))
}
