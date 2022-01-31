#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn operation(stack: &mut Vec<i32>, op: impl Fn(i32, i32) -> i32) -> Option<i32> {
    stack.pop().and_then(|b| stack.pop().map(|a| op(a, b)))
}

use std::ops::{Add, Div, Mul, Sub};

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    // for input in inputs {
    //     let a = stack.pop()?;
    //     let b = stack.pop()?;
    //     let op = stack.pop()?;
    //     return match op {
    //         CalculatorInput::Add => Some(i32::add(a, b)),
    //         CalculatorInput::Subtract => Some(i32::sub(b, a)),
    //         CalculatorInput::Multiply => Some(i32::mul(a, b)),
    //         CalculatorInput::Divide => Some(i32::div(b, a)),
    //         CalculatorInput::Value(n) => Some(n),
    //     }.map(|x| {
    //         stack.push(x);
    //         stack
    //     }).and_then(|stack| match stack.as_slice() {
    //         [x] => Some(*x),
    //         _ => None
    //         })
    for input in inputs {
        match input {
            CalculatorInput::Add => {
                let v1 = stack.pop()?;
                let v2 = stack.pop()?;
                stack.push(i32::add(v1, v2));
            },
            CalculatorInput::Subtract => {
                let v1 = stack.pop()?;
                let v2 = stack.pop()?;
                stack.push(i32::sub(v2, v1));
            },
            CalculatorInput::Multiply => {
                let v1 = stack.pop()?;
                let v2 = stack.pop()?;
                stack.push(i32::mul(v1, v2));
            },
            CalculatorInput::Divide => {
                let v1 = stack.pop()?;
                let v2 = stack.pop()?;
                stack.push(i32::div(v2, v1));
            },
            CalculatorInput::Value(v) => {
                stack.push(*v);
            }
        }
    }
    match stack.len() {
        1 => stack.pop(),
        _ => None
    }
}
