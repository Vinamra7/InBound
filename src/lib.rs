use worker::{Context, Env, Method, Request, Response, event};

pub mod adapters;
pub mod config;
pub mod domain;
pub mod durable;
pub mod error;
pub mod pipeline;
pub mod ports;
pub mod routes;

#[event(fetch)]
pub async fn main(req: Request, env: Env, ctx: Context) -> worker::Result<Response> {
    console_error_panic_hook::set_once();
    match (req.method(), req.path().as_str()) {
        (Method::Get, "/health") => routes::health::handle_health(req).await,
        (Method::Post, "/api/session") => routes::session::handle_session(req, env, ctx).await,
        (Method::Post, "/api/chat") => routes::chat::handle_chat(req, env, ctx).await,
        _ => Response::error("Not Found", 404),
    }
}
