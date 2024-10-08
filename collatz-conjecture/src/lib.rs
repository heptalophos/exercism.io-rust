pub fn collatz(n: u64) -> Option<u64> {
    if n <= 0 { return None }
    let mut steps = 0u64;
    let mut n:u64 = n;
    while n > 1 {
        match n % 2 == 0 {
            true  => n >>= 1,
            false => n = n.checked_mul(3)?.checked_add(1)?
        }
        steps += 1
    } 
    Some(steps)
}
