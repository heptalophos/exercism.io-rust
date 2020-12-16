pub fn factors(n: u64) -> Vec<u64> {
    
    let mut num = n;
    let mut pfactors = vec![];
    let mut d = 2;

    while num > 1 {
        if num % d != 0 {
            d += 1;
        } else {
            pfactors.push(d);
            num /= d;
        }
    }

    pfactors
}
