# Contributing to Rusterix

Thank you for your interest in contributing to Rusterix! This document provides guidelines and information for contributors.

## Project Overview

Rusterix is a Rust workspace with multiple crates:

| Crate | Purpose |
|-------|---------|
| `rusterix-core` | Runtime types (BitReader, BitWriter, Fspec, traits) |
| `rusterix-codegen` | XML parsing and Rust code generation |
| `rusterix` | Main library that re-exports the other crates |
| `test-utils` | Shared test utilities |

## Getting Started

### Prerequisites

- Rust 1.70+ (stable)
- Git

### Setup

```bash
git clone https://github.com/davidegalletti99/rusterix.git
cd rusterix
cargo build --workspace
cargo test --workspace
```

## Development Workflow

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p rusterix-core
cargo test -p rusterix-codegen
cargo test -p rusterix

# Run a specific test
cargo test -p rusterix-core bit_reader::tests::read_single_bit
```

### Test Categories

The test suite includes:

- **Unit tests**: Located in `src/*.rs` files with `#[cfg(test)]` modules
- **Integration tests**: Located in `tests/` directories
- **Roundtrip tests**: In `rusterix/tests/roundtrip_tests.rs` - test real generated code

### Adding Test Fixtures

Test fixtures are in `testdata/`:

```
testdata/
├── valid/      # Valid XML definitions for testing
├── invalid/    # Invalid XML for error testing
└── expected/   # Expected generated code (for comparison tests)
```

To add a new fixture:

1. Add XML file to `testdata/valid/` or `testdata/invalid/`
2. Add corresponding test in the appropriate test file
3. For codegen tests, regenerate expected output:
   ```bash
   cargo test -p rusterix-codegen --test regenerate_expected -- --ignored
   ```

## Code Style

### Formatting

```bash
cargo fmt --all
```

### Linting

```bash
cargo clippy --workspace --all-targets
```

### Documentation

- All public APIs should have doc comments
- Use `///` for item documentation
- Use `//!` for module-level documentation
- Include examples in doc comments where helpful

## Architecture

### Code Generation Pipeline

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  XML File   │ --> │   Parser    │ --> │ Transformer │ --> │  Generator  │
│             │     │ (xml_model) │     │    (IR)     │     │(TokenStream)│
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
```

1. **Parser** (`rusterix-codegen/src/parse/`): Reads XML into `xml_model` structures
2. **Transformer** (`rusterix-codegen/src/transform/`): Converts to IR, validates
3. **Generator** (`rusterix-codegen/src/generate/`): Produces Rust code

### Key Design Decisions

- **Zero runtime dependencies for generated code**: Generated code only uses `rusterix-core`
- **Validation at transform time**: Bit count validation happens during IR transformation
- **Unknown enum variants**: All enums include `Unknown(uN)` for forward compatibility
- **FSPEC-based records**: Category records use ASTERIX FSPEC for item presence

## Making Changes

### Adding a New XML Element

1. Add to DTD in `rusterix.dtd`
2. Add to XML model in `rusterix-codegen/src/parse/xml_model.rs`
3. Add parsing in `rusterix-codegen/src/parse/parser.rs`
4. Add to IR in `rusterix-codegen/src/transform/ir.rs`
5. Add transformation in `rusterix-codegen/src/transform/transformer.rs`
6. Add code generation in appropriate `rusterix-codegen/src/generate/*.rs`
7. Add tests at each level
8. Update documentation

### Modifying Code Generation

When changing generated code:

1. Update the generator in `rusterix-codegen/src/generate/`
2. Regenerate expected outputs:
   ```bash
   cargo test -p rusterix-codegen --test regenerate_expected -- --ignored
   ```
3. Run roundtrip tests to verify encode/decode still works:
   ```bash
   cargo test -p rusterix --test roundtrip_tests
   ```

### Adding to rusterix-core

The core crate should remain minimal and dependency-free:

1. Only add types that generated code needs
2. Keep implementations simple and efficient
3. Add comprehensive unit tests
4. Document all public APIs

## Pull Request Process

1. **Fork** the repository
2. **Create a branch** for your feature/fix
3. **Make changes** following the guidelines above
4. **Run tests**: `cargo test --workspace`
5. **Run lints**: `cargo clippy --workspace`
6. **Format code**: `cargo fmt --all`
7. **Submit PR** with clear description of changes

### PR Checklist

- [ ] Tests pass (`cargo test --workspace`)
- [ ] No clippy warnings (`cargo clippy --workspace`)
- [ ] Code is formatted (`cargo fmt --all`)
- [ ] Documentation is updated if needed
- [ ] CHANGELOG is updated for user-facing changes

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Relevant XML definition (if applicable)

## License

By contributing to Rusterix, you agree that your contributions will be licensed under the MIT License.
