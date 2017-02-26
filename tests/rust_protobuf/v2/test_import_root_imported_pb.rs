//! Automatically generated rust module for 'test_import_root_imported_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{BytesReader, Result, MessageWrite};

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

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ImportedMessage { }

impl ImportedMessage {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ImportedMessage { }
