use std::{
    convert::TryInto,
    io::{Cursor, Read, Seek, Write},
    sync::Arc,
};

#[cfg(feature = "macros")]
pub use bufferfish_derive::Serialize;

#[derive(Debug)]
pub enum BufferfishError {
    FailedWrite(std::io::Error),
}

impl std::fmt::Display for BufferfishError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BufferfishError::FailedWrite(e) => write!(f, "Failed to write to buffer: {}", e),
        }
    }
}

impl From<std::io::Error> for BufferfishError {
    fn from(e: std::io::Error) -> Self {
        BufferfishError::FailedWrite(e)
    }
}

impl std::error::Error for BufferfishError {}

pub trait ToBufferfish {
    fn to_bufferfish(&self) -> Result<Bufferfish, BufferfishError>;
}

#[derive(Debug, Default)]
pub struct Bufferfish {
    inner: Cursor<Vec<u8>>,
    reading: bool,
    capacity: usize,
}

impl Write for Bufferfish {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.capacity > 0
            && (buf.len() >= self.capacity || self.as_ref().len() + buf.len() > self.capacity)
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Write of {} bytes exceeds the max capacity of {} bytes on this Bufferfish.",
                    buf.len(),
                    self.capacity
                ),
            ));
        }

        self.reading = false;
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl Seek for Bufferfish {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }
}

impl Bufferfish {
    /// Creates a new Bufferfish with a default max capacity (1024 bytes).
    pub fn new() -> Self {
        Self {
            inner: Cursor::new(Vec::new()),
            reading: false,
            capacity: 1024,
        }
    }

