use crate::core::interpreter::value::Value;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub value: Value,
    pub reassignable: bool,
}

impl Symbol {
    pub fn new(value: Value, reassignable: bool) -> Self {
        Self {
            value,
            reassignable,
        }
    }
}
