use crate::Endianness;
use std::io::{Error, ErrorKind, Read, Result};
use std::mem;

/// This trait is to read an endianness fixed-length bytes.
pub trait ReadEndianness: Sized {
    fn read_endianness_bytes_with_callback<F, R: Read>(
        reader: &mut R,
        callback: F,
        endianness: Endianness,
    ) -> Result<Self>
    where
        F: Fn(&mut [u8]);

    /// This method reads bytes in big-endian byte order.
    fn read_be_bytes<R: Read>(reader: &mut R) -> Result<Self> {
        Self::read_endianness_bytes_with_callback(reader, |_| {}, Endianness::BE)
    }
    /// This method reads bytes in little-endian byte order.
    fn read_le_bytes<R: Read>(reader: &mut R) -> Result<Self> {
        Self::read_endianness_bytes_with_callback(reader, |_| {}, Endianness::LE)
    }
    /// This method reads bytes in native-endian byte order.
    ///
    /// As the target platform’s native endianness is used, portable code should use read_be_bytes or read_le_bytes, as appropriate, instead.
    fn read_ne_bytes<R: Read>(reader: &mut R) -> Result<Self> {
        Self::read_endianness_bytes_with_callback(reader, |_| {}, Endianness::NE)
    }
}

macro_rules! read_endianness_impl {
    ( $( $t:ty ),* ) => ($(
        impl ReadEndianness for $t {
            fn read_endianness_bytes_with_callback<F, R: Read>(reader: &mut R, callback: F, endianness: Endianness) -> Result<Self> where F: Fn(&mut [u8]) {
                let mut buf = [0; mem::size_of::<$t>()];
                reader.read_exact(&mut buf)?;
                callback(&mut buf);
                Ok(
                match endianness {
                    Endianness::BE => <$t>::from_be_bytes(buf),
                    Endianness::LE => <$t>::from_le_bytes(buf),
                    Endianness::NE => <$t>::from_ne_bytes(buf),
                })
            }
        }
    )*)
}

read_endianness_impl!(f32, f64);
read_endianness_impl!(isize, i8, i16, i32, i64, i128);
read_endianness_impl!(usize, u8, u16, u32, u64, u128);

/// This trait is to read a variable-length array.
pub trait ReadVariable: Sized {
    fn read_variable_bytes_with_callback<F, R: Read>(
        reader: &mut R,
        callback: F,
        length: usize,
    ) -> Result<Self>
    where
        F: Fn(&mut [u8]);

    fn read_variable_bytes<R: Read>(reader: &mut R, length: usize) -> Result<Self> {
        Self::read_variable_bytes_with_callback(reader, |_| {}, length)
    }
}
impl ReadVariable for Vec<u8> {
    fn read_variable_bytes_with_callback<F, R: Read>(
        reader: &mut R,
        callback: F,
        length: usize,
    ) -> Result<Vec<u8>>
    where
        F: Fn(&mut [u8]),
    {
        let mut buf = vec![0; length];
        reader.read_exact(&mut buf)?;
        callback(&mut buf);
        Ok(buf)
    }
}
impl ReadVariable for String {
    fn read_variable_bytes_with_callback<F, R: Read>(
        reader: &mut R,
        callback: F,
        length: usize,
    ) -> Result<String>
    where
        F: Fn(&mut [u8]),
    {
        let vec = Vec::<u8>::read_variable_bytes_with_callback(reader, callback, length)?;
        let s = String::from_utf8(vec);
        match s {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }
}

/// This trait is to read a constant-length array.
pub trait ReadConstant<const LENGTH: usize>: Sized {
    fn read_constant_bytes_with_callback<F, R: Read>(reader: &mut R, callback: F) -> Result<Self>
    where
        F: Fn(&mut [u8]);

    fn read_constant_bytes<R: Read>(reader: &mut R) -> Result<Self> {
        Self::read_constant_bytes_with_callback(reader, |_| {})
    }
}
impl<const LENGTH: usize> ReadConstant<LENGTH> for [u8; LENGTH] {
    fn read_constant_bytes_with_callback<F, R: Read>(reader: &mut R, callback: F) -> Result<Self>
    where
        F: Fn(&mut [u8]),
    {
        let mut buf = [0; LENGTH];
        reader.read_exact(&mut buf)?;
        callback(&mut buf);
        Ok(buf)
    }
}
impl<const LENGTH: usize> ReadConstant<LENGTH> for String {
    fn read_constant_bytes_with_callback<F, R: Read>(reader: &mut R, callback: F) -> Result<Self>
    where
        F: Fn(&mut [u8]),
    {
        let bytes = <[u8; LENGTH]>::read_constant_bytes_with_callback(reader, callback)?;
        let s = String::from_utf8(bytes.to_vec());

        match s {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }
}
