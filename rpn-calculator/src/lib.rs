#[allow(unreachable_patterns)]
#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use std::ops::{Add, Div, Mul, Sub};

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    let mut error = false; 
    for input in inputs {
        match input {
            CalculatorInput::Add => {
                let (operand2, operand1) = (stack.pop()?, stack.pop()?);
                stack.push(i32::add(operand1, operand2));
            },
            CalculatorInput::Subtract => {
                let (operand2, operand1) = (stack.pop()?, stack.pop()?);
                stack.push(i32::sub(operand1, operand2));
            },
            CalculatorInput::Multiply => {
                let (operand2, operand1) = (stack.pop()?, stack.pop()?);
                stack.push(i32::mul(operand1, operand2));
            },
            CalculatorInput::Divide => {
                let (operand2, operand1) = (stack.pop()?, stack.pop()?);
                stack.push(i32::div(operand1, operand2));
            },
            CalculatorInput::Value(value) => {
                stack.push(*value);
            },
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
