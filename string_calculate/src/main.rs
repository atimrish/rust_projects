mod common;
use common::solve::solve;
use std::process::ExitCode;

fn main() -> ExitCode {
    // let expression = "((1 + 2) * 3) + 4 + 5 / 9 ^ 2 ^ (3 - 1)";
    let expression = "1+2-3";
    match solve(expression) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
