use std::fmt::Display;

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
