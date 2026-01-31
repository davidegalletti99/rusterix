# Rusterix Code Generation

This document describes the code generation pipeline for ASTERIX message definitions.

## Architecture

The code generation follows a three-stage pipeline:

```
XML Input → Parse → Transform (IR) → Generate → Rust Code
```

### 1. Parse Stage (`data_builder/parse/`)

- **xml_model.rs**: Serde data structures matching the XML schema
- **parser.rs**: XML parsing using `serde-xml-rs`

**Input**: ASTERIX XML definition files
**Output**: Structured Rust types representing the XML

### 2. Transform Stage (`data_builder/transform/`)

- **ir.rs**: Intermediate Representation (IR) data structures
- **transformer.rs**: Converts XML model to validated IR

**Key Features**:
- Validates bit counts match byte declarations
- Normalizes different item types (Fixed, Extended, Compound, etc.)
- Prepares data for code generation

### 3. Generate Stage (`data_builder/generate/`)

Main orchestration and utilities:
- **generator.rs**: Main entry point, produces complete module
- **utils.rs**: Helper functions (type mapping, naming conventions)

Specialized generators:
- **record_gen.rs**: Generates `Cat{N}Record` with automatic FSPEC management
- **item_gen.rs**: Generates `Item{N}` structs with decode/encode
- **struct_gen.rs**: Low-level struct generation
- **decode_gen.rs**: Generates decode implementations
- **encode_gen.rs**: Generates encode implementations
- **enum_gen.rs**: Generates enum types with Unknown variant

## Supported ASTERIX Features

### Item Types

| Type | Description | Wire Format |
|------|-------------|-------------|
| **Fixed** | Fixed-length item | `[data bytes]` |
| **Explicit** | Fixed with length byte | `[LEN:1][data bytes]` |
| **Extended** | Variable-length with FX bits | `[byte0: 7 bits + FX][byte1: ...]` |
| **Repetitive** | Repeated structure | `[rep 0][rep 1]...[rep N-1]` |
| **Compound** | Multiple optional sub-items | `[FSPEC][sub-item 0 if present][...]` |

### Element Types

| Type | Description | Generated Rust Type |
|------|-------------|-------------------|
| **Field** | Raw binary data | `u8`, `u16`, `u32`, `u64`, `u128` |
| **Enum** | Named values | `enum TypeName { Variant1 = 1, Unknown(u8) }` |
| **EPB** | Extended Primary Bit | `Option<T>` (automatic validity bit) |
| **Spare** | Unused bits | Not in struct (skip on read, write 0) |

## Code Generation Examples

### Simple Fixed Item

**XML**:
```xml
<item id="010" frn="0">
    <fixed bytes="2">
        <field name="sac" bits="8"/>
        <field name="sic" bits="8"/>
    </fixed>
</item>
```

**Generated Rust**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Item010 {
    pub sac: u8,
    pub sic: u8,
}

impl Item010 {
    pub fn decode<R: Read>(reader: &mut BitReader<R>) 
        -> Result<Self, DecodeError> {
        let sac = reader.read_bits(8)? as u8;
        let sic = reader.read_bits(8)? as u8;
        Ok(Self { sac, sic })
    }
    
    pub fn encode<W: Write>(&self, writer: &mut BitWriter<W>) 
        -> Result<(), DecodeError> {
        writer.write_bits(self.sac as u64, 8)?;
        writer.write_bits(self.sic as u64, 8)?;
        Ok(())
    }
}
```

### Item with Enum

**XML**:
```xml
<item id="020" frn="1">
    <fixed bytes="1">
        <enum name="target_type" bits="3">
            <value name="PSR" value="1"/>
            <value name="SSR" value="2"/>
        </enum>
        <spare bits="5"/>
    </fixed>
