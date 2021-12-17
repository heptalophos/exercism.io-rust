pub fn collatz(n: u64) -> Option<u64> {
    if n <= 0 { return None }
    let mut steps = 0u64;
    let mut n = n;
    while n > 1 {
        match n % 2 == 0 {
            true  => n /= 2,
            false => { 
                n = n.checked_mul(3)?;
                n = n.checked_add(1)?
            }
        }
        steps += 1
    } 
    Some(steps)
}
