use serde::Deserialize;

use crate::error::AppError;
use worker::d1::{D1Database, D1PreparedStatement};
use worker::wasm_bindgen::JsValue;

pub struct Allow {
    pub allowed: bool,
    pub count: u32,
}

// index of each statement inside the batch (§5 order; inserts are 0 and 1, never read)
const IDX_UPDATE_USER: usize = 2;
const IDX_UPDATE_GLOBAL: usize = 3;
const IDX_SELECT_USER: usize = 4;
const IDX_SELECT_GLOBAL: usize = 5;

#[derive(Debug, Deserialize)]
struct CountRow {
    count: u32,
}

fn stmt(db: &D1Database, sql: &str, args: &[JsValue]) -> Result<D1PreparedStatement, AppError> {
    Ok(db.prepare(sql).bind(args)?)
}

pub async fn incr(
    db: &D1Database,
    day: &str,
    device_id: &str,
    user_limit: u32,
    global_limit: u32,
) -> Result<(Allow, Allow), AppError> {
    let sql_insert_user =
        "INSERT OR IGNORE INTO user_counters (day, device_id, count) VALUES (?1, ?2, 0)";
    let sql_insert_global = "INSERT OR IGNORE INTO global_counters (day, count) VALUES (?1, 0)";
    let sql_update_user = "UPDATE user_counters SET count = count + 1 WHERE day = ?1 AND device_id = ?2 AND count < ?3";
    // changes() = 1 → the user bump in [2] succeeded; a denied user must not burn a global slot.
    // Runs sequentially on one connection — verify in `wrangler dev`.
    let sql_update_global = "UPDATE global_counters SET count = count + 1 WHERE day = ?1 AND count < ?2 AND changes() = 1";
    let sql_select_user = "SELECT count FROM user_counters WHERE day = ?1 AND device_id = ?2";
    let sql_select_global = "SELECT count FROM global_counters WHERE day = ?1";

    let results = db
        .batch(vec![
            stmt(db, sql_insert_user, &[day.into(), device_id.into()])?,
            stmt(db, sql_insert_global, &[day.into()])?,
            stmt(
                db,
                sql_update_user,
                &[
                    day.into(),
                    device_id.into(),
                    JsValue::from_f64(user_limit as f64),
                ],
            )?,
            stmt(
                db,
                sql_update_global,
                &[day.into(), JsValue::from_f64(global_limit as f64)],
            )?,
            stmt(db, sql_select_user, &[day.into(), device_id.into()])?,
            stmt(db, sql_select_global, &[day.into()])?,
        ])
        .await?;

    let user_changes = results
        .get(IDX_UPDATE_USER)
        .ok_or(AppError::Internal)?
        .meta()?
        .ok_or(AppError::Internal)?
        .changes
        .unwrap_or(0);
    let global_changes = results
        .get(IDX_UPDATE_GLOBAL)
        .ok_or(AppError::Internal)?
        .meta()?
        .ok_or(AppError::Internal)?
        .changes
        .unwrap_or(0);

    let user_count = results
        .get(IDX_SELECT_USER)
        .ok_or(AppError::Internal)?
        .results::<CountRow>()?
        .first()
        .ok_or(AppError::Internal)?
        .count;
    let global_count = results
        .get(IDX_SELECT_GLOBAL)
        .ok_or(AppError::Internal)?
        .results::<CountRow>()?
        .first()
        .ok_or(AppError::Internal)?
        .count;

    Ok((
        Allow {
            allowed: user_changes > 0,
            count: user_count,
        },
        Allow {
            allowed: global_changes > 0,
            count: global_count,
        },
    ))
}
