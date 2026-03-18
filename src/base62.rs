// Based on code from https://github.com/fbernier/base62

use crate::error::DecodeError;
use crate::error::DecodeError::{ArithmeticOverflow, InvalidBase62Byte};

const BASE: u128 = 62;
const ALPHABET: [u8; BASE as usize] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J',
    b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T',
    b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
    b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
    b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x',
    b'y', b'z',
];

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
    String::from_utf8(buf[pos..].to_vec()).unwrap()
}

pub(crate) fn decode(string: &str) -> Result<u128, DecodeError> {
    let mut result: u128 = 0;

    for (i, c) in string.as_bytes().iter().rev().enumerate() {
        let exp = u32::try_from(i).map_err(|_| ArithmeticOverflow)?;
        let num = BASE.checked_pow(exp).ok_or(ArithmeticOverflow)?;
        let idx = DECODE_MAP[*c as usize];
        if idx == u8::MAX {
            return Err(InvalidBase62Byte(*c as char, string.len() - i));
        }
        let z = u128::from(idx)
            .checked_mul(num)
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
