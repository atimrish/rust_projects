use crate::common::primitives::ProgramError;

use super::primitives::{Brace, Number, Operator, Primitive};
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

pub fn display_primitives(primitives: &Vec<Primitive>) {
    primitives.iter().for_each(|item| {
        let string_view: String = match item {
            Primitive::Brace(brace) => match brace {
                Brace::Start => "BraceStart".to_string(),
                Brace::End => "BraceEnd".to_string(),
            },
            Primitive::Number(number) => match number {
                Number::Float(float) => format!("Float({float})"),
                Number::Integer(integer) => format!("Integer({integer})"),
            },
            Primitive::Operator(operator) => match operator {
                Operator::Addition => "Addition".to_string(),
                Operator::Subtraction => "Subtraction".to_string(),
                Operator::Multiplication => "Multiplication".to_string(),
                Operator::Division => "Division".to_string(),
                Operator::Exponentiation => "Exponentiation".to_string(),
            },
        };

        print!("{} ", string_view);
    });
    println!();
}

pub fn display_braces(braces: &Vec<Brace>) {
    eprintln!("Braces");

    braces.iter().for_each(|b| match b {
        Brace::Start => eprintln!("StartBrace"),
        Brace::End => eprintln!("EndBrace"),
    });
}
