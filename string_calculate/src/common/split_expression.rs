use super::primitives::{Brace, Number, Operator, Primitive};
use std::fmt::Display;

enum SplitExpressionState {
    Default,
    WaitingStartBrace,
}

#[derive(Debug, PartialEq)]
pub struct SubExpression<'a> {
    pub left: &'a [Primitive],
    pub right: &'a [Primitive],
    pub operator: &'a Operator,
}

#[derive(Debug, PartialEq)]
pub enum SplitExressionResult<'a> {
    SubExpression(SubExpression<'a>),
    Number(&'a Number),
}

impl<'a> Display for SubExpression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Left: ")?;
        for p in self.left.iter() {
            write!(f, "{} ", p)?;
        }

        writeln!(f)?;
        write!(f, "Right: ")?;
        for p in self.right.iter() {
            write!(f, "{} ", p)?;
        }

        writeln!(f)?;
        writeln!(f, "Operator: {}", self.operator)?;

        writeln!(f, "")
    }
}

pub fn split_expression<'a>(
    primitives: &'a [Primitive],
) -> Result<SplitExressionResult<'a>, String> {
    //нужно найти оператор с наименьшим приоритетом
    let mut state: SplitExpressionState = SplitExpressionState::Default;
    let mut braces_stack: Vec<Brace> = Vec::new();
    let mut min_operator_reverse_index: Option<usize> = None;
    let mut min_operator: Option<&Operator> = None;

    //есть ли выражение?
    for (i, primitive) in primitives.iter().rev().enumerate() {
        match primitive {
            Primitive::Brace(Brace::End) => {
                state = SplitExpressionState::WaitingStartBrace;
                braces_stack.push(Brace::End);
                continue;
            }
            Primitive::Brace(Brace::Start) => match braces_stack.pop() {
                Some(_) => {
                    state = SplitExpressionState::Default;
                    continue;
                }
                None => {
                    return Err("Braces stack error".to_string());
                }
            },
            Primitive::Operator(operator) => {
                match state {
                    SplitExpressionState::Default => match min_operator {
                        Some(min_op) => {
                            if *min_op == Operator::Exponentiation
                                && *operator == Operator::Exponentiation
                            {
                                min_operator_reverse_index = Some(i);
                                continue;
                            }

                            if min_op.get_order() > operator.get_order() {
                                min_operator = Some(operator);
                                min_operator_reverse_index = Some(i);
                                continue;
                            }
                        }
                        None => {
                            min_operator = Some(operator);
                            min_operator_reverse_index = Some(i);
                        }
                    },
                    _ => (),
                }
                continue;
            }
            _ => {
                continue;
            }
        };
    }

    let Some(min_operator_reverse_index) = min_operator_reverse_index else {
        return match primitives.first() {
            Some(Primitive::Number(n)) => Ok(SplitExressionResult::Number(n)),
            _ => Err("unreachable".to_string()),
        };
    };

    let Some(min_operator) = min_operator else {
        return Err("no min operator".to_string());
    };

    let absolute_index = primitives.len() - min_operator_reverse_index;
    let left = &primitives[..absolute_index - 1];
    let right = &primitives[absolute_index..];

    Ok(SplitExressionResult::SubExpression(SubExpression {
        left,
        right,
        operator: &min_operator,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_expression() {
        let primitives = [
            Primitive::Number(Number::Integer(1)),
            Primitive::Operator(Operator::Addition),
            Primitive::Number(Number::Integer(2)),
        ];

        let expect = SplitExressionResult::SubExpression(SubExpression {
            left: &[Primitive::Number(Number::Integer(1))],
            right: &[Primitive::Number(Number::Integer(2))],
            operator: &Operator::Addition,
        });

        let Ok(result) = split_expression(&primitives) else {
            assert!(false);
            return;
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn simple_value() {
        let primitives = [Primitive::Number(Number::Integer(1))];
        let expect = SplitExressionResult::Number(&Number::Integer(1));

        let Ok(result) = split_expression(&primitives) else {
            assert!(false);
            return;
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn expression_with_braces() {
        let primitives = [
            Primitive::Brace(Brace::Start),
            Primitive::Number(Number::Integer(1)),
            Primitive::Operator(Operator::Addition),
            Primitive::Number(Number::Integer(2)),
            Primitive::Brace(Brace::End),
            Primitive::Operator(Operator::Multiplication),
            Primitive::Number(Number::Integer(3)),
        ];

        let expect = SplitExressionResult::SubExpression(SubExpression {
            left: &[
                Primitive::Brace(Brace::Start),
                Primitive::Number(Number::Integer(1)),
                Primitive::Operator(Operator::Addition),
                Primitive::Number(Number::Integer(2)),
                Primitive::Brace(Brace::End),
            ],
            right: &[Primitive::Number(Number::Integer(3))],
            operator: &Operator::Multiplication,
        });

        let Ok(result) = split_expression(&primitives) else {
            assert!(false);
            return;
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn expression_with_priorities() {
        let primitives = [
            Primitive::Number(Number::Integer(1)),
            Primitive::Operator(Operator::Addition),
            Primitive::Number(Number::Integer(2)),
            Primitive::Operator(Operator::Addition),
            Primitive::Number(Number::Integer(3)),
        ];

        let expect = SplitExressionResult::SubExpression(SubExpression {
            left: &[
                Primitive::Number(Number::Integer(1)),
                Primitive::Operator(Operator::Addition),
                Primitive::Number(Number::Integer(2)),
            ],
            right: &[Primitive::Number(Number::Integer(3))],
            operator: &Operator::Addition,
        });

        let Ok(result) = split_expression(&primitives) else {
            assert!(false);
            return;
        };

        assert_eq!(result, expect);
    }
}
