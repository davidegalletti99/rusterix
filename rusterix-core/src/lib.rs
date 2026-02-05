pub mod bit_reader;
pub mod bit_writer;
pub mod buffer;
pub mod error;
pub mod fspec;

pub use bit_reader::BitReader;
pub use bit_writer::BitWriter;
pub use error::DecodeError;
pub use fspec::Fspec;

/// Trait for encoding ASTERIX data structures to a bit stream.
pub trait Encode {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError>;
}

/// Trait for decoding ASTERIX data structures from a bit stream.
pub trait Decode: Sized {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError>;
}


#[cfg(test)]
mod tests {
}
