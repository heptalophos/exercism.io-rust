#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32)
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> 
                                            Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    } 
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    } 
    if let Some(x) = number.iter()
                    .find(|&&d| d >= from_base)
                    .cloned() {
                        return Err(Error::InvalidDigit(x));
                    }

    let mut from_form = number.iter()
                        .fold(0, |acc, d| acc * from_base + *d);

    if from_form == 0 { return Ok(vec![0]); }
    
    let mut to_base_vector = vec![];
    
    while from_form > 0 {
        to_base_vector.push(from_form % to_base);
        from_form /= to_base as u32;
    }

    to_base_vector.reverse();
    
    Ok(to_base_vector)

}
