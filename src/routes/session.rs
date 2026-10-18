use worker::{Context, Env, Request, Response, Result};

use crate::adapters::{clock, edge, turnstile};
use crate::config::SessionConfig;
use crate::domain::edge::EdgeInfo;
use crate::domain::fingerprint::validate_fingerprint;
use crate::domain::token::{SessionRequestDto, SessionResponseDto, TokenRaw, sign};
use crate::error::AppError;
use crate::pipeline::risk;

const BODY_CAP_BYTES: usize = 8192;

pub async fn handle_session(
    mut req: Request,
    config: SessionConfig,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let edge_info = edge::extract(&req);
    let outcome = session_flow(&mut req, &config, &env, &edge_info).await;
    let resp = match outcome {
        Ok(r) => r,
        Err(e) => e.response(),
    };
    Ok(resp)
}

async fn session_flow(
    req: &mut Request,
    config: &SessionConfig,
    env: &Env,
    edge_info: &EdgeInfo,
) -> std::result::Result<Response, AppError> {
    risk::assert_allowed(edge_info)?;

    let limiter = env
        .rate_limiter("SESSION_LIMITER")
        .map_err(|_| AppError::Internal)?;
    let outcome = limiter
        .limit(edge_info.ip.clone())
        .await
        .map_err(|_| AppError::Internal)?;
    if !outcome.success {
        return Err(AppError::RateLimited);
    }

    let bytes = req.bytes().await.map_err(|_| AppError::BadRequest)?;
    if bytes.len() > BODY_CAP_BYTES {
        return Err(AppError::BadRequest);
    }
    let payload: SessionRequestDto = serde_json::from_slice(&bytes).map_err(AppError::from)?;

    let verified =
        turnstile::verify(&config.turnstile_secret, &payload.turnstile, &edge_info.ip).await?;
    if !verified {
        return Err(AppError::HumanCheckFailed);
    }

    let verdict = validate_fingerprint(&payload.fingerprint, edge_info);
    let q = risk::quota_tier(edge_info, verdict);

    let now = clock::now_secs();
    let raw = TokenRaw::new(payload.fingerprint.device_id.clone(), q, now);
    let token = sign(&raw, &config.hmac_key).map_err(AppError::from)?;

    Response::from_json(&SessionResponseDto {
        token,
        expires_at: raw.exp,
    })
    .map_err(|_| AppError::Internal)
}
