//! Automatically generated rust module for 'test_nonunique_enum_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::io::{Write};
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageA {
}

impl MessageA {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for MessageA {
    fn get_size(&self) -> usize { 0 }

    fn write_message<W: Write>(&self, _: &mut Writer<W>) -> Result<()> { Ok(()) }
}

pub mod mod_MessageA {

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnumA {
    FOO = 0,
}

impl Default for EnumA {
    fn default() -> Self {
        EnumA::FOO
    }
}

impl From<i32> for EnumA {
    fn from(i: i32) -> Self {
        match i {
            0 => EnumA::FOO,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageB {
}

impl MessageB {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for MessageB {
    fn get_size(&self) -> usize { 0 }

    fn write_message<W: Write>(&self, _: &mut Writer<W>) -> Result<()> { Ok(()) }
}

pub mod mod_MessageB {

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnumB {
    FOO = 0,
}

impl Default for EnumB {
    fn default() -> Self {
        EnumB::FOO
    }
}

impl From<i32> for EnumB {
    fn from(i: i32) -> Self {
        match i {
            0 => EnumB::FOO,
            _ => Self::default(),
        }
    }
}

}
