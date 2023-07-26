use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ParsingError {
    err: String,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl Error for ParsingError {
    fn description(&self) -> &str {
        &self.err
    }
}

impl ParsingError {
    pub fn new(msg: &str) -> Self {
        Self {
            err: msg.to_string(),
        }
    }
}
