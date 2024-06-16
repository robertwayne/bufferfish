pub mod decodable;
pub mod encodable;

use std::{
    convert::TryFrom,
    io::{Cursor, Read, Seek, Write},
    sync::Arc,
};

pub use encodable::Encodable;

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

impl std::error::Error for BufferfishError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BufferfishError::FailedWrite(e) => Some(e),
        }
    }
}

#[derive(Debug, Default)]
pub struct Bufferfish {
    inner: Cursor<Vec<u8>>,
    reading: bool,
    capacity: usize,
}

impl Write for Bufferfish {
    fn write(&mut self, bf: &[u8]) -> std::io::Result<usize> {
        if self.capacity > 0
            && (bf.len() >= self.capacity || self.as_ref().len() + bf.len() > self.capacity)
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "write of {} bytes exceeds the max capacity of {} bytes on this Bufferfish",
                    bf.len(),
                    self.capacity
                ),
            ));
        }

        self.reading = false;
        self.inner.write(bf)
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
    /// Creates a new `Bufferfish` with a default max capacity (1024 bytes).
    pub fn new() -> Self {
        Self {
            inner: Cursor::new(Vec::new()),
            reading: false,
            capacity: 1024,
        }
    }

    /// Creates a new `Bufferfish` with a max capacity (in bytes).
    /// A value of 0 will allow the buffer to grow indefinitely.
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

    /// Returns a `Vec<u8>` of the internal byte buffer.
    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.get_ref().to_vec()
    }

    /// Returns an `Arc<[u8]>` of the internal byte buffer for cheaply cloning
    /// and sharing the buffer.
    pub fn as_bytes(&self) -> Arc<[u8]> {
        self.inner.get_ref().clone().into()
    }

    /// Set the max capacity (in bytes) for the internal buffer.
    /// A value of 0 will allow the buffer to grow indefinitely.
    pub fn set_max_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
    }

    /// Adds a `Bufferfish` or `Vec<u8>` to the end of the buffer.
    /// See `try_extends` for a version that returns a `Result`.
    ///
    /// # Panics
    /// Panics if the buffer is at max capacity.
    pub fn extend<T: Into<Bufferfish>>(&mut self, other: T) {
        self.try_extend(other).unwrap();
    }

    /// Adds a `Bufferfish` or `Vec<u8>` to the end of the buffer.
    /// Returns a `Result` if the buffer is at max capacity.
    pub fn try_extend<T: Into<Bufferfish>>(&mut self, other: T) -> Result<(), BufferfishError> {
        let other = other.into();
        self.write_all(other.as_ref())?;

        Ok(())
    }

    /// Returns the next byte in the buffer without advancing the cursor.
    /// Returns a `Result` if the cursor is at the end of the buffer.
    pub fn peek(&mut self) -> Result<u8, BufferfishError> {
        self.start_reading();
        let pos = self.inner.position();

        let Some(byte) = self.inner.get_ref().get(pos as usize) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "peek of 1 byte exceeds the max capacity of {} bytes on this Bufferfish",
                    self.capacity
                ),
            ))?;
        };

        let byte = *byte;

        self.inner.set_position(pos);

        Ok(byte)
    }

    /// Returns the next n-bytes in the buffer without advancing the cursor.
    /// Returns a Result if the cursor is at the end of the buffer.
    pub fn peek_n(&mut self, n: usize) -> Result<Vec<u8>, BufferfishError> {
        self.start_reading();
        let pos = self.inner.position();

        let Some(bytes) = self.inner.get_ref().get(pos as usize..pos as usize + n) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "peek of {} bytes exceeds the max capacity of {} bytes on this Bufferfish",
                    n, self.capacity
                ),
            ))?;
        };

        let bytes = bytes.to_vec();

        self.inner.set_position(pos);

        Ok(bytes)
    }

    /// Writes a u8 to the buffer as one byte.
    pub fn write_u8(&mut self, value: u8) -> Result<(), BufferfishError> {
        self.write_all(&[value])?;

        Ok(())
    }

    /// Writes a u16 to the buffer as two bytes.
    pub fn write_u16(&mut self, value: u16) -> Result<(), BufferfishError> {
        self.write_all(&value.to_be_bytes())?;

        Ok(())
    }

    /// Writes a u32 to the buffer as four bytes.
    pub fn write_u32(&mut self, value: u32) -> Result<(), BufferfishError> {
        self.write_all(&value.to_be_bytes())?;

        Ok(())
    }

    /// Writes an i8 to the buffer as one byte.
    pub fn write_i8(&mut self, value: i8) -> Result<(), BufferfishError> {
        self.write_all(&[value as u8])?;

        Ok(())
    }

    /// Writes an i16 to the buffer as two bytes.
    pub fn write_i16(&mut self, value: i16) -> Result<(), BufferfishError> {
        self.write_all(&value.to_be_bytes())?;

        Ok(())
    }

    /// Writes an i32 to the buffer as four bytes.
    pub fn write_i32(&mut self, value: i32) -> Result<(), BufferfishError> {
        self.write_all(&value.to_be_bytes())?;

        Ok(())
    }

    /// Writes a bool to the buffer as one byte.
    pub fn write_bool(&mut self, value: bool) -> Result<(), BufferfishError> {
        self.write_u8(if value { 1 } else { 0 })?;

        Ok(())
    }

    /// Writes a packed array of booleans to the buffer as a single byte.
    /// Can pack up to 8 booleans into a single byte.
    pub fn write_packed_bools(&mut self, values: &[bool]) -> Result<(), BufferfishError> {
        if values.len() > 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Cannot pack more than 8 booleans into a single byte.",
            ))?;
        }

        let mut packed = 0u8;

        for (i, value) in values.iter().enumerate() {
            if *value {
                packed |= 1 << (7 - i); // Pack from most significant bit to least significant bit
            }
        }

        self.write_u8(packed)?;

        Ok(())
    }

    /// Writes a variable length string to the buffer. It will be prefixed with
    /// its length in bytes as a u16 (two bytes).
    pub fn write_string(&mut self, value: &str) -> Result<(), BufferfishError> {
        let len = u16::try_from(value.len()).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "String length exceeds u16 max value",
            )
        })?;

        self.write_u16(len)?;
        self.write_all(value.as_bytes())?;

        Ok(())
    }

    /// Writes an array to the buffer, where the items implement the Encodable
    /// trait. The array will be prefixed with its length as a u16 (two bytes).
    pub fn write_array<T: Encodable>(&mut self, vec: &[T]) -> Result<(), BufferfishError> {
        self.write_u16(vec.len() as u16)?;

        for item in vec {
            item.encode(self)?;
        }

        Ok(())
    }

    /// Writes an array of raw bytes to the buffer. Useful for encoding
    /// distinct structs into byte arrays and appending them to a buffer later.
    pub fn write_raw_bytes(&mut self, bytes: &[u8]) -> Result<(), BufferfishError> {
        self.write_all(bytes)?;
        Ok(())
    }

    /// Reads a u8 from the buffer.
    pub fn read_u8(&mut self) -> Result<u8, BufferfishError> {
        self.start_reading();

        let mut bf = [0u8; 1];
        self.inner.read_exact(&mut bf)?;

        Ok(bf[0])
    }

    /// Reads a u16 from the buffer.
    pub fn read_u16(&mut self) -> Result<u16, BufferfishError> {
        self.start_reading();

        let mut bf = [0u8; 2];
        self.inner.read_exact(&mut bf)?;

        Ok(u16::from_be_bytes(bf))
    }

    /// Reads a u32 from the buffer.
    pub fn read_u32(&mut self) -> Result<u32, BufferfishError> {
        self.start_reading();

        let mut bf = [0u8; 4];
        self.inner.read_exact(&mut bf)?;

        Ok(u32::from_be_bytes(bf))
    }

    /// Reads an i8 from the buffer.
    pub fn read_i8(&mut self) -> Result<i8, BufferfishError> {
        self.start_reading();

        let mut bf = [0u8; 1];
        self.inner.read_exact(&mut bf)?;

        Ok(i8::from_be_bytes(bf))
    }

    /// Reads an i16 from the buffer.
    pub fn read_i16(&mut self) -> Result<i16, BufferfishError> {
        self.start_reading();

        let mut bf = [0u8; 2];
        self.inner.read_exact(&mut bf)?;

        Ok(i16::from_be_bytes(bf))
    }

    /// Reads an i32 from the buffer.
    pub fn read_i32(&mut self) -> Result<i32, BufferfishError> {
        self.start_reading();

        let mut bf = [0u8; 4];
        self.inner.read_exact(&mut bf)?;

        Ok(i32::from_be_bytes(bf))
    }

    /// Reads a bool from the buffer.
    pub fn read_bool(&mut self) -> Result<bool, BufferfishError> {
        let value = self.read_u8()?;

        Ok(value != 0)
    }

    /// Attempts to read a packed array of booleans from the buffer.
    /// You must specify the number of booleans to read.
    pub fn read_packed_bools(&mut self, count: u8) -> Result<Vec<bool>, BufferfishError> {
        if count > 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Cannot pack more than 8 booleans into a single byte.",
            ))?;
        }

        let packed = self.read_u8()?;
        let mut bools = Vec::with_capacity(count as usize);

        for i in 0..count {
            bools.push(packed & (1 << (7 - i)) != 0);
        }

        Ok(bools)
    }

    /// Reads a variable length string from the buffer.
    pub fn read_string(&mut self) -> Result<String, BufferfishError> {
        self.start_reading();

        let len = self.read_u16()? as usize;
        let pos = self.inner.position() as usize;
        self.inner.set_position((pos + len) as u64);

        let Some(slice) = &mut self.inner.get_mut().get(pos..pos + len) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Unexpected EOF",
            ))?;
        };

        let string = String::from_utf8(slice.to_vec());

        match string {
            Ok(s) => Ok(s),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            ))?,
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
