use chrono::{Duration, NaiveTime};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: NaiveTime
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: NaiveTime::from_hms(0, 0, 0) + 
                     Duration::hours(hours as i64) + 
                     Duration::minutes(minutes as i64)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock { minutes: self.minutes + Duration::minutes(minutes as i64) }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.minutes.format("%H:%M").to_string())
    }
}