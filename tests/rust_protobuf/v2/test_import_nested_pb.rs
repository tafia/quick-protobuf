//! Automatically generated rust module for 'test_import_nested_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::io::{Write};
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

use super::test_import_nested_imported_pb::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ContainsImportedNested {
    pub m: Option<mod_ContainerForNested::NestedMessage>,
    pub e: Option<mod_ContainerForNested::NestedEnum>,
}

impl ContainsImportedNested {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.m = Some(r.read_message(bytes, mod_ContainerForNested::NestedMessage::from_reader)?),
                Ok(16) => msg.e = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ContainsImportedNested {
    fn get_size(&self) -> usize {
        0
        + self.m.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.e.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.m { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.e { w.write_with_tag(16, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}
