use crate::{Endianness, WriteEndianness, WriteRef};
use std::io::{Result, Write};

pub trait TypedWrite: Write + Sized {
    // WriteEndianness
    fn write_endian_with_callback<F, T: WriteEndianness>(
        &mut self,
        n: T,
        callback: F,
        endianness: Endianness,
    ) -> Result<()>
    where
        F: Fn(&mut [u8]),
    {
        n.write_endianness_with_callback(self, callback, endianness)
    }
    fn write_be<T: WriteEndianness>(&mut self, n: T) -> Result<()> {
        n.write_be_bytes(self)
    }
    fn write_le<T: WriteEndianness>(&mut self, n: T) -> Result<()> {
        n.write_le_bytes(self)
    }
    fn write_ne<T: WriteEndianness>(&mut self, n: T) -> Result<()> {
        n.write_ne_bytes(self)
    }

    // WriteRef
    fn write_ref_with_callback<F, T: WriteRef>(&mut self, n: T, callback: F) -> Result<()>
    where
        F: Fn(&[u8]),
    {
        n.write_ref_bytes_with_callback(self, callback)
    }
    fn write_ref<T: WriteRef + ?Sized>(&mut self, n: &T) -> Result<()> {
        n.write_ref_bytes(self)
    }
}

impl<W: Write> TypedWrite for W {}

#[cfg(test)]
mod tests {
    use super::TypedWrite;
    use std::io;

    #[test]
    fn write_string() -> io::Result<()> {
        let mut v = Vec::new();
        v.write_ref("bkbkb")?;

        assert_eq!(v, [98, 107, 98, 107, 98]);

        Ok(())
    }

    #[test]
    fn write_unsigned_integer_type() -> io::Result<()> {
        let mut v = Vec::new();

        v.write_le(1_u8)?;
        v.write_le(2_u16)?;
        v.write_le(3_u32)?;
        v.write_le(4_u64)?;
        v.write_be(5_u128)?;

        assert_eq!(
            v,
            [
                1, 2, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 5,
            ]
        );

        Ok(())
    }
}
