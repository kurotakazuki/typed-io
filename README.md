# Typed I/O

[![Crate](https://img.shields.io/crates/v/typed-io.svg)](https://crates.io/crates/typed-io)
[![API](https://docs.rs/typed-io/badge.svg)](https://docs.rs/typed-io)

Typed I/O extends and types `std::io` to make it easier to use.

`typed_io` contains a number of common things you’ll need when doing input and output. The most core part of this module is the TypedRead and TypedWrite traits, which provide the most general interface for reading and writing input and output with type that easy to handle.

## Motivation
The motivation for creating this was that I wanted to process the type as it was.

With `std::io` alone, you have to write a few clichéd statements as follows.

```rust
use std::io::Read;

let mut reader: &[u8] = &[1, 0];
let mut buf = [0; std::mem::size_of::<u16>()];
reader.read_exact(&mut buf).unwrap();
let which_i_want = u16::from_le_bytes(buf);
assert_eq!(1_u16, which_i_want);
```

I therefore decided to cover them up.

```rust
use typed_io::TypedRead;

let mut reader: &[u8] = &[1, 0];
let which_i_want: u16 = reader.read_le().unwrap();
assert_eq!(1_u16, which_i_want);
```

As a side effect, it also hides the raw data `[u8]` and allows it to be processed in a typed state.

## Getting Started

### Installation

This crate works with Cargo and is on [crates.io](https://crates.io/crates/typed-io).
Add it to your `Cargo.toml` like so:

```toml
[dependencies]
typed-io = "0.1"
```

If you want to augment existing `Read` and `Write` traits, then import the extension methods like so:

```rust
use typed_io::{TypedRead, TypedWrite};
```

### Examples

#### Typed Read
```rust
use std::io::Cursor;
use typed_io::TypedRead;

let mut reader = Cursor::new(vec![1, 2, 3, 4]);
assert_eq!(258_u16, reader.read_be().unwrap());
assert_eq!(772_u16, reader.read_be().unwrap());
```

#### Typed Write
```rust
use typed_io::TypedWrite;

let mut writer = Vec::new();
writer.write_le(1_u16).unwrap();
writer.write_be(2_u16).unwrap();
assert_eq!(writer, [1, 0, 0, 2]);
```

## To Do
- [ ] Write traits
- [ ] Add provided methods
- [ ] Methods naming