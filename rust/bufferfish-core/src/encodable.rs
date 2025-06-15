//! Types implementing this trait are able to be encoded to a `Bufferfish`. Implements encoding for primitive types.

use crate::{Bufferfish, BufferfishError};

/// Types implementing this trait are able to be encoded to a `Bufferfish`.
pub trait Encodable: Sized {
    /// Encode this type into a given `Bufferfish`.
    fn encode(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        self.encode_value(bf)
    }

    /// Encodes a raw value into a given `Bufferfish`.
    ///
    /// Note: This is generally not what you want to call on types
    /// implementing `Encodable`, as it will not encode the header value.
    /// Instead, use `encode` to encode an entire type.
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError>;

    /// Encode the type into a new `Bufferfish`.
    ///
    /// Note: As this allocates a new `Bufferfish`, consider using
    /// `encode` instead and reusing a previously allocated `Bufferfish`.
    fn to_bufferfish(&self) -> Result<Bufferfish, BufferfishError> {
        let mut bf = Bufferfish::new();
        self.encode(&mut bf)?;

        Ok(bf)
    }
}

impl Encodable for u8 {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_u8(*self)
    }
}

impl Encodable for u16 {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_u16(*self)
    }
}

impl Encodable for u32 {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_u32(*self)
    }
}

impl Encodable for u64 {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_u64(*self)
    }
}

impl Encodable for u128 {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_u128(*self)
    }
}

impl Encodable for i8 {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_i8(*self)
    }
}

impl Encodable for i16 {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_i16(*self)
    }
}

impl Encodable for i32 {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_i32(*self)
    }
}

impl Encodable for i64 {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_i64(*self)
    }
}

impl Encodable for i128 {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_i128(*self)
    }
}

impl Encodable for bool {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_bool(*self)
    }
}

impl Encodable for String {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_string(self)
    }
}

impl<T: Encodable> Encodable for Vec<T> {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_array(self)
    }
}

impl<T: Encodable> Encodable for Option<T> {
    fn encode_value(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        match self {
            Some(value) => {
                bf.write_u8(1)?;
                value.encode_value(bf)
            }
            None => bf.write_u8(0),
        }
    }
}
