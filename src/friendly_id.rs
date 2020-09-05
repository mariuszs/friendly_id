use uuid::Uuid;

use crate::base62;

pub fn create() -> String {
    let uuid = Uuid::new_v4();
    return encode(&uuid);
}

pub fn encode(uuid: &Uuid) -> String {
    let data = uuid.as_u128();
    let base62 = base62::encode(data);
    return base62.to_string();
}

pub fn decode(id: String) -> Uuid {
    let decode_base62 = base62::decode(id.as_str()).expect("Invalid id");
    return Uuid::from_u128(decode_base62);
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::friendly_id;

    #[test]
    fn test_decode() {
        assert_eq!(friendly_id::decode("5wbwf6yUxVBcr48AMbz9cb".to_string()), Uuid::parse_str("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap());
    }

    #[test]
    fn test_create() {
        assert!(friendly_id::create().len() > 1);
    }

    #[test]
    fn test_encode() {
        assert_eq!(friendly_id::encode(&Uuid::parse_str("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap()), "5wbwf6yUxVBcr48AMbz9cb");
    }
}