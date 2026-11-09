#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    Integer(i32),
    Float(f32),
}

impl Number {
    pub fn add(&self, num: &Number) -> Number {
        match self {
            Number::Float(left) => match num {
                Number::Float(right) => Number::Float(left + right),
                Number::Integer(right) => Number::Float(left + *right as f32),
            },
            Number::Integer(left) => match num {
                Number::Float(right) => Number::Float(*left as f32 + right),
                Number::Integer(right) => Number::Integer(left + right),
            },
        }
    }

    pub fn sub(&self, num: &Number) -> Number {
        match self {
            Number::Float(left) => match num {
                Number::Float(right) => Number::Float(left - right),
                Number::Integer(right) => Number::Float(left - *right as f32),
            },
            Number::Integer(left) => match num {
                Number::Float(right) => Number::Float(*left as f32 - right),
                Number::Integer(right) => Number::Integer(left - right),
            },
        }
    }

    pub fn mul(&self, num: &Number) -> Number {
        match self {
            Number::Float(left) => match num {
                Number::Float(right) => Number::Float(left * right),
                Number::Integer(right) => Number::Float(left * *right as f32),
            },
            Number::Integer(left) => match num {
                Number::Float(right) => Number::Float(*left as f32 * right),
                Number::Integer(right) => Number::Integer(left * right),
            },
        }
    }

    pub fn div(&self, num: &Number) -> Number {
        match self {
            Number::Float(left) => match num {
                Number::Float(right) => Number::Float(left / right),
                Number::Integer(right) => Number::Float(left / *right as f32),
            },
            Number::Integer(left) => match num {
                Number::Float(right) => Number::Float(*left as f32 / right),
                Number::Integer(right) => Number::Integer(left / right),
            },
        }
    }

    pub fn exp(&self, num: &Number) -> Number {
        match self {
            Number::Float(left) => match num {
                Number::Float(right) => Number::Float(left.powf(*right)),
                Number::Integer(right) => Number::Float(left.powi(*right)),
            },
            Number::Integer(left) => match num {
                Number::Float(right) => Number::Float((*left as f32).powf(*right)),
                Number::Integer(right) => Number::Float((*left as f32).powi(*right)),
            },
        }
    }
}
