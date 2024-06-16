use crate::{Bufferfish, BufferfishError};

pub trait Encodable {
    fn encode(&self, bf: &mut Bufferfish) -> std::io::Result<()>;
    fn to_bufferfish(&self) -> Result<Bufferfish, BufferfishError> {
        let mut bf = Bufferfish::new();
        self.encode(&mut bf)?;

        Ok(bf)
    }
}

impl Encodable for u8 {
    fn encode(&self, bf: &mut Bufferfish) -> std::io::Result<()> {
        bf.write_u8(*self)
    }
}

impl Encodable for u16 {
    fn encode(&self, bf: &mut Bufferfish) -> std::io::Result<()> {
        bf.write_u16(*self)
    }
}

impl Encodable for u32 {
    fn encode(&self, bf: &mut Bufferfish) -> std::io::Result<()> {
        bf.write_u32(*self)
    }
}

impl Encodable for i8 {
    fn encode(&self, bf: &mut Bufferfish) -> std::io::Result<()> {
        bf.write_i8(*self)
    }
}

impl Encodable for i16 {
    fn encode(&self, bf: &mut Bufferfish) -> std::io::Result<()> {
        bf.write_i16(*self)
    }
}

impl Encodable for i32 {
    fn encode(&self, bf: &mut Bufferfish) -> std::io::Result<()> {
        bf.write_i32(*self)
    }
}

impl Encodable for bool {
    fn encode(&self, bf: &mut Bufferfish) -> std::io::Result<()> {
        bf.write_bool(*self)
    }
}

impl Encodable for String {
    fn encode(&self, bf: &mut Bufferfish) -> std::io::Result<()> {
        bf.write_string(self)
    }
}
