use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Invalid character '{0}' at position {1}")]
    InvalidBase62Byte(char, usize),
    #[error("Decode result is too large")]
    ArithmeticOverflow,
}
