use core::fmt::Display;

pub trait Decode<'a, T> {
    fn decode(bytes: &'a [u8]) -> Result<T, DecodeError>;
}

#[derive(Debug)]
pub enum DecodeError {
    Unknown,
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Unknown error")
    }
}
