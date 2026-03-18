use uuid::Uuid;

use crate::base62;
use crate::error::DecodeError;

/// Creates a `FriendlyID` based on a random UUID v4.
///
/// Requires the `v4` feature (enabled by default).
///
/// # Examples
///
/// ```rust
/// let id = friendly_id::create();
/// assert!(id.len() <= 22);
/// ```
#[cfg(feature = "v4")]
#[must_use]
pub fn create() -> String {
    encode(&Uuid::new_v4())
}

/// Encodes a UUID to a `FriendlyID` string (Base62).
///
/// # Examples
///
/// ```rust
/// let uuid = uuid::Uuid::parse_str("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap();
/// let id = friendly_id::encode(&uuid);
/// assert_eq!(id, "5wbwf6yUxVBcr48AMbz9cb");
/// ```
#[must_use]
pub fn encode(uuid: &Uuid) -> String {
    base62::encode(uuid.as_u128())
}

/// Decodes a `FriendlyID` string back to a UUID.
///
/// # Errors
///
/// Returns [`DecodeError::InvalidBase62Byte`] if the string contains a character
/// outside the Base62 alphabet, or [`DecodeError::ArithmeticOverflow`] if the
/// encoded value exceeds the maximum 128-bit integer.
///
/// # Examples
///
/// ```rust
/// let uuid = friendly_id::decode("5wbwf6yUxVBcr48AMbz9cb").unwrap();
/// assert_eq!(uuid.to_string(), "c3587ec5-0976-497f-8374-61e0c2ea3da5");
/// ```
pub fn decode(id: &str) -> Result<Uuid, DecodeError> {
    Ok(Uuid::from_u128(base62::decode(id)?))
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::error::DecodeError;
    use crate::friendly_id;

    #[test]
    fn test_decode() {
        assert_eq!(
            friendly_id::decode("5wbwf6yUxVBcr48AMbz9cb").unwrap(),
            Uuid::parse_str("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap()
        );
    }

    #[test]
    fn test_decode_invalid_value() {
        let error = friendly_id::decode("5+").unwrap_err();
        assert_eq!(error, DecodeError::InvalidBase62Byte('+', 2));
    }

    #[cfg(feature = "v4")]
    #[test]
    fn test_create() {
        let id = friendly_id::create();
        assert!(!id.is_empty() && id.len() <= 22);
        assert!(friendly_id::decode(&id).is_ok());
    }

    #[test]
    fn test_encode() {
        let uuid = Uuid::parse_str("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap();
        assert_eq!(friendly_id::encode(&uuid), "5wbwf6yUxVBcr48AMbz9cb");
    }

    #[test]
    fn test_round_trip() {
        let uuid = Uuid::parse_str("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap();
        assert_eq!(
            friendly_id::decode(&friendly_id::encode(&uuid)).unwrap(),
            uuid
        );
    }

    #[test]
    fn test_nil_uuid() {
        let nil = Uuid::nil();
        let encoded = friendly_id::encode(&nil);
        assert_eq!(encoded, "0");
        assert_eq!(friendly_id::decode(&encoded).unwrap(), nil);
    }

    #[test]
    fn test_max_uuid() {
        let max = Uuid::max();
        let encoded = friendly_id::encode(&max);
        assert_eq!(encoded.len(), 22);
        assert_eq!(friendly_id::decode(&encoded).unwrap(), max);
    }
}
