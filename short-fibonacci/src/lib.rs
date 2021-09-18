/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    create_buffer(0)
}

/// Create a buffer of `count` zeroes.
///
/// /// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut five_fibs = create_empty();
    let mut i = 0;
    while i < 2 {
        five_fibs.push(1);
        i += 1;
    }
    while i < 5 {
        five_fibs.push(five_fibs[i - 1] + five_fibs[i - 2]);
        i += 1;
    }
    five_fibs
}
