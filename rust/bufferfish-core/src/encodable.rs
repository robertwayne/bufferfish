//! Types implementing this trait are able to be encoded to a `Bufferfish`. Implements encoding for primitive types.

use crate::{Bufferfish, BufferfishError};

/// Types implementing this trait are able to be encoded to a `Bufferfish`.
pub trait Encodable {
    /// Encode the type into a given `Bufferfish`.
    fn encode(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError>;

    /// Encode the type into a new `Bufferfish`.
    fn to_bufferfish(&self) -> Result<Bufferfish, BufferfishError> {
        let mut bf = Bufferfish::new();
        self.encode(&mut bf)?;

        Ok(bf)
    }
}

impl Encodable for u8 {
    fn encode(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_u8(*self)
    }
}

impl Encodable for u16 {
    fn encode(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_u16(*self)
    }
}

impl Encodable for u32 {
    fn encode(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_u32(*self)
    }
}

impl Encodable for u64 {
    fn encode(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_u64(*self)
    }
}

impl Encodable for i8 {
    fn encode(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_i8(*self)
    }
}

impl Encodable for i16 {
    fn encode(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_i16(*self)
    }
}

impl Encodable for i32 {
    fn encode(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_i32(*self)
    }
}

impl Encodable for i64 {
    fn encode(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_i64(*self)
    }
}

impl Encodable for bool {
    fn encode(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_bool(*self)
    }
}

impl Encodable for String {
    fn encode(&self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        bf.write_string(self)
    }
}
