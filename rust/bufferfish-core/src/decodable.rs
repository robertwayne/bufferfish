//! Base trait for types that can be decoded from a `Bufferfish`. Implements decoding for primitive types.

use crate::{Bufferfish, BufferfishError};

/// Types implementing this trait are able to be decoded from a `Bufferfish`.
pub trait Decodable: Sized {
    /// Decode the type from a `Bufferfish`.
    fn decode(bf: &mut Bufferfish) -> Result<Self, BufferfishError>;
}

impl Decodable for u8 {
    fn decode(bf: &mut Bufferfish) -> Result<u8, BufferfishError> {
        bf.read_u8()
    }
}

impl Decodable for u16 {
    fn decode(bf: &mut Bufferfish) -> Result<u16, BufferfishError> {
        bf.read_u16()
    }
}

impl Decodable for u32 {
    fn decode(bf: &mut Bufferfish) -> Result<u32, BufferfishError> {
        bf.read_u32()
    }
}

impl Decodable for u64 {
    fn decode(bf: &mut Bufferfish) -> Result<u64, BufferfishError> {
        bf.read_u64()
    }
}

impl Decodable for u128 {
    fn decode(bf: &mut Bufferfish) -> Result<u128, BufferfishError> {
        bf.read_u128()
    }
}

impl Decodable for i8 {
    fn decode(bf: &mut Bufferfish) -> Result<i8, BufferfishError> {
        bf.read_i8()
    }
}

impl Decodable for i16 {
    fn decode(bf: &mut Bufferfish) -> Result<i16, BufferfishError> {
        bf.read_i16()
    }
}

impl Decodable for i32 {
    fn decode(bf: &mut Bufferfish) -> Result<i32, BufferfishError> {
        bf.read_i32()
    }
}

impl Decodable for i64 {
    fn decode(bf: &mut Bufferfish) -> Result<i64, BufferfishError> {
        bf.read_i64()
    }
}

impl Decodable for i128 {
    fn decode(bf: &mut Bufferfish) -> Result<i128, BufferfishError> {
        bf.read_i128()
    }
}

impl Decodable for bool {
    fn decode(bf: &mut Bufferfish) -> Result<bool, BufferfishError> {
        bf.read_bool()
    }
}

impl Decodable for String {
    fn decode(bf: &mut Bufferfish) -> Result<String, BufferfishError> {
        bf.read_string()
    }
}

impl<T: Decodable> Decodable for Vec<T> {
    fn decode(bf: &mut Bufferfish) -> Result<Vec<T>, BufferfishError> {
        let len = bf.read_u16()? as usize;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::decode(bf)?);
        }
        Ok(vec)
    }
}
