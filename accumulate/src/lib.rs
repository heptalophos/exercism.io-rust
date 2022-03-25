pub fn map<F: FnMut(A) -> B, A, B>(input: Vec<A>, function: F) -> Vec<B> {
    input.into_iter()
         .map(function)
         .collect()
}