//! Automatically generated rust module for 'data_types_import.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod a {
pub mod b {

use std::io::{Write};
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ImportedMessage {
    pub i: Option<bool>,
}

impl ImportedMessage {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.i = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ImportedMessage {
    fn get_size(&self) -> usize {
        self.i.as_ref().map_or(0, |m| 1 + sizeof_bool(*m))
    }

    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.i { r.write_bool_with_tag(8, *s)?; }
        Ok(())
    }
}

}
}
