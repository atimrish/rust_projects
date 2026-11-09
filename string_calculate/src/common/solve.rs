use std::collections::VecDeque;

use super::primitives::{Number, Operator, ProgramError};
use super::split_expression::{SplitExressionResult, split_expression};
use super::tokenize;
use super::unwrap_braces;

pub fn solve(expression: &str) -> Result<Number, String> {
    let primitives = tokenize(expression).map_err(|err| match err {
        ProgramError::BracesMismatch => "Braces mismatch".to_string(),
        ProgramError::NothingGiven => "Nothing given".to_string(),
        ProgramError::Parse => "Parse error".to_string(),
        ProgramError::UnexpectedCharacter(c) => format!("Unexpected character: {}", c),
    })?;

    let mut pending_stack = vec![&primitives[..]];
    let mut ready_stack: VecDeque<Number> = VecDeque::new();
    let mut ready_operators: Vec<&Operator> = Vec::new();

    loop {
        let Some(temp_expr) = pending_stack.pop() else {
            break;
        };

        let temp_expr = unwrap_braces(temp_expr);
        let temp_expr = split_expression(temp_expr)?;

        match temp_expr {
            SplitExressionResult::SubExpression(sub) => {
                ready_operators.push(sub.operator);

                pending_stack.push(sub.right);
                pending_stack.push(sub.left);
            }
            SplitExressionResult::Number(n) => ready_stack.push_back(*n),
        }
    }
    println!();

    ready_operators.iter().for_each(|p| {
        print!("{} ", p);
    });
    println!();

    for op in ready_operators.iter().rev() {
        let Some(left) = ready_stack.pop_front() else {
            break;
        };

        let Some(right) = ready_stack.pop_front() else {
            break;
        };

        match op {
            Operator::Addition => ready_stack.push_front(left.add(&right)),
            Operator::Subtraction => ready_stack.push_front(left.sub(&right)),
            Operator::Multiplication => ready_stack.push_front(left.mul(&right)),
            Operator::Division => ready_stack.push_front(left.div(&right)),
            Operator::Exponentiation => ready_stack.push_front(left.exp(&right)),
        }
    }

    Ok(Number::Integer(1))
}
