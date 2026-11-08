use super::primitives::Operator;
use super::primitives::ProgramError;
use std::str::FromStr;

pub fn parse_slice<T: FromStr>(
    expression: &str,
    start: &usize,
    end: &usize,
) -> Result<T, ProgramError> {
    let slice = &expression[*start..*end + 1];
    slice.parse::<T>().map_err(|_| ProgramError::Parse)
}

pub fn is_math_operator(c: &char) -> Option<Operator> {
    match c {
        '+' => Some(Operator::Addition),
        '-' => Some(Operator::Subtraction),
        '*' => Some(Operator::Multiplication),
        '/' => Some(Operator::Division),
        '^' => Some(Operator::Exponentiation),
        _ => None,
    }
}
