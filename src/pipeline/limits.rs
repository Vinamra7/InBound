pub fn remaining_quota(limit: u32, used: u32) -> u32 {
    if used >= limit { 0 } else { limit - used }
}
