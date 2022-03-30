pub fn map<A, F: FnMut(A) -> B, B>(input: Vec<A>, mut function: F) -> Vec<B> {
    let mut accumulator = vec![];
    for item in input {
        let mapping = function(item);
        accumulator.push(mapping);
    }
    accumulator
}