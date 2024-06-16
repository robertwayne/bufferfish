use crate::{Bufferfish, BufferfishError};

pub trait Decodable {
    fn decode(&mut self, bf: &mut Bufferfish) -> Result<(), BufferfishError>;
}

impl Decodable for u8 {
    fn decode(&mut self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        *self = bf.read_u8()?;
        Ok(())
    }
}

impl Decodable for u16 {
    fn decode(&mut self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        *self = bf.read_u16()?;
        Ok(())
    }
}

impl Decodable for u32 {
    fn decode(&mut self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        *self = bf.read_u32()?;
        Ok(())
    }
}

impl Decodable for i8 {
    fn decode(&mut self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        *self = bf.read_i8()?;
        Ok(())
    }
}

impl Decodable for i16 {
    fn decode(&mut self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        *self = bf.read_i16()?;
        Ok(())
    }
}

impl Decodable for i32 {
    fn decode(&mut self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        *self = bf.read_i32()?;
        Ok(())
    }
}

impl Decodable for bool {
    fn decode(&mut self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        *self = bf.read_bool()?;
        Ok(())
    }
}

impl Decodable for String {
    fn decode(&mut self, bf: &mut Bufferfish) -> Result<(), BufferfishError> {
        *self = bf.read_string()?;
        Ok(())
    }
}
