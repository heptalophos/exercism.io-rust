use chrono::{DateTime, Utc};
use chrono::Duration;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    
    let gigasecond: i64 = 10i64.pow(9);
    
    return start + Duration::seconds(gigasecond)
}
