# rasterix-core

Core runtime library for ASTERIX message encoding/decoding.

This crate provides the fundamental types used by code generated from ASTERIX XML definitions. It has **zero external dependencies**, making it lightweight and suitable for embedded systems.

## Overview

`rasterix-core` provides:

- **Bit-level I/O**: Read and write individual bits from byte streams
- **FSPEC handling**: Parse and generate ASTERIX Field Specification bytes
- **Traits**: `Encode` and `Decode` traits implemented by generated code
- **Error handling**: Unified error type for encode/decode operations

## Types

### BitReader

Reads bits from any `std::io::Read` source:

```rust
use rasterix_core::BitReader;
use std::io::Cursor;

let data = [0b10110100, 0xFF];
let mut reader = BitReader::new(Cursor::new(&data));

// Read individual bits
let bit = reader.read_bits(1)?;  // Returns 1

// Read multiple bits (up to 64)
let value = reader.read_bits(7)?;  // Returns 0b0110100 = 52
```

### BitWriter

Writes bits to any `std::io::Write` destination:

```rust
use rasterix_core::BitWriter;

let mut buffer = Vec::new();
let mut writer = BitWriter::new(&mut buffer);

// Write individual bits
writer.write_bits(1, 1)?;  // Write single '1' bit

// Write multiple bits
writer.write_bits(0b1010, 4)?;  // Write 4 bits

// Always flush when done to write remaining partial byte
writer.flush()?;
```

### Fspec

Handles ASTERIX Field Specification - a variable-length bitmap indicating which data items are present:

```rust
use rasterix_core::Fspec;

// Create and set bits
let mut fspec = Fspec::new();
fspec.set(0, 0);  // Set bit 0 in byte 0 (MSB, item present)
fspec.set(0, 3);  // Set bit 3 in byte 0

// Check if bits are set
assert!(fspec.is_set(0, 0));
assert!(!fspec.is_set(0, 1));

// Write to stream
let mut buffer = Vec::new();
fspec.write(&mut buffer)?;

// Read from stream
let fspec = Fspec::read(&mut Cursor::new(&buffer))?;
```

### Traits

```rust
use rasterix_core::{Decode, Encode, BitReader, BitWriter, DecodeError};

// Implemented by generated ASTERIX structures
pub trait Decode: Sized {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError>;
}

pub trait Encode {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError>;
}
```

### DecodeError

Unified error type for all encode/decode operations:

```rust
use rasterix_core::DecodeError;

pub enum DecodeError {
    Io(std::io::Error),
    InvalidData(String),
}
```

## Usage

This crate is typically used indirectly through generated code. The main `rasterix` crate re-exports it as the `rcore` module:

```rust
use rasterix::rcore::{BitReader, BitWriter, Decode, Encode};
```

## Dependencies

This crate has **no external dependencies** - only the Rust standard library.

## License

MIT License - see the main repository for details.
