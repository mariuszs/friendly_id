use uuid::Uuid;

use crate::base62;
use crate::error::DecodeError;

///
/// Create FriendlyID id
/// # Examples
///
/// ```
/// let result = friendly_id::create();
///
/// "5wbwf6yUxVBcr48AMbz9cb";
/// ```
///
pub fn create() -> String {
    let uuid = Uuid::new_v4();
    return encode(&uuid);
}

///
/// Encode UUID to FriendlyID id
///
/// ```
/// let uuid = uuid::Uuid::parse_str("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap();
/// friendly_id::encode(&uuid);
///
/// "5wbwf6yUxVBcr48AMbz9cb";
/// ```
///
pub fn encode(uuid: &Uuid) -> String {
    let data = uuid.as_u128();
    let base62 = base62::encode(data);
    return base62.to_string();
}

///
/// Decode FriendlyID id to UUID
///
/// ```
/// friendly_id::decode("5wbwf6yUxVBcr48AMbz9cb");
///
/// "c3587ec5-0976-497f-8374-61e0c2ea3da5";
/// ```
///
pub fn decode(id: &str) -> Result<Uuid, DecodeError> {
    match base62::decode(id) {
        Ok(base62) => return Ok(Uuid::from_u128(base62)),
        Err(e) => return Err(e)
    };
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::friendly_id;

    #[test]
    fn test_decode() {
        assert_eq!(friendly_id::decode("5wbwf6yUxVBcr48AMbz9cb").unwrap(),
                   Uuid::parse_str("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap());
    }

    #[test]
    fn test_decode_invalid_value() {
        let result = friendly_id::decode("5+");
        assert!(result.is_err());
        let error = result.unwrap_err();
        println!("{}", error.to_string());
    }

    #[test]
    fn test_create() {
        assert!(friendly_id::create().len() > 1);
    }

    #[test]
    fn test_encode() {
        let uuid = Uuid::parse_str("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap();
        assert_eq!(friendly_id::encode(&uuid), "5wbwf6yUxVBcr48AMbz9cb");
    }
}