pub fn egg_count(display_value: u32) -> usize {
    if display_value == 0 { return 0 }
    (display_value % 2u32 + (egg_count(display_value / 2u32) as u32)) as usize
}