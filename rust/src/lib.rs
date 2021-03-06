use std::io::{Cursor, Read, Seek, Write};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Default)]
pub struct Bufferfish {
    inner: Cursor<Vec<u8>>,
    reading: bool,
    capacity: usize,
}

impl Bufferfish {
    pub fn new() -> Self {
        Self {
            inner: Cursor::new(Vec::with_capacity(12)),
            reading: false,
            capacity: 1024,
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
        for (i, c) in inner.iter().enumerate() {
            #[cfg(feature = "pretty-print")]
            let width = UnicodeWidthStr::width(c.to_string().as_str());

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

impl PartialEq for Bufferfish {
    fn eq(&self, other: &Self) -> bool {
        self.inner.get_ref() == other.inner.get_ref()
    }
}

impl From<Vec<u8>> for Bufferfish {
    fn from(vec: Vec<u8>) -> Self {
        Self {
            inner: Cursor::new(vec),
            reading: false,
            capacity: 1024,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<u8>> for Bufferfish {
    fn into(self) -> Vec<u8> {
        self.inner.into_inner()
    }
}

impl Write for Bufferfish {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.len() > self.capacity || self.as_ref().len() + buf.len() > self.capacity {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Bufferfish is full",
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

    /// Set the max capacity (in bytes) for the internal buffer.
    pub fn set_max_capacity(&mut self, capacity: usize) {
        if capacity < 1 {
            panic!("Max capacity must be at least 1 byte");
        }

        self.capacity = capacity;
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
        let slice = &mut self.inner.get_mut()[pos..pos + len];
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
        let slice = &mut self.inner.get_mut()[pos..pos + size];
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
        let slice = &mut self.inner.get_mut()[pos..];
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

#[cfg(test)]
mod tests {
    use super::*;

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
        buf.write_all(&[0u8; 1024]).unwrap();

        assert!(buf.write_u8(0).is_err());
    }

    #[test]
    fn test_oversized_write_buffer() {
        let mut buf = Bufferfish::new();

        assert!(buf.write(&[0u8; 1025]).is_err());
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
        buf.write_string("???????????????").unwrap();

        assert_eq!(
            buf.as_ref(),
            &[0, 15, 236, 149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184, 236, 154, 148]
        )
    }

    #[test]
    fn test_write_multiple_strings() {
        let mut buf = Bufferfish::new();
        buf.write_string("Bufferfish").unwrap();
        buf.write_string("???????????????").unwrap();

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
}
