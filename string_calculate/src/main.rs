mod common;
use common::tokenize::tokenize;
use common::{primitives::ProgramError, utils::display_primitives};
use std::process::ExitCode;

fn main() -> ExitCode {
    let expression = "1 + 1";

    match tokenize(expression) {
        Ok(value) => {
            display_primitives(&value);
            ExitCode::SUCCESS
        }
        Err(ProgramError::BracesMismatch) => {
            eprintln!("Braces mismatch");
            ExitCode::FAILURE
        }
        Err(ProgramError::NothingGiven) => {
            eprintln!("Nothing given");
            ExitCode::FAILURE
        }
        Err(ProgramError::Parse) => {
            eprintln!("Number parse error");
            ExitCode::FAILURE
        }
        Err(ProgramError::UnexpectedCharacter(c)) => {
            eprintln!("Unexpected character {c}");
            ExitCode::FAILURE
        }
    }
}
