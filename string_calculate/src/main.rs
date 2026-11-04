mod common;
use common::primitives::ProgramError;
use common::tokenize::tokenize;
use std::process::ExitCode;

use crate::common::solve::solve;
use crate::common::utils::display_primitives;

fn main() -> ExitCode {
    let expression = "((1 + 2) * 3) + 4 + 5 / 9 ^ 2 ^ (3 - 1)";

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

    // display_primitives(&primitives);
    solve(&primitives);

    ExitCode::SUCCESS
}
