pub use self::friendly_id::{create, encode, decode};
mod base62;
mod friendly_id;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
