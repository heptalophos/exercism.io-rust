/// What should the type of _function be?
pub fn map<F: Fn(i32) -> i32>(input: Vec<i32>, function: F) -> Vec<i32> {
    input.iter().map(|x| function(*x)).collect()
}

