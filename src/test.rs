#![allow(unused_imports)]
#![allow(dead_code)]
use crate::framework::{BitReader, BitWriter, Decode, DecodeError, Encode, Fspec};
use std::io::{Read, Write};
#[doc = r" ASTERIX Category record."]
#[doc = r" "]
#[doc = r" Contains optional data items, each controlled by a bit in the FSPEC."]
#[derive(Debug, Clone, PartialEq)]
pub struct Cat255Record {
    pub item010: Option<Item010>,
    pub item020: Option<Item020>,
    pub item030: Option<Item030>,
    pub item040: Option<Item040>,
    pub item050: Option<Item050>,
    pub item060: Option<Item060>,
    pub item070: Option<Item070>,
    pub item080: Option<Item080>,
    pub item090: Option<Item090>,
    pub item100: Option<Item100>,
    pub item110: Option<Item110>,
    pub item120: Option<Item120>,
}
impl Cat255Record {
    #[doc = r" Decodes a record from a binary stream."]
    #[doc = r" "]
    #[doc = r" Reads the FSPEC to determine which items are present, then"]
    #[doc = r" decodes only the present items."]
    #[doc = r" "]
    #[doc = r" # Arguments"]
    #[doc = r" "]
    #[doc = r" * `reader` - The input stream to read from"]
    #[doc = r" "]
    #[doc = r" # Errors"]
    #[doc = r" "]
    #[doc = r" Returns an error if reading or parsing fails."]
    pub fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let fspec = Fspec::read(reader)?;
        let mut bit_reader = BitReader::new(reader);
        Ok(Self {
            item010: if fspec.is_set(0usize, 7u8) {
                Some(Item010::decode(&mut bit_reader)?)
            } else {
                None
            },
            item020: if fspec.is_set(0usize, 6u8) {
                Some(Item020::decode(&mut bit_reader)?)
            } else {
                None
            },
            item030: if fspec.is_set(0usize, 5u8) {
                Some(Item030::decode(&mut bit_reader)?)
            } else {
                None
            },
            item040: if fspec.is_set(0usize, 4u8) {
                Some(Item040::decode(&mut bit_reader)?)
            } else {
                None
            },
            item050: if fspec.is_set(0usize, 3u8) {
                Some(Item050::decode(&mut bit_reader)?)
            } else {
                None
            },
            item060: if fspec.is_set(0usize, 2u8) {
                Some(Item060::decode(&mut bit_reader)?)
            } else {
                None
            },
            item070: if fspec.is_set(0usize, 1u8) {
                Some(Item070::decode(&mut bit_reader)?)
            } else {
                None
            },
            item080: if fspec.is_set(0usize, 0u8) {
                Some(Item080::decode(&mut bit_reader)?)
            } else {
                None
            },
            item090: if fspec.is_set(1usize, 7u8) {
                Some(Item090::decode(&mut bit_reader)?)
            } else {
                None
            },
            item100: if fspec.is_set(1usize, 6u8) {
                Some(Item100::decode(&mut bit_reader)?)
            } else {
                None
            },
            item110: if fspec.is_set(1usize, 5u8) {
                Some(Item110::decode(&mut bit_reader)?)
            } else {
                None
            },
            item120: if fspec.is_set(1usize, 4u8) {
                Some(Item120::decode(&mut bit_reader)?)
            } else {
                None
            },
        })
    }
}
impl Cat255Record {
    #[doc = r" Encodes a record to a binary stream."]
    #[doc = r" "]
    #[doc = r" Automatically constructs the FSPEC based on which items are present,"]
    #[doc = r" then encodes all present items."]
    #[doc = r" "]
    #[doc = r" # Arguments"]
    #[doc = r" "]
    #[doc = r" * `writer` - The output stream to write to"]
    #[doc = r" "]
    #[doc = r" # Errors"]
    #[doc = r" "]
    #[doc = r" Returns an error if writing fails."]
    pub fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), DecodeError> {
        let mut fspec = Fspec::new();
        if self.item010.is_some() {
            fspec.set(0usize, 7u8);
        }
        if self.item020.is_some() {
            fspec.set(0usize, 6u8);
        }
        if self.item030.is_some() {
            fspec.set(0usize, 5u8);
        }
        if self.item040.is_some() {
            fspec.set(0usize, 4u8);
        }
        if self.item050.is_some() {
            fspec.set(0usize, 3u8);
        }
        if self.item060.is_some() {
            fspec.set(0usize, 2u8);
        }
        if self.item070.is_some() {
            fspec.set(0usize, 1u8);
        }
        if self.item080.is_some() {
            fspec.set(0usize, 0u8);
        }
        if self.item090.is_some() {
            fspec.set(1usize, 7u8);
        }
        if self.item100.is_some() {
            fspec.set(1usize, 6u8);
        }
        if self.item110.is_some() {
            fspec.set(1usize, 5u8);
        }
        if self.item120.is_some() {
            fspec.set(1usize, 4u8);
        }
        fspec.write(writer)?;
        let mut bit_writer = BitWriter::new(writer);
        if let Some(ref item) = self.item010 {
            item.encode(&mut bit_writer)?;
        }
        if let Some(ref item) = self.item020 {
            item.encode(&mut bit_writer)?;
        }
        if let Some(ref item) = self.item030 {
            item.encode(&mut bit_writer)?;
        }
        if let Some(ref item) = self.item040 {
            item.encode(&mut bit_writer)?;
        }
        if let Some(ref item) = self.item050 {
            item.encode(&mut bit_writer)?;
        }
        if let Some(ref item) = self.item060 {
            item.encode(&mut bit_writer)?;
        }
        if let Some(ref item) = self.item070 {
            item.encode(&mut bit_writer)?;
        }
        if let Some(ref item) = self.item080 {
            item.encode(&mut bit_writer)?;
        }
        if let Some(ref item) = self.item090 {
            item.encode(&mut bit_writer)?;
        }
        if let Some(ref item) = self.item100 {
            item.encode(&mut bit_writer)?;
        }
        if let Some(ref item) = self.item110 {
            item.encode(&mut bit_writer)?;
        }
        if let Some(ref item) = self.item120 {
            item.encode(&mut bit_writer)?;
        }
        bit_writer.flush()?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item010 {
    pub sac: u8,
    pub sic: u8,
}
impl Decode for Item010 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let sac = reader.read_bits(8usize)? as u8;
        let sic = reader.read_bits(8usize)? as u8;
        Ok(Self { sac, sic })
    }
}
impl Encode for Item010 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(self.sac as u64, 8usize)?;
        writer.write_bits(self.sic as u64, 8usize)?;
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    NorthMarker = 0u8,
    SectorCrossing = 1u8,
    GeographicalFilter = 2u8,
    JammingStrobe = 3u8,
    Unknown(u8),
}
impl TryFrom<u8> for MessageType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0u8 => Ok(Self::NorthMarker),
            1u8 => Ok(Self::SectorCrossing),
            2u8 => Ok(Self::GeographicalFilter),
            3u8 => Ok(Self::JammingStrobe),
            _ => Ok(Self::Unknown(value)),
        }
    }
}
impl From<MessageType> for u8 {
    fn from(val: MessageType) -> u8 {
        match val {
            MessageType::NorthMarker => 0u8,
            MessageType::SectorCrossing => 1u8,
            MessageType::GeographicalFilter => 2u8,
            MessageType::JammingStrobe => 3u8,
            MessageType::Unknown(v) => v,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item020 {
    pub message_type: MessageType,
}
impl Decode for Item020 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let message_type = {
            let value = reader.read_bits(3usize)? as u8;
            MessageType::try_from(value).unwrap()
        };
        reader.read_bits(5usize)?;
        Ok(Self { message_type })
    }
}
impl Encode for Item020 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(u8::from(self.message_type) as u64, 3usize)?;
        writer.write_bits(0, 5usize)?;
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    Ok = 0u8,
    Warning = 1u8,
    Error = 2u8,
    Unknown(u8),
}
impl TryFrom<u8> for Status {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0u8 => Ok(Self::Ok),
            1u8 => Ok(Self::Warning),
            2u8 => Ok(Self::Error),
            _ => Ok(Self::Unknown(value)),
        }
    }
}
impl From<Status> for u8 {
    fn from(val: Status) -> u8 {
        match val {
            Status::Ok => 0u8,
            Status::Warning => 1u8,
            Status::Error => 2u8,
            Status::Unknown(v) => v,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item030 {
    pub warning_level: Option<u8>,
    pub status: Option<Status>,
    pub flags: u8,
}
impl Decode for Item030 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let warning_level = {
            let valid = reader.read_bits(1)? != 0;
            if valid {
                Some(reader.read_bits(3usize)? as u8)
            } else {
                reader.read_bits(3usize)?;
                None
            }
        };
        let status = {
            let valid = reader.read_bits(1)? != 0;
            if valid {
                let value = reader.read_bits(2usize)? as u8;
                Some(Status::try_from(value).unwrap())
            } else {
                reader.read_bits(2usize)?;
                None
            }
        };
        reader.read_bits(3usize)?;
        let flags = reader.read_bits(6usize)? as u8;
        Ok(Self {
            warning_level,
            status,
            flags,
        })
    }
}
impl Encode for Item030 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        if let Some(value) = self.warning_level {
            writer.write_bits(1, 1)?;
            writer.write_bits(value as u64, 3usize)?;
        } else {
            writer.write_bits(0, 1)?;
            writer.write_bits(0, 3usize)?;
        }
        if let Some(value) = self.status {
            writer.write_bits(1, 1)?;
            writer.write_bits(u8::from(value) as u64, 2usize)?;
        } else {
            writer.write_bits(0, 1)?;
            writer.write_bits(0, 2usize)?;
        }
        writer.write_bits(0, 3usize)?;
        writer.write_bits(self.flags as u64, 6usize)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item040Part0 {
    pub bit1: u8,
    pub bit2: u8,
    pub bit3: u8,
    pub bit4: u8,
    pub bit5: u8,
    pub bit6: u8,
    pub bit7: u8,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item040 {
    pub part0: Item040Part0,
}
impl Item040Part0 {
    pub fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let bit1 = reader.read_bits(1usize)? as u8;
        let bit2 = reader.read_bits(1usize)? as u8;
        let bit3 = reader.read_bits(1usize)? as u8;
        let bit4 = reader.read_bits(1usize)? as u8;
        let bit5 = reader.read_bits(1usize)? as u8;
        let bit6 = reader.read_bits(1usize)? as u8;
        let bit7 = reader.read_bits(1usize)? as u8;
        Ok(Self {
            bit1,
            bit2,
            bit3,
            bit4,
            bit5,
            bit6,
            bit7,
        })
    }
}
impl Decode for Item040 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let part0 = Item040Part0::decode(reader)?;
        let mut fx = reader.read_bits(1)? != 0;
        Ok(Self { part0 })
    }
}
impl Item040Part0 {
    pub fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(self.bit1 as u64, 1usize)?;
        writer.write_bits(self.bit2 as u64, 1usize)?;
        writer.write_bits(self.bit3 as u64, 1usize)?;
        writer.write_bits(self.bit4 as u64, 1usize)?;
        writer.write_bits(self.bit5 as u64, 1usize)?;
        writer.write_bits(self.bit6 as u64, 1usize)?;
        writer.write_bits(self.bit7 as u64, 1usize)?;
        Ok(())
    }
}
impl Encode for Item040 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        self.part0.encode(writer)?;
        writer.write_bits(0, 1)?;
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    Auto = 0u8,
    Manual = 1u8,
    Unknown(u8),
}
impl TryFrom<u8> for Mode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0u8 => Ok(Self::Auto),
            1u8 => Ok(Self::Manual),
            _ => Ok(Self::Unknown(value)),
        }
    }
}
impl From<Mode> for u8 {
    fn from(val: Mode) -> u8 {
        match val {
            Mode::Auto => 0u8,
            Mode::Manual => 1u8,
            Mode::Unknown(v) => v,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item050Part0 {
    pub a: u8,
    pub b: u8,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item050Part1 {
    pub mode: Mode,
    pub c: u8,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item050Part2 {
    pub optional_data: Option<u8>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item050 {
    pub part0: Item050Part0,
    pub part1: Option<Item050Part1>,
    pub part2: Option<Item050Part2>,
}
impl Item050Part0 {
    pub fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let a = reader.read_bits(2usize)? as u8;
        let b = reader.read_bits(3usize)? as u8;
        reader.read_bits(2usize)?;
        Ok(Self { a, b })
    }
}
impl Item050Part1 {
    pub fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let mode = {
            let value = reader.read_bits(2usize)? as u8;
            Mode::try_from(value).unwrap()
        };
        let c = reader.read_bits(5usize)? as u8;
        Ok(Self { mode, c })
    }
}
impl Item050Part2 {
    pub fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let optional_data = {
            let valid = reader.read_bits(1)? != 0;
            if valid {
                Some(reader.read_bits(6usize)? as u8)
            } else {
                reader.read_bits(6usize)?;
                None
            }
        };
        Ok(Self { optional_data })
    }
}
impl Decode for Item050 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let part0 = Item050Part0::decode(reader)?;
        let mut fx = reader.read_bits(1)? != 0;
        let part1 = if fx {
            let part = Item050Part1::decode(reader)?;
            fx = reader.read_bits(1)? != 0;
            Some(part)
        } else {
            None
        };
        let part2 = if fx {
            let part = Item050Part2::decode(reader)?;
            fx = reader.read_bits(1)? != 0;
            Some(part)
        } else {
            None
        };
        Ok(Self {
            part0,
            part1,
            part2,
        })
    }
}
impl Item050Part0 {
    pub fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(self.a as u64, 2usize)?;
        writer.write_bits(self.b as u64, 3usize)?;
        writer.write_bits(0, 2usize)?;
        Ok(())
    }
}
impl Item050Part1 {
    pub fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(u8::from(self.mode) as u64, 2usize)?;
        writer.write_bits(self.c as u64, 5usize)?;
        Ok(())
    }
}
impl Item050Part2 {
    pub fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        if let Some(value) = self.optional_data {
            writer.write_bits(1, 1)?;
            writer.write_bits(value as u64, 6usize)?;
        } else {
            writer.write_bits(0, 1)?;
            writer.write_bits(0, 6usize)?;
        }
        Ok(())
    }
}
impl Encode for Item050 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        self.part0.encode(writer)?;
        writer.write_bits(self.part1.is_some() as u64, 1)?;
        if let Some(ref part_data) = self.part1 {
            part_data.encode(writer)?;
            writer.write_bits(self.part2.is_some() as u64, 1)?;
        }
        if let Some(ref part_data) = self.part2 {
            part_data.encode(writer)?;
            writer.write_bits(0, 1)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item060 {
    pub altitude: u16,
    pub speed: u16,
}
impl Decode for Item060 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let _len = reader.read_bits(8)? as usize;
        let altitude = reader.read_bits(16usize)? as u16;
        let speed = reader.read_bits(16usize)? as u16;
        Ok(Self { altitude, speed })
    }
}
impl Encode for Item060 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(5usize as u64, 8)?;
        writer.write_bits(self.altitude as u64, 16usize)?;
        writer.write_bits(self.speed as u64, 16usize)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item070Element {
    pub azimuth: u16,
    pub range: u8,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item070 {
    pub items: Vec<Item070Element>,
}
impl Item070Element {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let azimuth = reader.read_bits(16usize)? as u16;
        let range = reader.read_bits(8usize)? as u8;
        Ok(Self { azimuth, range })
    }
}
impl Decode for Item070 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let mut items = Vec::with_capacity(5usize);
        for _ in 0..5usize {
            items.push(Item070Element::decode(reader)?);
        }
        Ok(Self { items })
    }
}
impl Item070Element {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(self.azimuth as u64, 16usize)?;
        writer.write_bits(self.range as u64, 8usize)?;
        Ok(())
    }
}
impl Encode for Item070 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        for item in &self.items {
            item.encode(writer)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TrackStatus {
    Confirmed = 0u8,
    Tentative = 1u8,
    Coast = 2u8,
    Unknown(u8),
}
impl TryFrom<u8> for TrackStatus {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0u8 => Ok(Self::Confirmed),
            1u8 => Ok(Self::Tentative),
            2u8 => Ok(Self::Coast),
            _ => Ok(Self::Unknown(value)),
        }
    }
}
impl From<TrackStatus> for u8 {
    fn from(val: TrackStatus) -> u8 {
        match val {
            TrackStatus::Confirmed => 0u8,
            TrackStatus::Tentative => 1u8,
            TrackStatus::Coast => 2u8,
            TrackStatus::Unknown(v) => v,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item080Element {
    pub track_status: Option<TrackStatus>,
    pub track_number: u16,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item080 {
    pub items: Vec<Item080Element>,
}
impl Item080Element {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let track_status = {
            let valid = reader.read_bits(1)? != 0;
            if valid {
                let value = reader.read_bits(2usize)? as u8;
                Some(TrackStatus::try_from(value).unwrap())
            } else {
                reader.read_bits(2usize)?;
                None
            }
        };
        let track_number = reader.read_bits(12usize)? as u16;
        reader.read_bits(1usize)?;
        Ok(Self {
            track_status,
            track_number,
        })
    }
}
impl Decode for Item080 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let mut items = Vec::with_capacity(3usize);
        for _ in 0..3usize {
            items.push(Item080Element::decode(reader)?);
        }
        Ok(Self { items })
    }
}
impl Item080Element {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        if let Some(value) = self.track_status {
            writer.write_bits(1, 1)?;
            writer.write_bits(u8::from(value) as u64, 2usize)?;
        } else {
            writer.write_bits(0, 1)?;
            writer.write_bits(0, 2usize)?;
        }
        writer.write_bits(self.track_number as u64, 12usize)?;
        writer.write_bits(0, 1usize)?;
        Ok(())
    }
}
impl Encode for Item080 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        for item in &self.items {
            item.encode(writer)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item090Sub0 {
    pub primary: u8,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item090Sub1 {
    pub secondary: u16,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item090 {
    pub sub0: Option<Item090Sub0>,
    pub sub1: Option<Item090Sub1>,
}
impl Decode for Item090Sub0 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let primary = reader.read_bits(8usize)? as u8;
        Ok(Self { primary })
    }
}
impl Decode for Item090Sub1 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let secondary = reader.read_bits(16usize)? as u16;
        Ok(Self { secondary })
    }
}
impl Item090 {
    pub fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let fspec = Fspec::read(reader)?;
        let mut reader = BitReader::new(reader);
        let sub0 = if fspec.is_set(0usize, 7u8) {
            Some(Item090Sub0::decode(&mut reader)?)
        } else {
            None
        };
        let sub1 = if fspec.is_set(0usize, 6u8) {
            Some(Item090Sub1::decode(&mut reader)?)
        } else {
            None
        };
        Ok(Self { sub0, sub1 })
    }
}
impl Encode for Item090Sub0 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(self.primary as u64, 8usize)?;
        Ok(())
    }
}
impl Encode for Item090Sub1 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(self.secondary as u64, 16usize)?;
        Ok(())
    }
}
impl Item090 {
    pub fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), DecodeError> {
        let mut fspec = Fspec::new();
        if self.sub0.is_some() {
            fspec.set(0usize, 7u8);
        }
        if self.sub1.is_some() {
            fspec.set(0usize, 6u8);
        }
        fspec.write(writer)?;
        let mut writer = BitWriter::new(writer);
        if let Some(ref sub_data) = self.sub0 {
            sub_data.encode(&mut writer)?;
        }
        if let Some(ref sub_data) = self.sub1 {
            sub_data.encode(&mut writer)?;
        }
        writer.flush()?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item100Sub0 {
    pub flags: u8,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item100Sub1 {
    pub data1: u16,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item100Sub2Part0 {
    pub ext_a: u8,
    pub ext_b: u8,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item100Sub2Part1 {
    pub ext_c: u8,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item100Sub2 {
    pub part0: Item100Sub2Part0,
    pub part1: Option<Item100Sub2Part1>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item100Sub3Element {
    pub rep_data: u8,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item100Sub3 {
    pub items: Vec<Item100Sub3Element>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item100 {
    pub sub0: Option<Item100Sub0>,
    pub sub1: Option<Item100Sub1>,
    pub sub2: Option<Item100Sub2>,
    pub sub3: Option<Item100Sub3>,
}
impl Decode for Item100Sub0 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let flags = reader.read_bits(8usize)? as u8;
        Ok(Self { flags })
    }
}
impl Decode for Item100Sub1 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let _len = reader.read_bits(8)? as usize;
        let data1 = reader.read_bits(16usize)? as u16;
        Ok(Self { data1 })
    }
}
impl Item100Sub2Part0 {
    pub fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let ext_a = reader.read_bits(4usize)? as u8;
        let ext_b = reader.read_bits(3usize)? as u8;
        Ok(Self { ext_a, ext_b })
    }
}
impl Item100Sub2Part1 {
    pub fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let ext_c = reader.read_bits(7usize)? as u8;
        Ok(Self { ext_c })
    }
}
impl Decode for Item100Sub2 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let part0 = Item100Sub2Part0::decode(reader)?;
        let mut fx = reader.read_bits(1)? != 0;
        let part1 = if fx {
            let part = Item100Sub2Part1::decode(reader)?;
            fx = reader.read_bits(1)? != 0;
            Some(part)
        } else {
            None
        };
        Ok(Self { part0, part1 })
    }
}
impl Item100Sub3Element {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let rep_data = reader.read_bits(8usize)? as u8;
        Ok(Self { rep_data })
    }
}
impl Decode for Item100Sub3 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let mut items = Vec::with_capacity(2usize);
        for _ in 0..2usize {
            items.push(Item100Sub3Element::decode(reader)?);
        }
        Ok(Self { items })
    }
}
impl Item100 {
    pub fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let fspec = Fspec::read(reader)?;
        let mut reader = BitReader::new(reader);
        let sub0 = if fspec.is_set(0usize, 7u8) {
            Some(Item100Sub0::decode(&mut reader)?)
        } else {
            None
        };
        let sub1 = if fspec.is_set(0usize, 6u8) {
            Some(Item100Sub1::decode(&mut reader)?)
        } else {
            None
        };
        let sub2 = if fspec.is_set(0usize, 5u8) {
            Some(Item100Sub2::decode(&mut reader)?)
        } else {
            None
        };
        let sub3 = if fspec.is_set(0usize, 4u8) {
            Some(Item100Sub3::decode(&mut reader)?)
        } else {
            None
        };
        Ok(Self {
            sub0,
            sub1,
            sub2,
            sub3,
        })
    }
}
impl Encode for Item100Sub0 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(self.flags as u64, 8usize)?;
        Ok(())
    }
}
impl Encode for Item100Sub1 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(3usize as u64, 8)?;
        writer.write_bits(self.data1 as u64, 16usize)?;
        Ok(())
    }
}
impl Item100Sub2Part0 {
    pub fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(self.ext_a as u64, 4usize)?;
        writer.write_bits(self.ext_b as u64, 3usize)?;
        Ok(())
    }
}
impl Item100Sub2Part1 {
    pub fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(self.ext_c as u64, 7usize)?;
        Ok(())
    }
}
impl Encode for Item100Sub2 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        self.part0.encode(writer)?;
        writer.write_bits(self.part1.is_some() as u64, 1)?;
        if let Some(ref part_data) = self.part1 {
            part_data.encode(writer)?;
            writer.write_bits(0, 1)?;
        }
        Ok(())
    }
}
impl Item100Sub3Element {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(self.rep_data as u64, 8usize)?;
        Ok(())
    }
}
impl Encode for Item100Sub3 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        for item in &self.items {
            item.encode(writer)?;
        }
        Ok(())
    }
}
impl Item100 {
    pub fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), DecodeError> {
        let mut fspec = Fspec::new();
        if self.sub0.is_some() {
            fspec.set(0usize, 7u8);
        }
        if self.sub1.is_some() {
            fspec.set(0usize, 6u8);
        }
        if self.sub2.is_some() {
            fspec.set(0usize, 5u8);
        }
        if self.sub3.is_some() {
            fspec.set(0usize, 4u8);
        }
        fspec.write(writer)?;
        let mut writer = BitWriter::new(writer);
        if let Some(ref sub_data) = self.sub0 {
            sub_data.encode(&mut writer)?;
        }
        if let Some(ref sub_data) = self.sub1 {
            sub_data.encode(&mut writer)?;
        }
        if let Some(ref sub_data) = self.sub2 {
            sub_data.encode(&mut writer)?;
        }
        if let Some(ref sub_data) = self.sub3 {
            sub_data.encode(&mut writer)?;
        }
        writer.flush()?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item110 {}
impl Decode for Item110 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        reader.read_bits(16usize)?;
        Ok(Self {})
    }
}
impl Encode for Item110 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(0, 16usize)?;
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Quality {
    High = 0u8,
    Medium = 1u8,
    Low = 2u8,
    Unknown(u8),
}
impl TryFrom<u8> for Quality {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0u8 => Ok(Self::High),
            1u8 => Ok(Self::Medium),
            2u8 => Ok(Self::Low),
            _ => Ok(Self::Unknown(value)),
        }
    }
}
impl From<Quality> for u8 {
    fn from(val: Quality) -> u8 {
        match val {
            Quality::High => 0u8,
            Quality::Medium => 1u8,
            Quality::Low => 2u8,
            Quality::Unknown(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TypeCode {
    TypeA = 0u8,
    TypeB = 1u8,
    TypeC = 2u8,
    Unknown(u8),
}
impl TryFrom<u8> for TypeCode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0u8 => Ok(Self::TypeA),
            1u8 => Ok(Self::TypeB),
            2u8 => Ok(Self::TypeC),
            _ => Ok(Self::Unknown(value)),
        }
    }
}
impl From<TypeCode> for u8 {
    fn from(val: TypeCode) -> u8 {
        match val {
            TypeCode::TypeA => 0u8,
            TypeCode::TypeB => 1u8,
            TypeCode::TypeC => 2u8,
            TypeCode::Unknown(v) => v,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Item120 {
    pub field1: u8,
    pub quality: Option<Quality>,
    pub type_code: TypeCode,
    pub field2: u16,
    pub optional_field: Option<u8>,
}
impl Decode for Item120 {
    fn decode<R: std::io::Read>(reader: &mut BitReader<R>) -> Result<Self, DecodeError> {
        let field1 = reader.read_bits(4usize)? as u8;
        let quality = {
            let valid = reader.read_bits(1)? != 0;
            if valid {
                let value = reader.read_bits(3usize)? as u8;
                Some(Quality::try_from(value).unwrap())
            } else {
                reader.read_bits(3usize)?;
                None
            }
        };
        reader.read_bits(3usize)?;
        let type_code = {
            let value = reader.read_bits(2usize)? as u8;
            TypeCode::try_from(value).unwrap()
        };
        let field2 = reader.read_bits(12usize)? as u16;
        let optional_field = {
            let valid = reader.read_bits(1)? != 0;
            if valid {
                Some(reader.read_bits(6usize)? as u8)
            } else {
                reader.read_bits(6usize)?;
                None
            }
        };
        Ok(Self {
            field1,
            quality,
            type_code,
            field2,
            optional_field,
        })
    }
}
impl Encode for Item120 {
    fn encode<W: std::io::Write>(&self, writer: &mut BitWriter<W>) -> Result<(), DecodeError> {
        writer.write_bits(self.field1 as u64, 4usize)?;
        if let Some(value) = self.quality {
            writer.write_bits(1, 1)?;
            writer.write_bits(u8::from(value) as u64, 3usize)?;
        } else {
            writer.write_bits(0, 1)?;
            writer.write_bits(0, 3usize)?;
        }
        writer.write_bits(0, 3usize)?;
        writer.write_bits(u8::from(self.type_code) as u64, 2usize)?;
        writer.write_bits(self.field2 as u64, 12usize)?;
        if let Some(value) = self.optional_field {
            writer.write_bits(1, 1)?;
            writer.write_bits(value as u64, 6usize)?;
        } else {
            writer.write_bits(0, 1)?;
            writer.write_bits(0, 6usize)?;
        }
        Ok(())
    }
}