</item>
```

**Generated Rust**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TargetType {
    Psr = 1,
    Ssr = 2,
    Unknown(u8),
}

impl TryFrom<u8> for TargetType { /* ... */ }
impl From<TargetType> for u8 { /* ... */ }

#[derive(Debug, Clone, PartialEq)]
pub struct Item020 {
    pub target_type: TargetType,
}

impl Item020 {
    pub fn decode<R: Read>(reader: &mut BitReader<R>) 
        -> Result<Self, DecodeError> {
        let target_type = {
            let value = reader.read_bits(3)? as u8;
            TargetType::try_from(value).unwrap()
        };
        reader.read_bits(5)?; // Skip spare
        Ok(Self { target_type })
    }
    
    pub fn encode<W: Write>(&self, writer: &mut BitWriter<W>) 
        -> Result<(), DecodeError> {
        writer.write_bits(u8::from(self.target_type) as u64, 3)?;
        writer.write_bits(0, 5)?; // Spare bits
        Ok(())
    }
}
```

### Item with EPB

**XML**:
```xml
<item id="030" frn="2">
    <fixed bytes="2">
        <epb>
            <field name="optional_value" bits="15"/>
        </epb>
    </fixed>
</item>
```

**Generated Rust**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Item030 {
    pub optional_value: Option<u16>,  // Name taken from inner field
}

impl Item030 {
    pub fn decode<R: Read>(reader: &mut BitReader<R>) 
        -> Result<Self, DecodeError> {
        let optional_value = {
            let valid = reader.read_bits(1)? != 0;
            if valid {
                Some(reader.read_bits(15)? as u16)
            } else {
                reader.read_bits(15)?; // Skip value
                None
            }
        };
        Ok(Self { optional_value })
    }
    
    pub fn encode<W: Write>(&self, writer: &mut BitWriter<W>) 
        -> Result<(), DecodeError> {
        if let Some(value) = self.optional_value {
            writer.write_bits(1, 1)?; // Valid bit
            writer.write_bits(value as u64, 15)?;
        } else {
            writer.write_bits(0, 1)?; // Invalid bit
            writer.write_bits(0, 15)?; // Zero value
        }
        Ok(())
    }
}
```

### Extended Item

**XML**:
```xml
<item id="040" frn="3">
    <extended bytes="2">
        <byte index="0">
            <field name="a" bits="3"/>
            <field name="b" bits="4"/>
        </byte>
        <byte index="1">
            <field name="c" bits="5"/>
            <spare bits="2"/>
        </byte>
    </extended>
</item>
```

**Generated Rust**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Item040Part0 {
    pub a: u8,
    pub b: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item040Part1 {
    pub c: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item040 {
    pub part0: Item040Part0,        // Always present
    pub part1: Option<Item040Part1>, // Present if FX=1
}

// Decode/encode automatically handle FX bits
```

### Category Record

**Generated Rust**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Cat048Record {
    pub item010: Option<Item010>,
    pub item020: Option<Item020>,
    pub item030: Option<Item030>,
    // ...
}

impl Cat048Record {
    pub fn decode<R: Read>(reader: &mut R) 
        -> Result<Self, DecodeError> {
        // Reads FSPEC automatically
        // Decodes only present items
    }
    
    pub fn encode<W: Write>(&self, writer: &mut W) 
        -> Result<(), DecodeError> {
        // Builds FSPEC from Some/None
        // Writes FSPEC + present items
    }
}
```

## Build Integration

Add to `build.rs`:

```rust
use rusterix::data_builder::builder::{Builder, RustBuilder};

fn main() {
    let builder = RustBuilder {};
    let code = builder.build("asterix/cat048.xml")
        .expect("Failed to generate code");
    
    fs::write("generated/cat048.rs", code)
        .expect("Failed to write generated code");
}
```

## Validation

The transformer validates:
- ✅ Bit counts match byte declarations
- ✅ Extended byte groups have exactly 7 bits (+1 FX)
- ✅ Enum values are valid u8
- ✅ EPB content is Field or Enum only
- ✅ Repetitive counters are valid numbers

Build-time panics ensure invalid XML is caught early.

## Testing

Generated code includes:
- Unit tests for helper functions
- Integration tests can be added in `tests/`
- Round-trip tests: encode → bytes → decode → verify equality

## Future Enhancements

Potential improvements:
- [ ] Support for dynamic repetition (field-based counter)
- [ ] Nested compound items
- [ ] Custom derive macros for common patterns
- [ ] Performance optimizations (zero-copy parsing)
- [ ] ASTERIX data block wrapper (multiple records)