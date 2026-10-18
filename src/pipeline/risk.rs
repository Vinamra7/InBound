use crate::domain::edge::EdgeInfo;
use crate::domain::fingerprint::FpVerdict;
use crate::domain::risk::{RiskDecision, gate};
use crate::domain::token::{Q_NORMAL, Q_RESTRICTED};
use crate::error::AppError;

pub fn assert_allowed(edge: &EdgeInfo) -> Result<(), AppError> {
    if gate(edge) == RiskDecision::Block {
        Err(AppError::HumanCheckFailed)
    } else {
        Ok(())
    }
}

pub fn quota_tier(edge: &EdgeInfo, fp: FpVerdict) -> u8 {
    if fp == FpVerdict::Suspicious || gate(edge) == RiskDecision::Restricted {
        Q_RESTRICTED
    } else {
        Q_NORMAL
    }
}
