use std::process::ExitCode;

enum MathExpressionNumber {
    Integer,
    Float,
}

enum MathExpressionOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
    Negation,
}

enum MathExpressionElement {
    Number(MathExpressionNumber),
    Operator(MathExpressionOperator),
}

#[derive(Eq, PartialEq)]
enum ProgramState {
    Started,
    Idle,
    ParseInteger,
    ParseFloat,
    WaitingDigitAfterFloatPoint,
}

enum NumberValue {
    Integer(i32),
    Float(f32),
}

//TODO: Нужно посимвольно пробежаться по строке и распарсить ее
fn calculate(expression: &str) -> Result<NumberValue, String> {
    let trimmed = expression.trim();

    let mut program_state: ProgramState = ProgramState::Started;
    let mut parse_start_index: usize = 0;
    let mut parse_end_index: usize = 0;

    for (i, char) in trimmed.chars().enumerate() {
        if char.is_ascii_digit() {
            match program_state {
                ProgramState::Started | ProgramState::Idle => {
                    program_state = ProgramState::ParseInteger;
                    parse_start_index = i;
                }
                ProgramState::WaitingDigitAfterFloatPoint => {
                    program_state = ProgramState::ParseFloat;
                }
                ProgramState::ParseInteger | ProgramState::ParseFloat => (),
            };

            parse_end_index = i;
            continue;
        }

        //TODO: need to implement
        if char.is_ascii_whitespace() {
            match program_state {
                ProgramState::WaitingDigitAfterFloatPoint => {
                    return Err(String::from("Orphan point symbol found"));
                }
                ProgramState::Started
                | ProgramState::Idle
                | ProgramState::ParseFloat
                | ProgramState::ParseInteger => {
                    program_state = ProgramState::Idle;
                    continue;
                }
            }
        }

        if char == '.' {
            match program_state {
                ProgramState::ParseInteger => {
                    program_state = ProgramState::WaitingDigitAfterFloatPoint;
                    continue;
                }
                ProgramState::Started
                | ProgramState::Idle
                | ProgramState::ParseFloat
                | ProgramState::WaitingDigitAfterFloatPoint => {
                    return Err(String::from("Unexpected point symbol"));
                }
            }
        }

        //TODO: need to implement
        if char == '+' {
            return Ok(NumberValue::Integer(1));
        }

        //TODO: need to implement
        if char == '-' {
            return Ok(NumberValue::Integer(1));
        }

        //TODO: need to implement
        if char == '*' {
            return Ok(NumberValue::Integer(1));
        }

        //TODO: need to implement
        if char == '/' {
            return Ok(NumberValue::Integer(1));
        }

        //TODO: need to implement
        if char == '^' {
            return Ok(NumberValue::Integer(1));
        }

        //TODO: need to implement
        if char == '(' {
            return Ok(NumberValue::Integer(1));
        }

        //TODO: need to implement
        if char == ')' {
            return Ok(NumberValue::Integer(1));
        }

        return Err(format!("Unexpected character: {char}"));
    }

    let number_slice = &trimmed[parse_start_index..parse_end_index + 1];

    //Calc the value
    match program_state {
        ProgramState::Started | ProgramState::Idle => Err(String::from("No expression given")),
        ProgramState::ParseInteger => match number_slice.parse::<i32>() {
            Ok(integer) => Ok(NumberValue::Integer(integer)),
            Err(_) => Err(format!("Error parse value {number_slice} into i32")),
        },
        ProgramState::ParseFloat => match number_slice.parse::<f32>() {
            Ok(float) => Ok(NumberValue::Float(float)),
            Err(_) => Err(format!("Error parse value {number_slice} into f32")),
        },
        ProgramState::WaitingDigitAfterFloatPoint => Err(String::from("Orphan point symbol found")),
    }
}

fn main() -> ExitCode {
    let expression = "0.1";

    match calculate(expression) {
        Ok(value) => {
            match value {
                NumberValue::Float(float) => {
                    println!("float: {float}")
                }
                NumberValue::Integer(integer) => {
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