    /// Creates a new Bufferfish with a max capacity (in bytes).
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Cursor::new(Vec::with_capacity(capacity)),
            reading: false,
            capacity,
        }
    }

    /// Returns the current length (bytes) of the buffer.
    pub fn len(&self) -> usize {
        self.inner.get_ref().len()
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.get_ref().is_empty()
    }

    /// #[doc(hidden)]
    /// Resets the buffer cursor to the start position when reading after a
    /// write.
    fn start_reading(&mut self) {
        if self.reading {
            return;
        }

        self.inner.set_position(0);
        self.reading = true;
    }

    /// Returns a Vec<u8> of the internal byte buffer.
    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.get_ref().to_vec()
    }

    /// Returns an Arc<[u8]> of the internal byte buffer for cheaply cloning and
    /// sharing the buffer.
    pub fn as_bytes(&self) -> Arc<[u8]> {
        self.inner.get_ref().clone().into()
    }

    /// Set the max capacity (in bytes) for the internal buffer.
    /// A value of 0 will allow the buffer to grow indefinitely.
    pub fn set_max_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
    }

    /// Adds a Bufferfish or Vec<u8> to the end of the buffer.
    /// See `try_extends` for a version that returns a Result.
    pub fn extend<T: Into<Bufferfish>>(&mut self, other: T) {
        self.try_extend(other).unwrap();
    }

    /// Adds a Bufferfish or Vec<u8> to the end of the buffer.
    /// Returns a Result if the buffer is at max capacity.
    pub fn try_extend<T: Into<Bufferfish>>(&mut self, other: T) -> std::io::Result<()> {
        let other = other.into();
        self.write_all(other.as_ref())?;

        Ok(())
    }

    /// Returns the next byte in the buffer without advancing the cursor.
    /// Returns a Result if the cursor is at the end of the buffer.
    pub fn peek(&mut self) -> std::io::Result<u8> {
        self.start_reading();
        let pos = self.inner.position();

        let Some(byte) = self.inner.get_ref().get(pos as usize) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Peek of 1 byte exceeds the max capacity of {} bytes on this Bufferfish.",
                    self.capacity
                ),
            ));
        };

        let byte = *byte;

        self.inner.set_position(pos);

        Ok(byte)
    }

    /// Returns the next n-bytes in the buffer without advancing the cursor.
    /// Returns a Result if the cursor is at the end of the buffer.
    pub fn peek_n(&mut self, n: usize) -> std::io::Result<Vec<u8>> {
        self.start_reading();
        let pos = self.inner.position();

        let Some(bytes) = self.inner.get_ref().get(pos as usize..pos as usize + n) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Peek of {} bytes exceeds the max capacity of {} bytes on this Bufferfish.",
                    n, self.capacity
                ),
            ));
        };

        let bytes = bytes.to_vec();

        self.inner.set_position(pos);

        Ok(bytes)
    }

    /// Writes a u8 to the buffer as one byte.
    pub fn write_u8(&mut self, value: u8) -> std::io::Result<()> {
        self.write_all(&[value])?;
        Ok(())
    }

    /// Writes a u16 to the buffer as two bytes.
    pub fn write_u16(&mut self, value: u16) -> std::io::Result<()> {
        self.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    /// Writes a u32 to the buffer as four bytes.
    pub fn write_u32(&mut self, value: u32) -> std::io::Result<()> {
        self.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    /// Writes an i8 to the buffer as one byte.
    pub fn write_i8(&mut self, value: i8) -> std::io::Result<()> {
        self.write_all(&[value as u8])?;
        Ok(())
    }

    /// Writes an i16 to the buffer as two bytes.
    pub fn write_i16(&mut self, value: i16) -> std::io::Result<()> {
        self.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    /// Writes an i32 to the buffer as four bytes.
    pub fn write_i32(&mut self, value: i32) -> std::io::Result<()> {
        self.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    /// Writes a bool to the buffer as one byte.
    pub fn write_bool(&mut self, value: bool) -> std::io::Result<()> {
        self.write_u8(if value { 1 } else { 0 })?;
        Ok(())
    }

    /// Writes a series of bools to the buffer as one byte. This allows up to 4
    /// bools to be represented as a single byte. The first 4 bits are used as a
    /// mask to determine which of the last 4 bits are set.
    pub fn write_packed_bools(&mut self, values: &[bool]) -> std::io::Result<()> {
        if values.len() > 4 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Each packed bool can only represent 4 or fewer values",
            ));
        }

        let mut packed_value = 0x00;
        for value in values {
            packed_value <<= 1;
            if *value {
                packed_value |= 1;
            }
        }
        self.write_u8(packed_value)?;

        Ok(())
    }

    /// Writes a variable length string to the buffer. It will be prefixed with
    /// its length in bytes as a u16 (two bytes).
    pub fn write_string(&mut self, value: &str) -> std::io::Result<()> {
        self.write_u16(value.len().try_into().unwrap())?;
        self.write_all(value.as_bytes())?;
        Ok(())
    }

    /// Writes a string to the buffer without a length prefix.
    pub fn write_sized_string(&mut self, value: &str) -> std::io::Result<()> {
        self.write_all(value.as_bytes())?;
        Ok(())
    }

    /// Writes an array of raw bytes to the buffer. Useful for serializing
    /// distinct structs into byte arrays and appending them to a buffer later.
    pub fn write_raw_bytes(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        self.write_all(bytes)?;
        Ok(())
    }

    /// Reads a u8 from the buffer.
    pub fn read_u8(&mut self) -> std::io::Result<u8> {
        self.start_reading();

        let mut buf = [0u8; 1];
        self.inner.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Reads a u16 from the buffer.
    pub fn read_u16(&mut self) -> std::io::Result<u16> {
        self.start_reading();

        let mut buf = [0u8; 2];
        self.inner.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    /// Reads a u32 from the buffer.
    pub fn read_u32(&mut self) -> std::io::Result<u32> {
        self.start_reading();

        let mut buf = [0u8; 4];
        self.inner.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    /// Reads an i8 from the buffer.
    pub fn read_i8(&mut self) -> std::io::Result<i8> {
        self.start_reading();

        let mut buf = [0u8; 1];
        self.inner.read_exact(&mut buf)?;
        Ok(i8::from_be_bytes(buf))
    }

    /// Reads an i16 from the buffer.
    pub fn read_i16(&mut self) -> std::io::Result<i16> {
        self.start_reading();

        let mut buf = [0u8; 2];
        self.inner.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    /// Reads an i32 from the buffer.
    pub fn read_i32(&mut self) -> std::io::Result<i32> {
        self.start_reading();

        let mut buf = [0u8; 4];
        self.inner.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    /// Reads a bool from the buffer.
    pub fn read_bool(&mut self) -> std::io::Result<bool> {
        let value = self.read_u8()?;
        Ok(value != 0)
    }

    /// Reads a series of bools from the buffer as a vector of bytes.
    pub fn read_packed_bools(&mut self) -> std::io::Result<Vec<bool>> {
        todo!()
    }

    /// Reads a variable length string from the buffer.
    pub fn read_string(&mut self) -> std::io::Result<String> {
        self.start_reading();

        let len = self.read_u16()? as usize;
        let pos = self.inner.position() as usize;
        self.inner.set_position((pos + len) as u64);

        let Some(slice) = &mut self.inner.get_mut().get(pos..pos + len) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Unexpected EOF",
            ));
        };

        let string = String::from_utf8(slice.to_vec());

        match string {
            Ok(s) => Ok(s),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            )),
        }
    }

    /// Reads a sized string from the buffer. You must pass the length of the
    /// string in bytes.
    pub fn read_sized_string(&mut self, size: usize) -> std::io::Result<String> {
        self.start_reading();

        let pos = self.inner.position() as usize;
        self.inner.set_position((pos + size) as u64);

        let Some(slice) = &mut self.inner.get_mut().get(pos..pos + size) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Unexpected EOF",
            ));
        };

        let string = String::from_utf8(slice.to_vec());

        match string {
            Ok(s) => Ok(s),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            )),
        }
    }

    /// Reads a sized string from the buffer. This will read from the buffers
    /// current position until the end of the buffer, so this function should
    /// not be used unless you know that the string is the last value in the
    /// buffer. This removes the overhead of a length prefix; it is recommended
    /// to plan your packets out such that they end with a sized string where
    /// possible.
    pub fn read_string_remaining(&mut self) -> std::io::Result<String> {
        self.start_reading();

        let pos = self.inner.position() as usize;
        self.inner.set_position(self.inner.get_ref().len() as u64);

        let Some(slice) = &mut self.inner.get_mut().get(pos..) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Unexpected EOF",
            ));
        };

        let string = String::from_utf8(slice.to_vec());

        match string {
            Ok(s) => Ok(s),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            )),
        }
    }
}

