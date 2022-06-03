use crate::Endianness;
use std::io::{Result, Write};

/// This trait is to write an endianness fixed-length bytes.
pub trait WriteEndianness {
    fn write_endianness_with_callback<F, W: Write>(
        &self,
        writer: &mut W,
        callback: F,
        endianness: Endianness,
    ) -> Result<()>
    where
        F: Fn(&mut [u8]);

    /// This method writes bytes in big-endian byte order.
    fn write_be_bytes<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.write_endianness_with_callback(writer, |_| {}, Endianness::BE)
    }
    /// This method writes bytes in little-endian byte order.
    fn write_le_bytes<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.write_endianness_with_callback(writer, |_| {}, Endianness::LE)
    }
    /// This method writes bytes in little-endian byte order.
    ///
    /// As the target platform’s native endianness is used, portable code should use write_be_bytes or write_le_bytes, as appropriate, instead.
    fn write_ne_bytes<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.write_endianness_with_callback(writer, |_| {}, Endianness::NE)
    }
}

macro_rules! write_endianness_impl {
    ( $( $t:ty ),* ) => ($(
        impl WriteEndianness for $t {
            fn write_endianness_with_callback<F, W: Write>(&self, writer: &mut W, callback: F,
                endianness: Endianness) -> Result<()> where F: Fn(&mut [u8]) {
                let bytes = &mut match endianness {
                    Endianness::BE => self.to_be_bytes(),
                    Endianness::LE => self.to_le_bytes(),
                    Endianness::NE => self.to_ne_bytes(),
                };
                callback(bytes);
                writer.write_all(bytes)?;
                Ok(())
            }
        }
    )*)
}

write_endianness_impl!(f32, f64);
write_endianness_impl!(isize, i8, i16, i32, i64, i128);
write_endianness_impl!(usize, u8, u16, u32, u64, u128);

pub trait WriteRef {
    fn write_ref_bytes_with_callback<F, W: Write>(&self, writer: &mut W, callback: F) -> Result<()>
    where
        F: Fn(&[u8]);

    fn write_ref_bytes<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.write_ref_bytes_with_callback(writer, |_| {})
    }
}
impl WriteRef for [u8] {
    fn write_ref_bytes_with_callback<F, W: Write>(&self, writer: &mut W, callback: F) -> Result<()>
    where
        F: Fn(&[u8]),
    {
        callback(self);
        writer.write_all(self)?;
        Ok(())
    }
}
impl WriteRef for str {
    fn write_ref_bytes_with_callback<F, W: Write>(&self, writer: &mut W, callback: F) -> Result<()>
    where
        F: Fn(&[u8]),
    {
        let bytes = self.as_bytes();
        callback(bytes);
        writer.write_all(bytes)?;
        Ok(())
    }
}
