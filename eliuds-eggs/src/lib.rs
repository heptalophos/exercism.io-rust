pub fn egg_count(display_value: u32) -> usize {
    let (mut pcount, mut number) = (0, display_value);
    let turn_off_rightmost_one = |n: u32| (n - 1) & n;
    while number > 0u32 {
        pcount += 1;
        number = turn_off_rightmost_one(number);
    }
    pcount
}