impl std::fmt::Display for Bufferfish {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let inner = self.inner.get_ref();
        write!(f, " Byte: ")?;

        for val in inner {
            write!(f, " {} ", val)?;
        }

        write!(f, "\nIndex: ")?;
        #[allow(unused_variables)]
        for (i, c) in inner.iter().enumerate() {
            #[cfg(feature = "pretty-print")]
            let width = unicode_width::UnicodeWidthStr::width(c.to_string().as_str());

            #[cfg(not(feature = "pretty-print"))]
            let width = 1;

            write!(f, " {:width$} ", i, width = width)?;
        }

        Ok(())
    }
}

impl AsRef<[u8]> for Bufferfish {
    fn as_ref(&self) -> &[u8] {
        self.inner.get_ref()
    }
}

impl AsMut<[u8]> for Bufferfish {
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.get_mut()
    }
}

impl PartialEq for Bufferfish {
    fn eq(&self, other: &Self) -> bool {
        self.inner.get_ref() == other.inner.get_ref()
    }
}

impl From<&[u8]> for Bufferfish {
    fn from(slice: &[u8]) -> Self {
        Self {
            inner: Cursor::new(slice.to_vec()),
            reading: false,
            capacity: slice.len(),
        }
    }
}

impl From<Vec<u8>> for Bufferfish {
    fn from(vec: Vec<u8>) -> Self {
        let capacity = vec.len();
        Self {
            inner: Cursor::new(vec),
            reading: false,
            capacity,
        }
    }
}

