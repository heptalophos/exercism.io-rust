pub fn collatz(n: u64) -> Option<u64> {
    if n <= 0 { return None }
    let mut steps = 0u64;
    let mut n = n;
    while n > 1 {
        match n % 2 == 0 {
            true => n /= 2,
            false => { n *= 3; n += 1 } 
        }
        steps += 1
    } 
    Some(steps)
}
