# rusterix-codegen

Code generation library for ASTERIX message definitions.

This crate parses ASTERIX XML category definitions and generates type-safe Rust code for encoding and decoding ASTERIX messages.

## Overview

The code generation pipeline consists of three stages:

```
XML Definition → Parse → Transform → Generate → Rust Code
```

1. **Parse**: Reads XML and creates a structured model
2. **Transform**: Converts to IR and validates (bit counts, structure)
3. **Generate**: Produces Rust source code with `Encode`/`Decode` implementations

## Usage

### High-level Builder API

The simplest way to generate code:

```rust
use rusterix_codegen::builder::{Builder, RustBuilder};

let builder = RustBuilder::new();

// Generate code from a single file
let code = builder.build("cat048.xml")?;
std::fs::write("cat048.rs", code)?;

// Or build to a specific directory
builder.build_file("cat048.xml", "src/generated/")?;

// Or process an entire directory
builder.build_directory("definitions/", "src/generated/")?;
```

### Build Script Integration

For compile-time code generation, use in `build.rs`:

```rust
use rusterix_codegen::builder::{Builder, RustBuilder};
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=definitions/");

    let out_dir = env::var("OUT_DIR").unwrap();
    let builder = RustBuilder::new();

    builder.build_file("definitions/cat048.xml", &out_dir)
        .expect("Code generation failed");
}
```

### Low-level API

For more control over the generation process:

```rust
use rusterix_codegen::parse::parser::parse_category;
use rusterix_codegen::transform::transformer::to_ir;
use rusterix_codegen::generate::generate;

// Parse XML
let xml = std::fs::read_to_string("cat048.xml")?;
let category = parse_category(&xml)?;

// Transform to IR (validates structure)
let ir = to_ir(category);

// Generate Rust code
let tokens = generate(&ir);
let code = tokens.to_string();
```

## Module Structure

### `parse` - XML Parsing

Parses ASTERIX XML into Rust data structures:

```rust
use rusterix_codegen::parse::parser::parse_category;
use rusterix_codegen::parse::xml_model::*;

let category = parse_category(xml_content)?;
// category.id, category.items, etc.
```

### `transform` - IR Transformation

Converts parsed XML to an Intermediate Representation:

- Validates bit counts match byte declarations
- Normalizes item types
- Validates extended item FX bit allocation
- Checks for duplicate field names

```rust
use rusterix_codegen::transform::transformer::to_ir;
use rusterix_codegen::transform::ir::*;

let ir = to_ir(parsed_category);
// ir.category_id, ir.items, etc.
```

### `generate` - Code Generation

Produces Rust source code from IR:

```rust
use rusterix_codegen::generate::generate;

let tokens = generate(&ir);
let code = tokens.to_string();
```

Generated code includes:
- Category record struct (`Cat048Record`)
- Item structs (`Item010`, `Item020`, ...)
- Enum types with `Unknown` variant
- `Decode` and `Encode` trait implementations
- Documentation comments

## Supported XML Elements

| Element | Description |
|---------|-------------|
| `<category>` | Root element with category ID |
| `<item>` | Data item with ID and FRN |
| `<fixed>` | Fixed-length structure |
| `<extended>` | Variable-length with FX bits |
| `<compound>` | Multiple optional sub-items |
| `<repetitive>` | Repeated structures |
| `<explicit>` | Length-prefixed data |
| `<field>` | Named data field |
| `<enum>` | Enumerated values |
| `<epb>` | Element Populated Bit (optional) |
| `<spare>` | Reserved/padding bits |

See [XML_SCHEMA.md](../XML_SCHEMA.md) for complete documentation.

## Dependencies

- `quick-xml` - XML parsing
- `serde` - Deserialization
- `quote` / `proc-macro2` - Rust code generation
- `syn` - Rust syntax utilities

## License

MIT License - see the main repository for details.
