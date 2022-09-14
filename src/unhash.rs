use crate::HashFn;
use std::convert::TryInto;

/// A hash function that simply takes the last 4 bytes of a given key as the
/// hash value.
#[derive(Eq, PartialEq)]
pub struct UnHashFn;

impl HashFn for UnHashFn {
    #[inline]
    fn hash(bytes: &[u8]) -> u64 {
        let len = bytes.len();
        u64::from_le_bytes(bytes[len - 8..].try_into().unwrap())
    }
}
