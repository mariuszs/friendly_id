use thiserror::Error;

/// Errors that can occur when decoding a Base62 `FriendlyID` string.
#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum DecodeError {
    /// Encountered a character not in the Base62 alphabet.
    #[error("Invalid character '{0}' at position {1}")]
    InvalidBase62Byte(char, usize),
    /// The encoded value exceeds the maximum representable 128-bit integer.
    #[error("Decode result is too large")]
    ArithmeticOverflow,
}
