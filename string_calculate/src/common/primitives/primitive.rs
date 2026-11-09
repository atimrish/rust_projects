use super::Brace;
use super::Number;
use super::Operator;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Number(Number),
    Operator(Operator),
    Brace(Brace),
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
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

        write!(f, "{}", str)
    }
}
