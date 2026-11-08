use std::fmt::Display;

#[derive(Eq, PartialEq)]
pub enum ProgramState {
    Started,
    ProcessingInteger,
    ProcessingFloat,
    FinishedNumber,
    WaitingAfterNegation,
    WaitingAfterFloatPoint,
    WaitingAfterOpenBrace,
    WaitingAfterCloseBrace,
    WaitingAfterOperator,
}

pub enum ProgramError {
    UnexpectedCharacter(char),
    BracesMismatch,
    NothingGiven,
    Parse,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
}

impl Operator {
    pub fn get_order(&self) -> u8 {
        match self {
            Self::Addition | Self::Subtraction => 0,
            Self::Multiplication | Self::Division => 1,
            Self::Exponentiation => 2,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Addition => write!(f, "Addition"),
            Operator::Subtraction => write!(f, "Subtraction"),
            Operator::Multiplication => write!(f, "Multiplication"),
            Operator::Division => write!(f, "Division"),
            Operator::Exponentiation => write!(f, "Exponentiation"),
        }
    }
}

#[derive(Debug)]
pub enum Brace {
    Start,
    End,
}

#[derive(Clone, Copy, Debug)]
pub enum Number {
    Integer(i32),
    Float(f32),
}

impl Number {
    pub fn add(&self, num: &Number) -> Number {
        match self {
            Number::Float(left) => match num {
                Number::Float(right) => Number::Float(left + right),
                Number::Integer(right) => Number::Float(left + *right as f32),
            },
            Number::Integer(left) => match num {
                Number::Float(right) => Number::Float(*left as f32 + right),
                Number::Integer(right) => Number::Integer(left + right),
            },
        }
    }

    pub fn sub(&self, num: &Number) -> Number {
        match self {
            Number::Float(left) => match num {
                Number::Float(right) => Number::Float(left - right),
                Number::Integer(right) => Number::Float(left - *right as f32),
            },
            Number::Integer(left) => match num {
                Number::Float(right) => Number::Float(*left as f32 - right),
                Number::Integer(right) => Number::Integer(left - right),
            },
        }
    }

    pub fn mul(&self, num: &Number) -> Number {
        match self {
            Number::Float(left) => match num {
                Number::Float(right) => Number::Float(left * right),
                Number::Integer(right) => Number::Float(left * *right as f32),
            },
            Number::Integer(left) => match num {
                Number::Float(right) => Number::Float(*left as f32 * right),
                Number::Integer(right) => Number::Integer(left * right),
            },
        }
    }

    pub fn div(&self, num: &Number) -> Number {
        match self {
            Number::Float(left) => match num {
                Number::Float(right) => Number::Float(left / right),
                Number::Integer(right) => Number::Float(left / *right as f32),
            },
            Number::Integer(left) => match num {
                Number::Float(right) => Number::Float(*left as f32 / right),
                Number::Integer(right) => Number::Integer(left / right),
            },
        }
    }

    pub fn exp(&self, num: &Number) -> Number {
        match self {
            Number::Float(left) => match num {
                Number::Float(right) => Number::Float(left.powf(*right)),
                Number::Integer(right) => Number::Float(left.powi(*right)),
            },
            Number::Integer(left) => match num {
                Number::Float(right) => Number::Float((*left as f32).powf(*right)),
                Number::Integer(right) => Number::Float((*left as f32).powi(*right)),
            },
        }
    }
}

#[derive(Debug)]
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
