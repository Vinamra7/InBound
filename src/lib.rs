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
pub async fn main(req: Request, _env: Env, _ctx: Context) -> worker::Result<Response> {
    if matches!(req.method(), Method::Get) && req.path() == "/health" {
        return routes::health::handle_health(req).await;
    }
    Response::error("Not Found", 404)
}
