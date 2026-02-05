//! Rusterix - ASTERIX message encoding/decoding library.
//!
//! This crate re-exports the core runtime (`rusterix-core`) and code generation
//! (`rusterix-codegen`) crates for convenient single-import usage.
//!
//! ## Crate Structure
//!
//! - [`rcore`] - Core runtime types (BitReader, BitWriter, Encode, Decode, Fspec)
//! - [`codegen`] - Code generation from XML definitions
//!
//! ## Usage
//!
//! For runtime encoding/decoding, use the `rcore` module:
//!
//! ```ignore
//! use rusterix::rcore::{BitReader, BitWriter, Decode, Encode};
//! ```
//!
//! For code generation, use the `codegen` module:
//!
//! ```ignore
//! use rusterix::codegen::builder::RustBuilder;
//! ```

/// Re-export of rusterix-core as `rcore`.
///
/// Contains runtime types for ASTERIX message encoding/decoding:
/// - [`BitReader`](rcore::BitReader) - Bit-level reading from byte streams
/// - [`BitWriter`](rcore::BitWriter) - Bit-level writing to byte streams
/// - [`Decode`](rcore::Decode) - Trait for decoding ASTERIX structures
/// - [`Encode`](rcore::Encode) - Trait for encoding ASTERIX structures
/// - [`Fspec`](rcore::Fspec) - ASTERIX Field Specification handling
/// - [`DecodeError`](rcore::DecodeError) - Error type for decode operations
pub mod rcore {
    pub use rusterix_core::*;
}

/// Re-export of rusterix-codegen as `codegen`.
///
/// Contains code generation utilities:
/// - [`builder`](codegen::builder) - High-level Builder API
/// - [`parse`](codegen::parse) - XML parsing
/// - [`transform`](codegen::transform) - IR transformation
/// - [`generate`](codegen::generate) - Rust code generation
pub mod codegen {
    pub use rusterix_codegen::*;
}

// Re-export commonly used types at the crate root for convenience
pub use rcore::{BitReader, BitWriter, Decode, DecodeError, Encode, Fspec};
