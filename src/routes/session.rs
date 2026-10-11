use worker::{Context, Env, Request, Response, Result};

pub async fn handle_session(req: Request, env: Env, ctx: Context) -> Result<Response> {
    Response::from_json(&serde_json::json!({"ok":true}))
}
