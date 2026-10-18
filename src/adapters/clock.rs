use worker::Date;

pub fn now_secs() -> u64 {
    Date::now().as_millis() / 1000
}

// UTC calendar day as "YYYY-MM-DD" — the D1 counter key (§3: day buckets are UTC).
// Pure integer math (Hinnant's civil-from-days) — wasm-safe, no chrono.
pub fn utc_day(now_secs: u64) -> String {
    let days = now_secs / 86_400;
    let z = days as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    use super::utc_day;

    #[test]
    fn epoch_is_1970_01_01() {
        assert_eq!(utc_day(0), "1970-01-01");
    }

    #[test]
    fn known_anchor_2026_01_01() {
        assert_eq!(utc_day(1_767_225_600), "2026-01-01");
    }

    #[test]
    fn day_boundary_rolls_at_utc_midnight() {
        // 2026-01-01 ends at 1767311999; next second is 2026-01-02
        assert_eq!(utc_day(1_767_311_999), "2026-01-01");
        assert_eq!(utc_day(1_767_312_000), "2026-01-02");
    }
}
