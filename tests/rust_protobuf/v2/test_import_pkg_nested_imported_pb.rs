//! Automatically generated rust module for 'test_import_pkg_nested_imported_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod mod_foo {
pub mod mod_baz {

use std::io::{Write};
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ContainerForNested {
}

impl ContainerForNested {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ContainerForNested {
    fn get_size(&self) -> usize { 0 }

    fn write_message<W: Write>(&self, _: &mut Writer<W>) -> Result<()> { Ok(()) }
}

pub mod mod_ContainerForNested {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NestedMessage {
}

impl NestedMessage {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for NestedMessage {
    fn get_size(&self) -> usize { 0 }

    fn write_message<W: Write>(&self, _: &mut Writer<W>) -> Result<()> { Ok(()) }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NestedEnum {
    RED = 1,
}

impl Default for NestedEnum {
    fn default() -> Self {
        NestedEnum::RED
    }
}

impl From<i32> for NestedEnum {
    fn from(i: i32) -> Self {
        match i {
            1 => NestedEnum::RED,
            _ => Self::default(),
        }
    }
}

}

}
}
