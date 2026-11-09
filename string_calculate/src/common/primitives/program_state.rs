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
