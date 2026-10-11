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

    let config = match config::Config::from_env(&env) {
        Ok(c) => c,
        Err(_) => return Response::error("Internal Server Error", 500),
    };

    match (req.method(), req.path().as_str()) {
        (Method::Get, "/health") => routes::health::handle_health(req).await,
        (Method::Post, "/api/session") => {
            routes::session::handle_session(req, config.session, ctx).await
        }
        (Method::Post, "/api/chat") => routes::chat::handle_chat(req, config.chat, ctx).await,
        _ => Response::error("Not Found", 404),
    }
}
