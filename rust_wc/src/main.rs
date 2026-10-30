use std::{env, process::ExitCode};

fn word_count(text: &str) -> usize {
    text
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .count()
}

fn print_usage_description() {
    println!("Word counter");
    println!("Usage:");
    println!("\t-h, --help print usage description");
    println!("\t-t, --text <text> text to count words");
}

fn run(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let first_arg = args.next().ok_or("No args given, use --help to show usage")?;

    match first_arg.as_str() {
        "-h" | "--help" => {
            print_usage_description();
            Ok(())
        }
        "-t" | "--text" => {
            let text = args.next().ok_or("No text given")?;
            println!("Words: {}", word_count(&text));
            Ok(())
        }
        other => Err(format!("Unknown argument: {other}"))
    }
}

fn main() -> ExitCode {
    match run(env::args().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
