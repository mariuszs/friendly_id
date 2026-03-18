//! # `friendly_id`
//!
//! Converts UUIDs to URL-friendly Base62 IDs and back.
//!
//! The Base62 alphabet used is `0-9A-Za-z`, producing a compact string
//! of up to 22 characters for any UUID.
//!
//! # Examples
//!
//! ```rust
//! // Encode a UUID to a friendly ID
//! let uuid = uuid::Uuid::parse_str("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap();
//! let id = friendly_id::encode(&uuid);
//! assert_eq!(id, "5wbwf6yUxVBcr48AMbz9cb");
//!
//! // Decode back to UUID
//! let decoded = friendly_id::decode(&id).unwrap();
//! assert_eq!(decoded, uuid);
//! ```

#[cfg(feature = "v4")]
pub use self::friendly_id::create;
pub use self::error::DecodeError;
pub use self::friendly_id::{decode, encode};

mod base62;
mod error;
mod friendly_id;
