//! Automatically generated rust module for 'test_enum_values_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{BytesReader, Result, MessageRead, MessageWrite};
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TestEnumValuesEnum {
    WINTER = 11,
    SPRING = 22,
    SUMMER = 33,
    AUTUMN = 44,
}

impl Default for TestEnumValuesEnum {
    fn default() -> Self {
        TestEnumValuesEnum::WINTER
    }
}

impl From<i32> for TestEnumValuesEnum {
    fn from(i: i32) -> Self {
        match i {
            11 => TestEnumValuesEnum::WINTER,
            22 => TestEnumValuesEnum::SPRING,
            33 => TestEnumValuesEnum::SUMMER,
            44 => TestEnumValuesEnum::AUTUMN,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for TestEnumValuesEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "WINTER" => TestEnumValuesEnum::WINTER,
            "SPRING" => TestEnumValuesEnum::SPRING,
            "SUMMER" => TestEnumValuesEnum::SUMMER,
            "AUTUMN" => TestEnumValuesEnum::AUTUMN,
            _ => Self::default(),
        }
    }
}

