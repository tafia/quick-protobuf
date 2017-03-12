//! Automatically generated rust module for 'test_import_root_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ContainsImported {
    pub imported_message: Option<test_import_root_imported_pb::ImportedMessage>,
    pub imported_enum: Option<test_import_root_imported_pb::ImportedEnum>,
}

impl ContainsImported {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.imported_message = Some(r.read_message(bytes, test_import_root_imported_pb::ImportedMessage::from_reader)?),
                Ok(16) => msg.imported_enum = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ContainsImported {
    fn get_size(&self) -> usize {
        0
        + self.imported_message.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.imported_enum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.imported_message { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.imported_enum { w.write_with_tag(16, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

