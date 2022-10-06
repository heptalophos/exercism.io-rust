pub fn nth(n: u32) -> u32 {
    if n < 1 { return 2 }
    (2..)
    .filter(|x| is_prime(&x)).nth(n as usize).unwrap()
}

fn is_prime(n: &u32) -> bool {
    let sq_root_n = (*n as f32).sqrt() as u32;
    (2..sq_root_n + 1u32).all(|f| n % f != 0)
}
