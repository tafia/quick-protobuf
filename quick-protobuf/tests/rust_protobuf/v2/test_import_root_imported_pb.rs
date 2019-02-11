use super::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImportedEnum {
    SOMETHING = 1,
}

impl Default for ImportedEnum {
    fn default() -> Self {
        ImportedEnum::SOMETHING
    }
}

impl From<i32> for ImportedEnum {
    fn from(i: i32) -> Self {
        match i {
            1 => ImportedEnum::SOMETHING,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ImportedEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "SOMETHING" => ImportedEnum::SOMETHING,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ImportedMessage {}

impl<'a> MessageRead<'a> for ImportedMessage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ImportedMessage {}
