use super::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Nonunique {}

impl<'a> MessageRead<'a> for Nonunique {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Nonunique {}
