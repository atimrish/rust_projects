use super::primitives::{Brace, Primitive};

enum UnwrapState {
    Default,
    WaitingBraceEnd,
}

pub fn unwrap_braces(primitives: &[Primitive]) -> &[Primitive] {
    let mut braces_stack: Vec<Brace> = Vec::new();
    let mut padding = 0;
    let mut _state: UnwrapState = UnwrapState::Default;
    let mut _has_brace = false;

    loop {
        for p in primitives[padding..primitives.len() - padding].iter() {
            match p {
                Primitive::Brace(Brace::Start) => {
                    _state = UnwrapState::WaitingBraceEnd;
                    _has_brace = true;
                    braces_stack.push(Brace::Start);
                    continue;
                }
                Primitive::Brace(Brace::End) => {
                    if braces_stack.pop().is_none() {
                        _state = UnwrapState::Default;
                    }
                    continue;
                }
                Primitive::Operator(_) => {
                    if braces_stack.is_empty() {
                        return &primitives[padding..primitives.len() - padding];
                    }
                }
                _ => continue,
            };
        }
        padding = padding + 1;

        if padding + 1 > primitives.len() / 2 {
            if _has_brace {
                return &primitives[padding..primitives.len() - padding];
            } else {
                return &primitives;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::primitives::{Number, Operator, Primitive};

    #[test]
    fn expression_without_braces() {
        let primitives: [Primitive; 3] = [
            Primitive::Number(Number::Integer(1)),
            Primitive::Operator(Operator::Addition),
            Primitive::Number(Number::Integer(2)),
        ];

        let expect: [Primitive; 3] = [
            Primitive::Number(Number::Integer(1)),
            Primitive::Operator(Operator::Addition),
            Primitive::Number(Number::Integer(2)),
        ];

        let result = unwrap_braces(&primitives);
    }
}
