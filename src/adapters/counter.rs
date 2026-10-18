use serde::Deserialize;

use crate::error::AppError;
use worker::d1::{D1Database, D1PreparedStatement};

pub struct Allow {
    pub allowed: bool,
    pub count: u32,
}

const IDX_UPDATE_USER: usize = 2;
const IDX_UPDATE_GLOBAL: usize = 3;
const IDX_SELECT_USER: usize = 4;
const IDX_SELECT_GLOBAL: usize = 5;

#[derive(Debug, Deserialize)]
struct CountRow {
    count: u32,
}

//stmt statement
fn stmt(db: &D1Database, sql: &str, args: &[&str]) -> Result<D1PreparedStatement, AppError> {
    let js_args = args
        .iter()
        .map(|a| worker::wasm_bindgen::JsValue::from(*a))
        .collect::<Vec<_>>();
    Ok(db.prepare(sql).bind(&js_args)?)
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
            stmt(db, sql_insert_user, &[day, device_id])?,
            stmt(db, sql_insert_global, &[day])?,
            stmt(
                db,
                sql_update_user,
                &[day, device_id, user_limit.to_string().as_str()],
            )?,
            stmt(
                db,
                sql_update_global,
                &[day, global_limit.to_string().as_str()],
            )?,
            stmt(db, sql_select_user, &[day, device_id])?,
            stmt(db, sql_select_global, &[day])?,
        ])
        .await?;

    let user_changes = results[IDX_UPDATE_USER]
        .meta()?
        .ok_or(AppError::Internal)?
        .changes
        .unwrap_or(0);
    let global_changes = results[IDX_UPDATE_GLOBAL]
        .meta()?
        .ok_or(AppError::Internal)?
        .changes
        .unwrap_or(0);

    let user_count = results[IDX_SELECT_USER]
        .results::<CountRow>()?
        .first()
        .ok_or(AppError::Internal)?
        .count;
    let global_count = results[IDX_SELECT_GLOBAL]
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
