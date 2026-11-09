#[derive(Debug, PartialEq)]
pub enum ProgramError {
    UnexpectedCharacter(char),
    BracesMismatch,
    NothingGiven,
    Parse,
}
