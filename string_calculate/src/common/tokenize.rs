use super::primitives::{Brace, Number, Primitive, ProgramError, ProgramState};
use super::utils::{is_math_operator, parse_slice};

//TODO: Нужно посимвольно пробежаться по строке и распарсить ее на токены
pub fn tokenize(expression: &str) -> Result<Vec<Primitive>, ProgramError> {
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
                    braces_stack.pop().ok_or(ProgramError::BracesMismatch)?;

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
                    braces_stack.pop().ok_or(ProgramError::BracesMismatch)?;

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
                    braces_stack.pop().ok_or(ProgramError::BracesMismatch)?;
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
                    braces_stack.pop().ok_or(ProgramError::BracesMismatch)?;
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
        return Err(ProgramError::UnexpectedCharacter(char));
    }

    if !braces_stack.is_empty() {
        return Err(ProgramError::BracesMismatch);
    }

    match program_state {
        ProgramState::Started => return Err(ProgramError::NothingGiven),
        ProgramState::FinishedNumber | ProgramState::WaitingAfterCloseBrace => (),
        ProgramState::ProcessingInteger => {
            let integer = parse_slice(trimmed, &parse_start_index, &parse_end_index)?;
            primitives.push(Primitive::Number(Number::Integer(integer)));
        }
        ProgramState::ProcessingFloat => {
            let float = parse_slice(trimmed, &parse_start_index, &parse_end_index)?;
            primitives.push(Primitive::Number(Number::Float(float)));
        }
        ProgramState::WaitingAfterFloatPoint => return Err(ProgramError::UnexpectedCharacter('.')),
        ProgramState::WaitingAfterNegation => return Err(ProgramError::UnexpectedCharacter('-')),
        ProgramState::WaitingAfterOperator => {
            return Err(ProgramError::UnexpectedCharacter(
                trimmed.chars().next_back().unwrap_or(' '),
            ));
        }
        ProgramState::WaitingAfterOpenBrace => return Err(ProgramError::BracesMismatch),
    }

    Ok(primitives)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::primitives::Operator;

    #[test]
    fn simple_expression() {
        let expression = "1 + 1";
        let expect = [
            Primitive::Number(Number::Integer(1)),
            Primitive::Operator(Operator::Addition),
            Primitive::Number(Number::Integer(1)),
        ];

        let Ok(result) = tokenize(expression) else {
            assert!(false);
            return;
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn simple_value() {
        let expression = "1";
        let expect = [Primitive::Number(Number::Integer(1))];

        let Ok(result) = tokenize(expression) else {
            assert!(false);
            return;
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn negative_integer() {
        let expression = "-1";
        let expect = [Primitive::Number(Number::Integer(-1))];

        let Ok(result) = tokenize(expression) else {
            assert!(false);
            return;
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn negative_float() {
        let expression = "-1.1";
        let expect = [Primitive::Number(Number::Float(-1.1))];

        let Ok(result) = tokenize(expression) else {
            assert!(false);
            return;
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn negation_expression() {
        let expression = "1 + -1";
        let expect = [
            Primitive::Number(Number::Integer(1)),
            Primitive::Operator(Operator::Addition),
            Primitive::Number(Number::Integer(-1)),
        ];

        let Ok(result) = tokenize(expression) else {
            assert!(false);
            return;
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn orphan_float_point() {
        let expression = "1.";

        let Err(error) = tokenize(expression) else {
            assert!(false);
            return;
        };

        assert_eq!(error, ProgramError::UnexpectedCharacter('.'));
    }

    #[test]
    fn double_negation() {
        let expression = "--1";

        let Err(error) = tokenize(expression) else {
            assert!(false);
            return;
        };

        assert_eq!(error, ProgramError::UnexpectedCharacter('-'));
    }

    #[test]
    fn braces() {
        let expression = "(1 + 1)";
        let expect = [
            Primitive::Brace(Brace::Start),
            Primitive::Number(Number::Integer(1)),
            Primitive::Operator(Operator::Addition),
            Primitive::Number(Number::Integer(1)),
            Primitive::Brace(Brace::End),
        ];

        let Ok(result) = tokenize(expression) else {
            assert!(false);
            return;
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn braces_mismatch_start() {
        let expression = "((1 + 1)";

        let Err(error) = tokenize(expression) else {
            assert!(false);
            return;
        };

        assert_eq!(error, ProgramError::BracesMismatch);
    }

    #[test]
    fn braces_mismatch_end() {
        let expression = "(1 + 1))";

        let Err(error) = tokenize(expression) else {
            assert!(false);
            return;
        };

        assert_eq!(error, ProgramError::BracesMismatch);
    }
}
