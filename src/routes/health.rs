use worker::{Request, Response, Result};

pub async fn handle_health(req: Request) -> Result<Response> {
    if req.path() == "/health" {
        Response::from_json(&serde_json::json!({"ok": true}))
    } else {
        Response::error("Not Found", 404)
    }
}
