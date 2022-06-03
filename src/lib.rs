#![doc = include_str!("../README.md")]

pub use self::read_bytes::{ReadConstant, ReadEndianness, ReadVariable};
pub use self::typed_read::TypedRead;
pub use self::typed_write::TypedWrite;
pub use self::write_bytes::{WriteEndianness, WriteRef};

mod read_bytes;
mod typed_read;
mod typed_write;
mod write_bytes;

// Endianness
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endianness {
    /// Big-endian.
    BE,
    /// Little-endian.
    LE,
    /// Native-endian.
    NE,
}
