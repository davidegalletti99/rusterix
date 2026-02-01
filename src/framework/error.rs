#![allow(dead_code)]
#![allow(unused)]
use std::fmt;

#[derive(Debug)]
pub enum DecodeError {
    Io(std::io::Error),
    InvalidData(&'static str),
}

impl From<std::io::Error> for DecodeError {
    fn from(err: std::io::Error) -> Self {
        DecodeError::Io(err)
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::Io(e) => write!(f, "IO error: {}", e),
            DecodeError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}
