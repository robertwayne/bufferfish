//! Base trait for types that can be decoded from a `Bufferfish`. Implements decoding for primitive types.

use crate::{Bufferfish, BufferfishError};

/// Types implementing this trait are able to be decoded from a `Bufferfish`.
pub trait Decodable: Sized {
    /// Decode the type from a given `Bufferfish`.
    fn decode(bf: &mut Bufferfish) -> Result<Self, BufferfishError> {
        Self::decode_value(bf)
    }

    /// Decodes a raw value from a given `Bufferfish`.
    ///
    /// Note: This is generally not what you want to call on types
    /// implementing `Decodable`, as it will not decode the header value.
    /// Instead, use `decode` to decode an entire type.
    fn decode_value(bf: &mut Bufferfish) -> Result<Self, BufferfishError>;

    /// Creates a checked, generic type from a `Bufferfish`.
    ///
    /// If the `Bufferfish` does not contain enough bytes to properly
    /// decode the type, an error is returned.
    fn from_bufferfish(bf: &mut Bufferfish) -> Result<Self, BufferfishError> {
        if let Some(min) = Self::min_bytes_required()
            && bf.len() < min
        {
            return Err(BufferfishError::InsufficientBytes {
                available: bf.len(),
                required: min,
            });
        }

        if let Some(max) = Self::max_bytes_allowed()
            && bf.len() > max
        {
            return Err(BufferfishError::ExcessiveBytes {
                available: bf.len(),
                max_allowed: max,
            });
        }

        Self::decode(bf)
    }

    /// Get the minimum number of bytes required to decode this type.
    /// Returns None if the size can't be determined statically.
    fn min_bytes_required() -> Option<usize> {
        None
    }

    /// Get the maximum number of bytes this type can occupy.
    /// Returns None if the size can't be determined statically.
    fn max_bytes_allowed() -> Option<usize> {
        None
    }
}

impl Decodable for u8 {
    fn decode_value(bf: &mut Bufferfish) -> Result<u8, BufferfishError> {
        bf.read_u8()
    }

    fn min_bytes_required() -> Option<usize> {
        Some(1)
    }

    fn max_bytes_allowed() -> Option<usize> {
        Some(1)
    }
}

impl Decodable for u16 {
    fn decode_value(bf: &mut Bufferfish) -> Result<u16, BufferfishError> {
        bf.read_u16()
    }

    fn min_bytes_required() -> Option<usize> {
        Some(2)
    }

    fn max_bytes_allowed() -> Option<usize> {
        Some(2)
    }
}

impl Decodable for u32 {
    fn decode_value(bf: &mut Bufferfish) -> Result<u32, BufferfishError> {
        bf.read_u32()
    }

    fn min_bytes_required() -> Option<usize> {
        Some(4)
    }

    fn max_bytes_allowed() -> Option<usize> {
        Some(4)
    }
}

impl Decodable for u64 {
    fn decode_value(bf: &mut Bufferfish) -> Result<u64, BufferfishError> {
        bf.read_u64()
    }

    fn min_bytes_required() -> Option<usize> {
        Some(8)
    }

    fn max_bytes_allowed() -> Option<usize> {
        Some(8)
    }
}

impl Decodable for u128 {
    fn decode_value(bf: &mut Bufferfish) -> Result<u128, BufferfishError> {
        bf.read_u128()
    }

    fn min_bytes_required() -> Option<usize> {
        Some(16)
    }

    fn max_bytes_allowed() -> Option<usize> {
        Some(16)
    }
}

impl Decodable for i8 {
    fn decode_value(bf: &mut Bufferfish) -> Result<i8, BufferfishError> {
        bf.read_i8()
    }

    fn min_bytes_required() -> Option<usize> {
        Some(1)
    }

    fn max_bytes_allowed() -> Option<usize> {
        Some(1)
    }
}

impl Decodable for i16 {
    fn decode_value(bf: &mut Bufferfish) -> Result<i16, BufferfishError> {
        bf.read_i16()
    }

    fn min_bytes_required() -> Option<usize> {
        Some(2)
    }

    fn max_bytes_allowed() -> Option<usize> {
        Some(2)
    }
}

impl Decodable for i32 {
    fn decode_value(bf: &mut Bufferfish) -> Result<i32, BufferfishError> {
        bf.read_i32()
    }

    fn min_bytes_required() -> Option<usize> {
        Some(4)
    }

    fn max_bytes_allowed() -> Option<usize> {
        Some(4)
    }
}

impl Decodable for i64 {
    fn decode_value(bf: &mut Bufferfish) -> Result<i64, BufferfishError> {
        bf.read_i64()
    }

    fn min_bytes_required() -> Option<usize> {
        Some(8)
    }

    fn max_bytes_allowed() -> Option<usize> {
        Some(8)
    }
}

impl Decodable for i128 {
    fn decode_value(bf: &mut Bufferfish) -> Result<i128, BufferfishError> {
        bf.read_i128()
    }

    fn min_bytes_required() -> Option<usize> {
        Some(16)
    }

    fn max_bytes_allowed() -> Option<usize> {
        Some(16)
    }
}

impl Decodable for bool {
    fn decode_value(bf: &mut Bufferfish) -> Result<bool, BufferfishError> {
        bf.read_bool()
    }

    fn min_bytes_required() -> Option<usize> {
        Some(1)
    }

    fn max_bytes_allowed() -> Option<usize> {
        Some(1)
    }
}

impl Decodable for String {
    fn decode_value(bf: &mut Bufferfish) -> Result<String, BufferfishError> {
        bf.read_string()
    }

    fn min_bytes_required() -> Option<usize> {
        Some(2)
    }

    fn max_bytes_allowed() -> Option<usize> {
        Some(u16::MAX as usize + 2)
    }
}

impl<T: Decodable> Decodable for Vec<T> {
    fn decode_value(bf: &mut Bufferfish) -> Result<Vec<T>, BufferfishError> {
        let len = bf.read_u16()? as usize;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(T::decode_value(bf)?);
        }

        Ok(vec)
    }

    fn min_bytes_required() -> Option<usize> {
        T::min_bytes_required().map(|min_t_size| 2 + (u16::MAX as usize * min_t_size))
    }

    fn max_bytes_allowed() -> Option<usize> {
        T::max_bytes_allowed().map(|max_t_size| 2 + (u16::MAX as usize * max_t_size))
    }
}

impl<T: Decodable> Decodable for Option<T> {
    fn decode_value(bf: &mut Bufferfish) -> Result<Option<T>, BufferfishError> {
        let flag = bf.read_u8()?;
        match flag {
            0 => Ok(None),
            1 => Ok(Some(T::decode_value(bf)?)),
            _ => Err(BufferfishError::InvalidEnumVariant),
        }
    }

    fn min_bytes_required() -> Option<usize> {
        Some(1)
    }

    fn max_bytes_allowed() -> Option<usize> {
        T::max_bytes_allowed().map(|max_t_size| 1 + max_t_size)
    }
}
