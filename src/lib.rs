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
    let allow_localhost = config::allow_localhost(&env);

    if req.method() == Method::Options {
        return Ok(routes::cors::preflight(origin, allow_localhost));
    }

    // health answers even with missing secrets — config must not gate it
    if req.method() == Method::Get && req.path() == "/health" {
        return Ok(routes::cors::wrap(
            origin,
            routes::health::handle_health(req)
                .await
                .unwrap_or_else(|_| AppError::Internal.response()),
            allow_localhost,
        ));
    }

    if req.method() != Method::Post || !matches!(req.path().as_str(), "/api/session" | "/api/chat")
    {
        return Ok(routes::cors::wrap(
            origin,
            AppError::NotFound.response(),
            allow_localhost,
        ));
    }

    let config = match config::Config::from_env(&env) {
        Ok(c) => c,
        Err(_) => {
            worker::console_error!("configuration unavailable");
            return Ok(routes::cors::wrap(
                origin,
                AppError::Internal.response(),
                allow_localhost,
            ));
        }
    };

    let resp = match (req.method(), req.path().as_str()) {
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

    Ok(routes::cors::wrap(origin, resp, allow_localhost))
}
