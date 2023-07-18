#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const M: i32 = 26;
const A: i32 = b'a' as i32;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). 
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !coprime(a, M) {
        return Err (AffineCipherError::NotCoprime(a))
    }
    Ok (plaintext.to_ascii_lowercase().chars()
                 .filter(|c| c.is_alphanumeric())
                 .map(|c| if c.is_numeric() { c }
                          else { 
                            let e = ((c as i32 - A) * a + b) % M;
                            (e + A) as u8 as char 
                          }
                 )
                 .collect::<Vec<char>>()
                 .chunks(5)
                 .map(|s| s.iter().collect::<String>())
                 .collect::<Vec<String>>()
                 .join(" ")
    )
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`).
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !coprime(a, M) {
        return Err (AffineCipherError::NotCoprime(a))
    }
    Ok (ciphertext.chars()
                  .filter(|c| c.is_alphanumeric())
                  .map(|c|   if c.is_numeric() { c }
                             else { 
                                let x = mmi(a) * ((c as i32 - A) - b) % M;
                                let d = if x < 0 { x + M } else { x };
                                (d + A) as u8 as char 
                             }
                  )
                  .collect::<String>()
    )
}

/// Auxiliary 

fn coprime(x: i32, y: i32) -> bool {
    for i in 2..=x {
        if x % i == 0 && y % i == 0 { return false }
    }
    true
} 

fn mmi(x: i32) -> i32 {
    for i in 0..M {
        if (x * i) % M == 1 { return i }
    }
    0
}