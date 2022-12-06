const M: i32 = 26;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    unimplemented!("Encode {} with the key ({}, {})", plaintext, a, b);
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    unimplemented!("Decode {} with the key ({}, {})", ciphertext, a, b);
}

fn coprime(x: i32, y: i32) -> bool {
    for i in 2..=x {
        if x % i == 0 && y % i == 0 {
            return false;
        }
    }
    true
} 

fn mmi(x: i32) -> i32 {
    for i in 0..M {
        if (a * i) % M == 1 {
            return i;
        }
    }
    0;
}