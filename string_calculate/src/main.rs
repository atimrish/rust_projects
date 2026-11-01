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

fn parse_integer_slice(expression: &str, start: &usize, end: &usize) -> Result<i32, String> {
    let integer_slice = &expression[*start..*end + 1];
    integer_slice
        .parse::<i32>()
        .map_err(|_| format!("Error parse integer: {integer_slice}"))
}

fn parse_float_slice(expression: &str, start: &usize, end: &usize) -> Result<f32, String> {
    let float_slice = &expression[*start..*end + 1];
    float_slice
        .parse::<f32>()
        .map_err(|_| format!("Error parse float: {float_slice}"))
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
                ProgramState::Started | ProgramState::Idle => (),
                ProgramState::WaitingDigitAfterFloatPoint => {
                    return Err(String::from("Orphan point symbol found"));
                }
                ProgramState::ParseFloat => {
                    program_state = ProgramState::Idle;
                    parse_end_index = i;

                    let float = parse_float_slice(trimmed, &parse_start_index, &parse_end_index)?;
                }
                ProgramState::ParseInteger => {
                    program_state = ProgramState::Idle;
                    parse_end_index = i;

                    let integer =
                        parse_integer_slice(trimmed, &parse_start_index, &parse_end_index)?;
                }
            }

            continue;
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

    //Calc the value
    match program_state {
        ProgramState::Started | ProgramState::Idle => Err(String::from("No expression given")),
        ProgramState::ParseInteger => {
            parse_integer_slice(trimmed, &parse_start_index, &parse_end_index)
                .map(|v| Ok(NumberValue::Integer(v)))?
        }
        ProgramState::ParseFloat => {
            parse_float_slice(trimmed, &parse_start_index, &parse_end_index)
                .map(|v| Ok(NumberValue::Float(v)))?
        }
        ProgramState::WaitingDigitAfterFloatPoint => Err(String::from("Orphan point symbol found")),
    }
}

fn main() -> ExitCode {
    let expression = "1";

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
