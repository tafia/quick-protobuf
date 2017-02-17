//! Automatically generated rust module for 'struct.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use quick_protobuf::{BytesReader, Result, MessageWrite};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct KeepTheFile { }

impl KeepTheFile {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for KeepTheFile { }
