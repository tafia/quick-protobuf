//! Automatically generated rust module for 'test_nonunique_enum_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unknown_lints)]
#![allow(clippy)]
#[cfg_attr(rustfmt, rustfmt_skip)]

use quick_protobuf::{BytesReader, Result, MessageWrite};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageA { }

impl MessageA {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for MessageA { }

pub mod mod_MessageA {


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
pub struct MessageB { }

impl MessageB {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for MessageB { }

pub mod mod_MessageB {


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
