//! Base trait for types that can be decoded from a `Bufferfish`. Implements decoding for primitive types.

use crate::{Bufferfish, BufferfishError};

/// Types implementing this trait are able to be decoded from a `Bufferfish`.
pub trait Decodable: Sized {
    /// Decode the type from a `Bufferfish`.
    fn decode(bf: &mut Bufferfish) -> Result<Self, BufferfishError>;
}

impl Decodable for u8 {
    fn decode(bf: &mut Bufferfish) -> Result<u8, BufferfishError> {
        bf.read_u8().map_err(BufferfishError::from)
    }
}

impl Decodable for u16 {
    fn decode(bf: &mut Bufferfish) -> Result<u16, BufferfishError> {
        bf.read_u16().map_err(BufferfishError::from)
    }
}

impl Decodable for u32 {
    fn decode(bf: &mut Bufferfish) -> Result<u32, BufferfishError> {
        bf.read_u32().map_err(BufferfishError::from)
    }
}

impl Decodable for i8 {
    fn decode(bf: &mut Bufferfish) -> Result<i8, BufferfishError> {
        bf.read_i8().map_err(BufferfishError::from)
    }
}

impl Decodable for i16 {
    fn decode(bf: &mut Bufferfish) -> Result<i16, BufferfishError> {
        bf.read_i16().map_err(BufferfishError::from)
    }
}

impl Decodable for i32 {
    fn decode(bf: &mut Bufferfish) -> Result<i32, BufferfishError> {
        bf.read_i32().map_err(BufferfishError::from)
    }
}

impl Decodable for bool {
    fn decode(bf: &mut Bufferfish) -> Result<bool, BufferfishError> {
        bf.read_bool().map_err(BufferfishError::from)
    }
}

impl Decodable for String {
    fn decode(bf: &mut Bufferfish) -> Result<String, BufferfishError> {
        bf.read_string().map_err(BufferfishError::from)
    }
}
