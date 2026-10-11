use crate::config::SessionConfig;
use worker::{Context, Request, Response, Result};

pub async fn handle_session(
    _req: Request,
    _config: SessionConfig,
    _ctx: Context,
) -> Result<Response> {
    Response::from_json(&serde_json::json!({"ok": true}))
}
