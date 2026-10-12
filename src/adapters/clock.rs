use worker::Date;

pub fn now_secs() -> u64 {
    Date::now().as_millis() / 1000
}
