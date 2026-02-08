# rusterix

Main library crate for ASTERIX message encoding/decoding.

This crate re-exports the core runtime (`rusterix-core`) and code generation (`rusterix-codegen`) crates, providing a single import point for all Rusterix functionality.

## Overview

```rust
// All functionality available through one import
use rusterix::{
    // Core runtime types (from rusterix-core)
    BitReader, BitWriter, Decode, Encode, DecodeError, Fspec,

    // Or access via modules
    rcore,   // rusterix-core re-export
    codegen, // rusterix-codegen re-export
};
```

## Modules

### `rcore` - Runtime Core

Re-exports `rusterix-core` for runtime encoding/decoding:

```rust
use rusterix::rcore::{BitReader, BitWriter, Decode, Encode, Fspec, DecodeError};
```

### `codegen` - Code Generation

Re-exports `rusterix-codegen` for generating Rust code from XML:

```rust
use rusterix::codegen::builder::{Builder, RustBuilder};
use rusterix::codegen::parse::parser::parse_category;
use rusterix::codegen::transform::transformer::to_ir;
use rusterix::codegen::generate::generate;
```

## Usage

### As a dependency

```toml
[dependencies]
rusterix = "0.1"
```

### For build-time code generation

```toml
[build-dependencies]
rusterix = "0.1"
```

### Example

```rust
use rusterix::{BitReader, BitWriter, Decode, Encode, DecodeError};
use rusterix::codegen::builder::{Builder, RustBuilder};
use std::io::Cursor;

// Generate code from XML
let builder = RustBuilder::new();
let code = builder.build("cat048.xml")?;

// Use generated code (after including in your project)
// let record = cat48::Record::decode(&mut reader)?;
// record.encode(&mut buffer)?;
```

## Testing

This crate contains integration tests that verify the entire pipeline:

- **Builder tests**: Test the high-level Builder API
- **Roundtrip tests**: Verify `decode(encode(value)) == value` using real generated code

The roundtrip tests use actual code generated at build time from XML fixtures, ensuring that any changes to the code generator are immediately tested.

```bash
cargo test -p rusterix
```

## License

MIT License - see the main repository for details.
