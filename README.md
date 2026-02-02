# Rusterix
**Rusterix** is a Rust library for encoding and decoding [ASTERIX](https://www.eurocontrol.int/asterix) (All Purpose Structured Eurocontrol Surveillance Information Exchange) messages.

ASTERIX is the standard data format used in air traffic control systems for exchanging surveillance data (radar, ADS-B, etc.).

## Features
- **Code Generation**: Automatically generate Rust structs from ASTERIX XML category definitions
- **Bit-level I/O**: Efficient bit-level reading and writing for binary protocols
- **Type Safety**: Generated code is fully typed with enums, optional fields, and validation
- **Zero Runtime Dependencies**: Generated code only depends on `rusterix::rcore`

## Quick Start
### 1. Add dependency
```toml
[dependencies]
rusterix = { path = "../rusterix" }
```

### 2. Generate code from XML definition
```rust
use rusterix::builder::{Builder, RustBuilder};

fn main() {
    let builder = RustBuilder::new();

    // Generate Rust code from ASTERIX XML definition
    let code = builder.build("definitions/cat048.xml")
        .expect("Failed to generate code");

    // Write to file
    std::fs::write("src/generated/cat048.rs", code)
        .expect("Failed to write generated code");
}
```

### 3. Use generated code
```rust
use rusterix::rcore::{BitReader, BitWriter, DecodeError};
use crate::generated::cat048::Cat048Record;

// Decode a record from bytes
fn decode_record(data: &[u8]) -> Result<Cat048Record, DecodeError> {
    let mut reader = std::io::Cursor::new(data);
    Cat048Record::decode(&mut reader)
}

// Encode a record to bytes
fn encode_record(record: &Cat048Record) -> Result<Vec<u8>, DecodeError> {
    let mut buffer = Vec::new();
    record.encode(&mut buffer)?;
    Ok(buffer)
}
```

## Project Structure
```
rusterix/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── builder.rs          # High-level build API
│   ├── rcore/              # Runtime core (used by generated code)
│   │   ├── bit_reader.rs   # Bit-level reading
│   │   ├── bit_writer.rs   # Bit-level writing
│   │   ├── fspec.rs        # FSPEC handling
│   │   └── error.rs        # Error types
│   ├── parse/              # XML parsing
│   │   ├── parser.rs       # XML parser
│   │   └── xml_model.rs    # XML data structures
│   ├── transform/          # IR transformation
│   │   ├── ir.rs           # Intermediate representation
│   │   └── transformer.rs  # XML → IR conversion
│   └── generate/           # Code generation
│       ├── generator.rs    # Main generator
│       ├── record_gen.rs   # Category record generation
│       ├── item_gen.rs     # Item struct generation
│       └── ...
└── tests/
    └── fixtures/           # Test XML definitions
```

## Modules
### `rcore` - Runtime Core
The runtime module provides types used by generated code:

| Type | Description |
|------|-------------|
| `BitReader<R>` | Reads bits from a byte stream |
| `BitWriter<W>` | Writes bits to a byte stream |
| `Fspec` | Handles ASTERIX Field Specification |
| `DecodeError` | Error type for decode/encode operations |
| `Decode` | Trait for decodable types |
| `Encode` | Trait for encodable types |

### `parse` - XML Parsing
Parses ASTERIX XML category definitions into Rust data structures.

### `transform` - IR Transformation
Converts parsed XML into an Intermediate Representation (IR):
- Validates bit counts match byte declarations
- Normalizes different item types
- Prepares data for code generation

### `generate` - Code Generation
Generates Rust source code from the IR:
- Category record structs (`Cat048Record`)
- Item structs (`Item010`, `Item020`, ...)
- Enum types with `Unknown` variant for forward compatibility
- Decode/Encode implementations

## XML Schema

Rusterix uses XML files to define ASTERIX categories. See [XML_SCHEMA.md](XML_SCHEMA.md) for complete documentation.

### Quick Reference

**Data Structures:**
| Type | Description | Example |
|------|-------------|---------|
| `<fixed>` | Fixed-length data | `Item010` with 2 bytes |
| `<extended>` | Variable-length with FX bits | Target report with optional extensions |
| `<compound>` | Multiple optional sub-items | Composed data items |
| `<repetitive>` | Repeated structures | List of plot characteristics |
| `<explicit>` | Length-prefixed data | Variable content |

**Field Elements:**
| Type | Generated Rust Type |
|------|---------------------|
| `<field>` | `u8`, `u16`, `u32`, `u64` |
| `<enum>` | `enum Name { Value1 = 1, Unknown(u8) }` |
| `<epb>` | `Option<T>` |
| `<spare>` | Skipped (not in struct) |

### Minimal XML Example

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE category SYSTEM "rusterix.dtd">
<category id="048">
    <!-- Fixed: Data Source Identifier -->
    <item id="010" frn="1">
        <fixed bytes="2">
            <field name="sac" bits="8"/>
            <field name="sic" bits="8"/>
        </fixed>
    </item>

    <!-- Extended: Variable length with FX bits -->
    <item id="020" frn="2">
        <extended bytes="1">
            <part index="0">
                <enum name="type" bits="3">
                    <value name="PSR" value="1"/>
                    <value name="SSR" value="2"/>
                </enum>
                <field name="sim" bits="1"/>
                <field name="rdp" bits="1"/>
                <spare bits="2"/>
            </part>
        </extended>
    </item>

    <!-- Compound: Multiple optional sub-items -->
    <item id="130" frn="3">
        <compound>
            <fixed bytes="1">
                <field name="amplitude" bits="8"/>
            </fixed>
            <fixed bytes="1">
                <field name="runlength" bits="8"/>
            </fixed>
        </compound>
    </item>
</category>
```

## Example: Generated Code
From this XML:
```xml
<category id="48">
    <item id="010" frn="0">
        <fixed bytes="2">
            <field name="sac" bits="8"/>
            <field name="sic" bits="8"/>
        </fixed>
    </item>
</category>
```

Rusterix generates:
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Cat048Record {
    pub item010: Option<Item010>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item010 {
    pub sac: u8,
    pub sic: u8,
}

impl Decode for Item010 {
    fn decode<R: Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let sac = reader.read_bits(8)? as u8;
        let sic = reader.read_bits(8)? as u8;
        Ok(Self { sac, sic })
    }
}

impl Encode for Item010 {
    fn encode<W: Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(self.sac as u64, 8)?;
        writer.write_bits(self.sic as u64, 8)?;
        Ok(())
    }
}
```

## Testing
```bash
cargo test
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2025 Davide Galletti
