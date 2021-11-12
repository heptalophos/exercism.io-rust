use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond: i64 = 10i64.pow(9);
    start + Duration::seconds(gigasecond)
}
