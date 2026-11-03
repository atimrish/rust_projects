mod common;
use common::primitives::ProgramError;
use common::tokenize::tokenize;
use std::process::ExitCode;

use crate::common::solve::solve;

fn main() -> ExitCode {
    let expression = "1 + 2 * 3";

    let primitives = match tokenize(expression) {
        Ok(value) => value,
        Err(ProgramError::BracesMismatch) => {
            eprintln!("Braces mismatch");
            return ExitCode::FAILURE;
        }
        Err(ProgramError::NothingGiven) => {
            eprintln!("Nothing given");
            return ExitCode::FAILURE;
        }
        Err(ProgramError::Parse) => {
            eprintln!("Number parse error");
            return ExitCode::FAILURE;
        }
        Err(ProgramError::UnexpectedCharacter(c)) => {
            eprintln!("Unexpected character {c}");
            return ExitCode::FAILURE;
        }
    };

    solve(&primitives);

    ExitCode::SUCCESS
}
