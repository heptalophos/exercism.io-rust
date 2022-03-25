/// What should the type of _function be?
pub fn map<F: Fn(i32) -> i32>(input: Vec<i32>, function: F) -> Vec<i32> {
    input.iter().map(|x| function(*x)).collect()
}


pub fn map<F: FnMut(A) -> B, A, B>(input: Vec<A>, function: F) -> Vec<B> {
    input.into_iter()
         .map(function)
         .collect()
}