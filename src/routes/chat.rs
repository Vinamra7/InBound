use worker::{Context, Env, Request, Response, Result};

use crate::adapters::{clock, edge, llm};
use crate::config::ChatConfig;
use crate::constants::{CANNED_REPLY, SYSTEM_PROMPT};
use crate::domain::edge::EdgeInfo;
use crate::domain::message::{ChatRequestDto, ChatResponseDto};
use crate::error::AppError;
use crate::pipeline::{auth, limits, risk, validate};

// up to 21 messages with per-role caps can exceed 30 KB — 8 KB would 400 valid history
const BODY_CAP_BYTES: usize = 40 * 1024;

pub async fn handle_chat(
    mut req: Request,
    config: ChatConfig,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let edge_info = edge::extract(&req);
    let outcome = chat_flow(&mut req, &config, &env, &edge_info).await;
    let resp = match outcome {
        Ok(r) => r,
        Err(e) => e.response(),
    };
    Ok(resp)
}

async fn chat_flow(
    req: &mut Request,
    config: &ChatConfig,
    env: &Env,
    edge_info: &EdgeInfo,
) -> std::result::Result<Response, AppError> {
    let bytes = req.bytes().await.map_err(|_| AppError::BadRequest)?;
    if bytes.len() > BODY_CAP_BYTES {
        return Err(AppError::BadRequest);
    }
    let dto: ChatRequestDto = serde_json::from_slice(&bytes).map_err(AppError::from)?;

    validate::validate_chat_request(&dto.messages)?;
    risk::assert_allowed(edge_info)?;

    let now = clock::now_secs();
    let raw = auth::authenticate(req, &config.hmac_key, now)?;

    let db = env.d1("DB").map_err(|_| AppError::Internal)?;
    limits::enforce(&db, &clock::utc_day(now), &raw.device_id, raw.q).await?;

    let current = dto
        .messages
        .last()
        .ok_or(AppError::Internal)?
        .content
        .clone();
    let in_scope = llm::classify(&config.llm_api_key, &current).await?;

    if !in_scope {
        return Response::from_json(&ChatResponseDto {
            reply: CANNED_REPLY.to_string(),
            usage: None,
        })
        .map_err(|_| AppError::Internal);
    }

    let (reply, usage) = llm::complete(&config.llm_api_key, SYSTEM_PROMPT, &dto.messages).await?;

    Response::from_json(&ChatResponseDto { reply, usage }).map_err(|_| AppError::Internal)
}
