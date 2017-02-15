//! Automatically generated rust module for 'struct.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::io::{Write};
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct KeepTheFile {
}

impl KeepTheFile {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for KeepTheFile {
    fn get_size(&self) -> usize { 0 }

    fn write_message<W: Write>(&self, _: &mut Writer<W>) -> Result<()> { Ok(()) }
}