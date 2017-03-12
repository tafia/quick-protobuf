//! Automatically generated rust module for 'test_import_nested_imported_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{BytesReader, Result, MessageWrite};
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ContainerForNested { }

impl ContainerForNested {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ContainerForNested { }

pub mod mod_ContainerForNested {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NestedMessage { }

impl NestedMessage {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for NestedMessage { }

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

