pub mod bit_reader;
pub mod bit_writer;
pub mod buffer;
pub mod error;
pub mod fspec;

use bit_reader::BitReader;
use bit_writer::BitWriter;
use error::DecodeError;

// type aliases for convenience
use std::io::Write as IoWrite;
use std::io::Read as IoRead;

pub trait Encode {
    fn encode(
        writer: &mut BitWriter<impl IoWrite>,
        fspec: &mut fspec::Fspec,
    ) -> Result<(), DecodeError>;
}
pub trait Decode: Sized {
    fn decode(
        reader: &mut BitReader<impl IoRead>,
        fspec: &fspec::Fspec,
    ) -> Result<Self, DecodeError>;
}
