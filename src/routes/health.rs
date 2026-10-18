use worker::{Request, Response, Result};

pub async fn handle_health(_req: Request) -> Result<Response> {
    Response::from_json(&serde_json::json!({"ok": true}))
}
