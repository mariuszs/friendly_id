// Based on code from https://github.com/fbernier/base62

use crate::error::DecodeError;
use crate::error::DecodeError::{ArithmeticOverflow, InvalidBase62Byte};

const BASE: u128 = 62;
const ALPHABET: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const DECODE_MAP: [u8; 256] = {
    let mut map = [u8::MAX; 256];
    let mut i = 0usize;
    while i < 62 {
        #[allow(clippy::cast_possible_truncation)]
        { map[ALPHABET[i] as usize] = i as u8; }
        i += 1;
    }
    map
};

pub(crate) fn encode(mut num: u128) -> String {
    if num == 0 {
        return "0".to_owned();
    }
    let mut buf = [0u8; 22];
    let mut pos = 22usize;
    while num > 0 {
        pos -= 1;
        buf[pos] = ALPHABET[(num % BASE) as usize];
        num /= BASE;
    }
    std::str::from_utf8(&buf[pos..]).unwrap().to_owned()
}

pub(crate) fn decode(string: &str) -> Result<u128, DecodeError> {
    let mut result: u128 = 0;
    let mut power: u128 = 1;

    for (i, c) in string.as_bytes().iter().rev().enumerate() {
        if i > 0 {
            power = power.checked_mul(BASE).ok_or(ArithmeticOverflow)?;
        }
        let idx = DECODE_MAP[*c as usize];
        if idx == u8::MAX {
            return Err(InvalidBase62Byte(*c as char, string.len() - i));
        }
        let z = u128::from(idx)
            .checked_mul(power)
            .ok_or(ArithmeticOverflow)?;
        result = result.checked_add(z).ok_or(ArithmeticOverflow)?;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::base62;
    use crate::error::DecodeError;

    #[test]
    fn test_encode() {
        assert_eq!(base62::encode(852751187393), "F0ob4rZ");
    }

    #[test]
    fn test_decode() {
        assert_eq!(base62::decode("F0ob4rZ").unwrap(), 852751187393);
    }

    #[test]
    fn test_decode_invalid_char() {
        let result = base62::decode("ds{Z455f");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DecodeError::InvalidBase62Byte('{', 3));
    }

    #[test]
    fn test_decode_long_string() {
        let result = base62::decode(
            "dsZ455fzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz\
             zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz",
        );
        assert_eq!(result.unwrap_err(), DecodeError::ArithmeticOverflow);
    }
}
