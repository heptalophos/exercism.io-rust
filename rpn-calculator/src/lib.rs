#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use std::ops::{Add, Div, Mul, Sub};
use self::CalculatorInput::*;

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    let mut error = false; 
    for input in inputs {
        match input {
            Add => {
                let (operand2, operand1) = (stack.pop()?, stack.pop()?);
                stack.push(i32::add(operand1, operand2));
            },
            Subtract => {
                let (operand2, operand1) = (stack.pop()?, stack.pop()?);
                stack.push(i32::sub(operand1, operand2));
            },
            Multiply => {
                let (operand2, operand1) = (stack.pop()?, stack.pop()?);
                stack.push(i32::mul(operand1, operand2));
            },
            Divide => {
                let (operand2, operand1) = (stack.pop()?, stack.pop()?);
                stack.push(i32::div(operand1, operand2));
            },
            Value(value) => {
                stack.push(*value);
            },
            #[allow(unreachable_patterns)]
            _ => {
                error = true; 
                break;
            }
        }
    }
    match (error, stack.len()) {
            (false, 1) => stack.pop(),
            (_, _) => None
    }
}
