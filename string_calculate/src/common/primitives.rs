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

pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
}

pub enum Brace {
    Start,
    End,
}

pub enum Number {
    Integer(i32),
    Float(f32),
}

pub enum Primitive {
    Number(Number),
    Operator(Operator),
    Brace(Brace),
}
