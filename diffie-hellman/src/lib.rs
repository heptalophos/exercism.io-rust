use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modexp(g as u128, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modexp(b_pub as u128, a, p)
}

fn modexp(base: u128, exp: u64, modulus: u64) -> u64 {
    let mut b = base;
    let mut e = exp;
    let mut p = 1;
    while e > 0 {
        if e % 2 == 1 {
            p = (b * p) % modulus as u128;
        }
        b = (b * b) % modulus as u128;
        e >>= 1;
    }
    p as u64
}