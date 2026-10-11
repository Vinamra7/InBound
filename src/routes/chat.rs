use crate::config::ChatConfig;
use worker::{Context, Request, Response, Result};

pub async fn handle_chat(_req: Request, _config: ChatConfig, _ctx: Context) -> Result<Response> {
    Response::from_json(&serde_json::json!({"ok": true}))
}