impl From<Bufferfish> for Vec<u8> {
    fn from(buffer: Bufferfish) -> Self {
        buffer.inner.into_inner()
    }
}

impl From<bytes::Bytes> for Bufferfish {
    fn from(bytes: bytes::Bytes) -> Self {
        Self {
            inner: Cursor::new(bytes.to_vec()),
            reading: false,
            capacity: bytes.len(),
        }
    }
}

impl From<Bufferfish> for bytes::Bytes {
    fn from(buffer: Bufferfish) -> Self {
        buffer.inner.into_inner().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_one() {
        let mut buf = Bufferfish::new();
        buf.write_u8(0).unwrap();

        assert_eq!(buf.peek().unwrap(), 0);
        assert_eq!(buf.peek().unwrap(), 0);
    }

    #[test]
    fn peek_n() {
        let mut buf = Bufferfish::new();
        buf.write_u8(0).unwrap();
        buf.write_u8(1).unwrap();
        buf.write_u8(2).unwrap();

        assert_eq!(buf.peek_n(2).unwrap(), &[0, 1]);
        assert_eq!(buf.peek_n(2).unwrap(), &[0, 1]);
    }

    #[test]
    fn peek_one_past_capacity() {
        let mut buf = Bufferfish::new();

        let result = buf.peek();

        assert!(result.is_err());
    }

    #[test]
    fn peek_n_past_capacity() {
        let mut buf = Bufferfish::new();
        buf.write_u8(0).unwrap();

        let result = buf.peek_n(2);

        assert!(result.is_err());
    }

    #[test]
    fn test_extends_bufferfish() {
        let mut buf = Bufferfish::new();
        buf.write_u8(0).unwrap();

        let mut buf2 = Bufferfish::new();
        buf2.write_u8(1).unwrap();

        buf.extend(buf2);

        assert_eq!(buf.as_ref(), &[0, 1]);
    }

    #[test]
    fn test_extends_impls() {
        let mut buf = Bufferfish::new();
        buf.write_u8(0).unwrap();

        let slice: &[u8] = &[1];
        let vec = Vec::from([2]);

        buf.extend(slice);
        buf.extend(vec);

        assert_eq!(buf.as_ref(), &[0, 1, 2]);
    }

    #[test]
    fn test_write_u8() {
        let mut buf = Bufferfish::new();
        buf.write_u8(0).unwrap();
        buf.write_u8(255).unwrap();

        assert_eq!(buf.as_ref(), &[0, 255]);
    }

    #[test]
    fn test_write_u16() {
        let mut buf = Bufferfish::new();
        buf.write_u16(0).unwrap();
        buf.write_u16(12345).unwrap();
        buf.write_u16(65535).unwrap();

        assert_eq!(buf.as_ref(), &[0, 0, 48, 57, 255, 255]);
    }

    #[test]
    fn test_write_u32() {
        let mut buf = Bufferfish::new();
        buf.write_u32(0).unwrap();
        buf.write_u32(1234567890).unwrap();
        buf.write_u32(u32::max_value()).unwrap();

        assert_eq!(
            buf.as_ref(),
            &[0, 0, 0, 0, 73, 150, 2, 210, 255, 255, 255, 255]
        );
    }

    #[test]
    fn test_read_u8() {
        let mut buf = Bufferfish::new();
        buf.write_u8(0).unwrap();
        buf.write_u8(255).unwrap();

        assert_eq!(buf.read_u8().unwrap(), 0);
        assert_eq!(buf.read_u8().unwrap(), 255);
    }

    #[test]
    fn test_read_u16() {
        let mut buf = Bufferfish::new();
        buf.write_u16(0).unwrap();
        buf.write_u16(12345).unwrap();
        buf.write_u16(65535).unwrap();

        assert_eq!(buf.read_u16().unwrap(), 0);
        assert_eq!(buf.read_u16().unwrap(), 12345);
        assert_eq!(buf.read_u16().unwrap(), 65535);
    }

    #[test]
    fn test_read_u32() {
        let mut buf = Bufferfish::new();
        buf.write_u32(0).unwrap();
        buf.write_u32(1234567890).unwrap();
        buf.write_u32(u32::max_value()).unwrap();

        assert_eq!(buf.read_u32().unwrap(), 0);
        assert_eq!(buf.read_u32().unwrap(), 1234567890);
        assert_eq!(buf.read_u32().unwrap(), u32::max_value());
    }

    #[test]
    fn test_write_i8() {
        let mut buf = Bufferfish::new();
        buf.write_i8(0).unwrap();
        buf.write_i8(127).unwrap();
        buf.write_i8(-128).unwrap();

        assert_eq!(buf.as_ref(), &[0, 127, 128]);
    }

    #[test]
    fn test_write_i16() {
        let mut buf = Bufferfish::new();
        buf.write_i16(0).unwrap();
        buf.write_i16(12345).unwrap();
        buf.write_i16(32767).unwrap();
        buf.write_i16(-32768).unwrap();

        assert_eq!(buf.as_ref(), &[0, 0, 48, 57, 127, 255, 128, 0]);
    }

    #[test]
    fn test_write_i32() {
        let mut buf = Bufferfish::new();
        buf.write_i32(0).unwrap();
        buf.write_i32(1234567890).unwrap();
        buf.write_i32(2147483647).unwrap();
        buf.write_i32(-2147483648).unwrap();

        assert_eq!(
            buf.as_ref(),
            &[0, 0, 0, 0, 73, 150, 2, 210, 127, 255, 255, 255, 128, 0, 0, 0]
        );
    }

    #[test]
    fn test_read_i8() {
        let mut buf = Bufferfish::new();
        buf.write_i8(0).unwrap();
        buf.write_i8(127).unwrap();
        buf.write_i8(-128).unwrap();

        assert_eq!(buf.read_i8().unwrap(), 0);
        assert_eq!(buf.read_i8().unwrap(), 127);
        assert_eq!(buf.read_i8().unwrap(), -128);
    }

    #[test]
    fn test_read_i16() {
        let mut buf = Bufferfish::new();
        buf.write_i16(0).unwrap();
        buf.write_i16(12345).unwrap();
        buf.write_i16(32767).unwrap();
        buf.write_i16(-32768).unwrap();

        assert_eq!(buf.read_i16().unwrap(), 0);
        assert_eq!(buf.read_i16().unwrap(), 12345);
        assert_eq!(buf.read_i16().unwrap(), 32767);
        assert_eq!(buf.read_i16().unwrap(), -32768);
    }

    #[test]
    fn test_read_i32() {
        let mut buf = Bufferfish::new();
        buf.write_i32(0).unwrap();
        buf.write_i32(1234567890).unwrap();
        buf.write_i32(2147483647).unwrap();
        buf.write_i32(-2147483648).unwrap();

        assert_eq!(buf.read_i32().unwrap(), 0);
        assert_eq!(buf.read_i32().unwrap(), 1234567890);
        assert_eq!(buf.read_i32().unwrap(), 2147483647);
        assert_eq!(buf.read_i32().unwrap(), -2147483648);
    }

    #[test]
    fn test_read_reset() {
        let mut buf = Bufferfish::new();
        buf.write_u8(0).unwrap();
        buf.read_u8().unwrap();
        buf.write_u8(255).unwrap();

        assert_eq!(buf.read_u8().unwrap(), 0);
    }

    #[test]
    fn test_bufferfish_overflow() {
        let mut buf = Bufferfish::new();
        buf.write_all(&[0u8; 1023]).unwrap();

        let result = buf.write_u8(0);
        assert!(result.is_ok());

        let result = buf.write_u8(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_oversized_write_buffer() {
        let mut buf = Bufferfish::new();

        assert!(buf.write(&[0u8; 1025]).is_err());
    }

    #[test]
    fn test_unbounded_capacity() {
        let mut buf = Bufferfish::with_capacity(0);

        assert!(buf.write(&[0u8; 2000]).is_ok());
    }

    #[test]
    fn test_write_string() {
        let mut buf = Bufferfish::new();
        buf.write_string("Bufferfish").unwrap();

        assert_eq!(
            buf.as_ref(),
            &[0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104]
        );
    }

    #[test]
    fn test_write_string_big_chars() {
        let mut buf = Bufferfish::new();
        buf.write_string("안녕하세요").unwrap();

        assert_eq!(
            buf.as_ref(),
            &[0, 15, 236, 149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184, 236, 154, 148]
        )
    }

    #[test]
    fn test_write_multiple_strings() {
        let mut buf = Bufferfish::new();
        buf.write_string("Bufferfish").unwrap();
        buf.write_string("안녕하세요").unwrap();

        assert_eq!(
            buf.as_ref(),
            &[
                0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104, 0, 15, 236, 149, 136, 235,
                133, 149, 237, 149, 152, 236, 132, 184, 236, 154, 148
            ]
        );
    }

    #[test]
    fn test_write_fixed_string() {
        let mut buf = Bufferfish::new();
        buf.write_sized_string("Bufferfish").unwrap();

        assert_eq!(
            buf.as_ref(),
            &[66, 117, 102, 102, 101, 114, 102, 105, 115, 104]
        );
    }

    #[test]
    fn test_read_string() {
        let mut buf = Bufferfish::new();
        buf.write_string("Bufferfish").unwrap();

        assert_eq!(buf.read_string().unwrap(), "Bufferfish");
    }

    #[test]
    fn test_read_sized_string() {
        let mut buf = Bufferfish::new();
        buf.write_sized_string("Bufferfish").unwrap();

        assert_eq!(buf.read_sized_string(10).unwrap(), "Bufferfish");
    }

    #[test]
    fn test_write_bool() {
        let mut buf = Bufferfish::new();
        buf.write_bool(true).unwrap();
        buf.write_bool(false).unwrap();

        assert_eq!(buf.as_ref(), &[1, 0]);
    }

    #[test]
    fn test_write_packed_bools() {
        let mut buf = Bufferfish::new();
        buf.write_packed_bools(&[true, false, true, true]).unwrap();
        buf.write_packed_bools(&[false, false, true, false])
            .unwrap();

        assert_eq!(buf.as_ref(), &[11, 2]);
    }

    #[test]
    fn test_read_bool() {
        let mut buf = Bufferfish::new();
        buf.write_bool(true).unwrap();
        buf.write_bool(false).unwrap();

        assert!(buf.read_bool().unwrap());
        assert!(!buf.read_bool().unwrap());
    }

    #[test]
    fn test_read_packed_bools() {}

    #[test]
    // This is just a visual test for ensuring pretty-formatting on output.
    // Must be run with `cargo test -- --show-output` to see the string.
    fn test_display_trait() {
        let mut buf = Bufferfish::new();
        buf.write_u16(4).unwrap();
        buf.write_string("Bufferfish").unwrap();

        // Should look like this:
        //  Byte:  0  4  0  10  66  117  102  102  101  114  102  105  115  104
        // Index:  0  1  2   3   4    5    6    7    8    9   10   11   12   13
        println!("{}", buf);
    }

    #[test]
    fn test_write_raw_bytes() {
        let mut buf = Bufferfish::new();
        buf.write_string("Bufferfish").unwrap();

        let mut buf2 = Bufferfish::new();
        buf2.write_string("안녕하세요").unwrap();

        buf.write_raw_bytes(buf2.as_ref()).unwrap();

        assert!(buf.read_string().unwrap() == "Bufferfish");
        assert!(buf.read_string().unwrap() == "안녕하세요");
    }
}
