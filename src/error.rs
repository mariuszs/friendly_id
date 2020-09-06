use failure::Fail;

pub use self::DecodeError::*;

#[derive(Fail, Debug)]
pub enum DecodeError {
    #[fail(display = "Invalid character '{}' at position {}", _0, _1)]
    InvalidBase62Byte(char, usize),
    #[fail(display = "Decode result is too large")]
    ArithmeticOverflow,
}