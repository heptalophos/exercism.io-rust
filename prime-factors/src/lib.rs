pub fn factors(n: u64) -> Vec<u64> {
    
    let mut num = n;
    let mut prime_factors = vec![];
    let mut candidate = 2;

    while num >= 2 {
        if num % candidate != 0 {
            candidate += 1;
        } else {
            prime_factors.push(candidate);
            num /= candidate;
        }
    }
    prime_factors
}
