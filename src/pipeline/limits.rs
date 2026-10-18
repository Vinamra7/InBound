use worker::d1::D1Database;

use crate::adapters::counter;
use crate::config::{GLOBAL_QUOTA, USER_QUOTA_NORMAL, USER_QUOTA_RESTRICTED};
use crate::domain::token::Q_RESTRICTED;
use crate::error::AppError;

pub fn user_limit_for(q: u8) -> u32 {
    if q == Q_RESTRICTED {
        USER_QUOTA_RESTRICTED
    } else {
        USER_QUOTA_NORMAL
    }
}

// ONE atomic D1 batch, before the classifier and the LLM (§3: count attempts, no refunds)
pub async fn enforce(db: &D1Database, day: &str, device_id: &str, q: u8) -> Result<(), AppError> {
    let (user, global) = counter::incr(db, day, device_id, user_limit_for(q), GLOBAL_QUOTA).await?;

    if !user.allowed {
        return Err(AppError::RateLimited);
    }
    if !global.allowed {
        return Err(AppError::BudgetExceeded);
    }

    Ok(())
}
