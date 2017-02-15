//! Automatically generated rust module for 'data_types_unit.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use quick_protobuf::{BytesReader, Result, MessageWrite};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum test {
    a = 10,
}

impl Default for test {
    fn default() -> Self {
        test::a
    }
}

impl From<i32> for test {
    fn from(i: i32) -> Self {
        match i {
            10 => test::a,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct unit_message { }

impl unit_message {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for unit_message { }
