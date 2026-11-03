use std::{process::ExitCode, str::FromStr};

#[derive(Eq, PartialEq)]
enum ProgramState {
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

enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
}

enum Brace {
    Start,
    End,
}

enum Number {
    Integer(i32),
    Float(f32),
}

enum Primitive {
    Number(Number),
    Operator(Operator),
    Brace(Brace),
}

fn parse_slice<T: FromStr>(expression: &str, start: &usize, end: &usize) -> Result<T, String> {
    let slice = &expression[*start..*end + 1];
    slice
        .parse::<T>()
        .map_err(|_| format!("Error parse number: {slice}"))
}

fn is_math_operator(c: &char) -> Option<Operator> {
    match c {
        '+' => Some(Operator::Addition),
        '-' => Some(Operator::Subtraction),
        '*' => Some(Operator::Multiplication),
        '/' => Some(Operator::Division),
        '^' => Some(Operator::Exponentiation),
        _ => None,
    }
}

fn display_primitives(primitives: &Vec<Primitive>) {
    primitives.iter().for_each(|item| {
        let string_view: String = match item {
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

        print!("{} ", string_view);
    });
    println!();
}

fn display_braces(braces: &Vec<Brace>) {
    eprintln!("Braces");

    braces.iter().for_each(|b| match b {
        Brace::Start => eprintln!("StartBrace"),
        Brace::End => eprintln!("EndBrace"),
    });
}

//TODO: Нужно посимвольно пробежаться по строке и распарсить ее
fn calculate(expression: &str) -> Result<Number, String> {
    let trimmed = expression.trim();
    let mut primitives: Vec<Primitive> = Vec::new();
    let mut braces_stack: Vec<Brace> = Vec::new();
    let mut program_state: ProgramState = ProgramState::Started;
    let mut parse_start_index: usize = 0;
    let mut parse_end_index: usize = 0;

    for (i, char) in trimmed.chars().enumerate() {
        match program_state {
            ProgramState::Started => {
                if char == '-' {
                    parse_start_index = i;
                    program_state = ProgramState::WaitingAfterNegation;
                    continue;
                }

                if char.is_ascii_digit() {
                    parse_start_index = i;
                    parse_end_index = i;
                    program_state = ProgramState::ProcessingInteger;
                    continue;
                }

                if char == '(' {
                    braces_stack.push(Brace::Start);
                    primitives.push(Primitive::Brace(Brace::Start));
                    program_state = ProgramState::WaitingAfterOpenBrace;
                    continue;
                }
            }
            ProgramState::ProcessingInteger => {
                if char.is_ascii_digit() {
                    parse_end_index = i;
                    continue;
                }

                if char == '.' {
                    program_state = ProgramState::WaitingAfterFloatPoint;
                    continue;
                }

                if char.is_ascii_whitespace() {
                    program_state = ProgramState::FinishedNumber;
                    let integer = parse_slice(trimmed, &parse_start_index, &parse_end_index)?;
                    primitives.push(Primitive::Number(Number::Integer(integer)));
                    continue;
                }

                if char == ')' {
                    program_state = ProgramState::WaitingAfterCloseBrace;
                    braces_stack
                        .pop()
                        .ok_or(format!("Braces mismatch at {i}"))?;

                    let integer = parse_slice(trimmed, &parse_start_index, &parse_end_index)?;
                    primitives.push(Primitive::Number(Number::Integer(integer)));
                    primitives.push(Primitive::Brace(Brace::End));
                    continue;
                }

                match is_math_operator(&char) {
                    Some(operator) => {
                        program_state = ProgramState::WaitingAfterOperator;

                        let integer = parse_slice(trimmed, &parse_start_index, &parse_end_index)?;
                        primitives.push(Primitive::Number(Number::Integer(integer)));
                        primitives.push(Primitive::Operator(operator));
                        continue;
                    }
                    None => (),
                }
            }
            ProgramState::ProcessingFloat => {
                if char.is_ascii_digit() {
                    parse_end_index = i;
                    continue;
                }

                if char.is_ascii_whitespace() {
                    program_state = ProgramState::FinishedNumber;

                    let float = parse_slice(trimmed, &parse_start_index, &parse_end_index)?;
                    primitives.push(Primitive::Number(Number::Float(float)));
                    continue;
                }

                if char == ')' {
                    program_state = ProgramState::WaitingAfterCloseBrace;
                    braces_stack
                        .pop()
                        .ok_or(format!("Braces mismatch at {i}"))?;

                    let float = parse_slice(trimmed, &parse_start_index, &parse_end_index)?;
                    primitives.push(Primitive::Number(Number::Float(float)));
                    primitives.push(Primitive::Brace(Brace::End));
                    continue;
                }

                match is_math_operator(&char) {
                    Some(operator) => {
                        program_state = ProgramState::WaitingAfterOperator;

                        let float = parse_slice(trimmed, &parse_start_index, &parse_end_index)?;
                        primitives.push(Primitive::Number(Number::Float(float)));
                        primitives.push(Primitive::Operator(operator));
                        continue;
                    }
                    None => (),
                }
            }
            ProgramState::FinishedNumber => {
                if char.is_ascii_whitespace() {
                    continue;
                }

                if char == ')' {
                    program_state = ProgramState::WaitingAfterCloseBrace;
                    braces_stack
                        .pop()
                        .ok_or(format!("Braces mismatch at {i}"))?;
                    primitives.push(Primitive::Brace(Brace::End));
                    continue;
                }

                match is_math_operator(&char) {
                    Some(operator) => {
                        program_state = ProgramState::WaitingAfterOperator;
                        primitives.push(Primitive::Operator(operator));
                        continue;
                    }
                    None => (),
                }
            }
            ProgramState::WaitingAfterFloatPoint => {
                if char.is_ascii_digit() {
                    program_state = ProgramState::ProcessingFloat;
                    parse_end_index = i;
                    continue;
                }
            }
            ProgramState::WaitingAfterNegation => {
                if char.is_ascii_digit() {
                    program_state = ProgramState::ProcessingInteger;
                    parse_end_index = i;
                    continue;
                }
            }
            ProgramState::WaitingAfterOpenBrace => {
                if char.is_ascii_digit() {
                    program_state = ProgramState::ProcessingInteger;
                    parse_start_index = i;
                    parse_end_index = i;
                    continue;
                }

                if char == '-' {
                    program_state = ProgramState::WaitingAfterNegation;
                    parse_start_index = i;
                    continue;
                }

                if char.is_whitespace() {
                    continue;
                }

                if char == '(' {
                    braces_stack.push(Brace::Start);
                    primitives.push(Primitive::Brace(Brace::Start));
                    continue;
                }
            }
            ProgramState::WaitingAfterOperator => {
                if char.is_whitespace() {
                    continue;
                }

                if char == '-' {
                    program_state = ProgramState::WaitingAfterNegation;
                    parse_start_index = i;
                    continue;
                }

                if char.is_ascii_digit() {
                    program_state = ProgramState::ProcessingInteger;
                    parse_start_index = i;
                    parse_end_index = i;
                    continue;
                }

                if char == '(' {
                    program_state = ProgramState::WaitingAfterOpenBrace;
                    braces_stack.push(Brace::Start);
                    primitives.push(Primitive::Brace(Brace::Start));
                    continue;
                }
            }
            ProgramState::WaitingAfterCloseBrace => {
                if char == ')' {
                    program_state = ProgramState::WaitingAfterCloseBrace;
                    braces_stack
                        .pop()
                        .ok_or(format!("Braces mismatch at {i}"))?;
                    primitives.push(Primitive::Brace(Brace::End));
                    continue;
                }

                if char.is_ascii_whitespace() {
                    continue;
                }

                match is_math_operator(&char) {
                    Some(operator) => {
                        program_state = ProgramState::WaitingAfterOperator;
                        primitives.push(Primitive::Operator(operator));
                        continue;
                    }
                    None => (),
                }
            }
        };
        return Err(format!("Unexpected character: {char} at {i}"));
    }

    if !braces_stack.is_empty() {
        display_braces(&braces_stack);
        return Err("Braces mismatch".to_string());
    }

    match program_state {
        ProgramState::Started => return Err("Nothing given".to_string()),
        ProgramState::FinishedNumber | ProgramState::WaitingAfterCloseBrace => (),
        ProgramState::ProcessingInteger => {
            let integer = parse_slice(trimmed, &parse_start_index, &parse_end_index)?;
            primitives.push(Primitive::Number(Number::Integer(integer)));
        }
        ProgramState::ProcessingFloat => {
            let float = parse_slice(trimmed, &parse_start_index, &parse_end_index)?;
            primitives.push(Primitive::Number(Number::Float(float)));
        }
        ProgramState::WaitingAfterFloatPoint => return Err("Orphan float point".to_string()),
        ProgramState::WaitingAfterNegation => return Err("Orphan negation operator".to_string()),
        ProgramState::WaitingAfterOperator => return Err("Orphan operator".to_string()),
        ProgramState::WaitingAfterOpenBrace => return Err("Orphan open brace".to_string()),
    }

    display_primitives(&primitives);

    Ok(Number::Integer(1))
}

fn main() -> ExitCode {
    let expression = "(-1.1 + -6) * -4";

    match calculate(expression) {
        Ok(value) => {
            match value {
                Number::Float(float) => {
                    println!("float: {float}")
                }
                Number::Integer(integer) => {
                    println!("integer: {integer}")
                }
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::FAILURE
        }
    }
}
