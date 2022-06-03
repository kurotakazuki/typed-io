use crate::{Endianness, ReadConstant, ReadEndianness, ReadVariable};
use std::io::{Read, Result};

pub trait TypedRead: Read + Sized {
    // ReadEndianness
    fn read_endianness_with_callback<F, T: ReadEndianness>(
        &mut self,
        callback: F,
        endianness: Endianness,
    ) -> Result<T>
    where
        F: Fn(&mut [u8]),
    {
        <T>::read_endianness_bytes_with_callback(self, callback, endianness)
    }
    fn read_be<T: ReadEndianness>(&mut self) -> Result<T> {
        <T>::read_be_bytes(self)
    }
    fn read_le<T: ReadEndianness>(&mut self) -> Result<T> {
        <T>::read_le_bytes(self)
    }
    fn read_ne<T: ReadEndianness>(&mut self) -> Result<T> {
        <T>::read_ne_bytes(self)
    }

    // ReadVariable
    fn read_var_len_with_callback<F, T: ReadVariable>(
        &mut self,
        callback: F,
        length: usize,
    ) -> Result<T>
    where
        F: Fn(&mut [u8]),
    {
        <T>::read_variable_bytes_with_callback(self, callback, length)
    }
    fn read_var_len<T: ReadVariable>(&mut self, length: usize) -> Result<T> {
        <T>::read_variable_bytes(self, length)
    }

    // ReadConstant
    fn read_const_len_with_callback<F, T: ReadConstant<LENGTH>, const LENGTH: usize>(
        &mut self,
        callback: F,
    ) -> Result<T>
    where
        F: Fn(&mut [u8]),
    {
        <T>::read_constant_bytes_with_callback(self, callback)
    }
    fn read_const_len<T: ReadConstant<LENGTH>, const LENGTH: usize>(&mut self) -> Result<T> {
        <T>::read_constant_bytes(self)
    }
}

impl<R: Read> TypedRead for R {}

#[cfg(test)]
mod tests {
    use super::TypedRead;
    use std::io;

    #[test]
    fn read_string_for() {
        let mut v: &[u8] = &[98, 107, 98, 107, 98];
        let bytes: [u8; 5] = v.read_const_len().unwrap();
        let s = std::str::from_utf8(&bytes).unwrap();
        assert_eq!(s, "bkbkb");
        let mut v: &[u8] = &[
            0xE3, 0x81, 0xB3, 0xE3, 0x81, 0x8B, 0xE3, 0x81, 0xB3, 0xE3, 0x81, 0x8B, 0xE3, 0x81,
            0xB3,
        ];
        let s: String = v.read_const_len::<_, 15>().unwrap();
        assert_eq!(s, "びかびかび");

        let mut v: &[u8] = &[
            0xE3, 0x81, 0xB3, 0xE3, 0x81, 0x8B, 0xE3, 0x81, 0xB3, 0xE3, 0x81, 0x8B, 0xE3, 0x81,
            0xB3,
        ];
        let s: String = v.read_var_len(15).unwrap();
        assert_eq!(s, "びかびかび");
    }

    #[test]
    fn read_unsigned_integer_type() -> io::Result<()> {
        let mut v: &[u8] = &[
            1, 2, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 5,
        ];

        let u8_bytes: u8 = v.read_le()?;
        assert_eq!(u8_bytes, 1);

        let u16_bytes: u16 = v.read_le()?;
        assert_eq!(u16_bytes, 2);

        let u32_bytes: u32 = v.read_le()?;
        assert_eq!(u32_bytes, 3);

        let u64_bytes: u64 = v.read_le()?;
        assert_eq!(u64_bytes, 4);

        let u128_bytes: u128 = v.read_be()?;
        assert_eq!(u128_bytes, 5);

        Ok(())
    }
}
