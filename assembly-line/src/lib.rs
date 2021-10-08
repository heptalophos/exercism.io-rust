const MAX_PRODUCTION_PER_HOUR: f64 = 221f64;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate: f64 = match speed {
        1..=4  => 1.0,
        5..=8  => 0.9,
        9 | 10 => 0.77,
        _      => 0.0
    };
    
    let true_speed: f64 = 
        speed as f64 * success_rate;
    
        MAX_PRODUCTION_PER_HOUR * true_speed
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60u32
}
