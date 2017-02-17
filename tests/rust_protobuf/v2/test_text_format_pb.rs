//! Automatically generated rust module for 'test_text_format_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use std::io::Write;
use std::borrow::Cow;
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TestEnum {
    DARK = 1,
    LIGHT = 2,
}

impl Default for TestEnum {
    fn default() -> Self {
        TestEnum::DARK
    }
}

impl From<i32> for TestEnum {
    fn from(i: i32) -> Self {
        match i {
            1 => TestEnum::DARK,
            2 => TestEnum::LIGHT,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestMessage {
    pub value: Option<i32>,
}

impl TestMessage {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.value = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestMessage {
    fn get_size(&self) -> usize {
        0
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.value { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestTypes<'a> {
    pub double_singular: Option<f64>,
    pub float_singular: Option<f32>,
    pub int32_singular: Option<i32>,
    pub int64_singular: Option<i64>,
    pub uint32_singular: Option<u32>,
    pub uint64_singular: Option<u64>,
    pub sint32_singular: Option<i32>,
    pub sint64_singular: Option<i64>,
    pub fixed32_singular: Option<u32>,
    pub fixed64_singular: Option<u64>,
    pub sfixed32_singular: Option<i32>,
    pub sfixed64_singular: Option<i64>,
    pub bool_singular: Option<bool>,
    pub string_singular: Option<Cow<'a, str>>,
    pub bytes_singular: Option<Cow<'a, [u8]>>,
    pub test_enum_singular: Option<TestEnum>,
    pub test_message_singular: Option<TestMessage>,
    pub double_repeated: Vec<f64>,
    pub float_repeated: Vec<f32>,
    pub int32_repeated: Vec<i32>,
    pub int64_repeated: Vec<i64>,
    pub uint32_repeated: Vec<u32>,
    pub uint64_repeated: Vec<u64>,
    pub sint32_repeated: Vec<i32>,
    pub sint64_repeated: Vec<i64>,
    pub fixed32_repeated: Vec<u32>,
    pub fixed64_repeated: Vec<u64>,
    pub sfixed32_repeated: Vec<i32>,
    pub sfixed64_repeated: Vec<i64>,
    pub bool_repeated: Vec<bool>,
    pub string_repeated: Vec<Cow<'a, str>>,
    pub bytes_repeated: Vec<Cow<'a, [u8]>>,
    pub test_enum_repeated: Vec<TestEnum>,
    pub test_message_repeated: Vec<TestMessage>,
}

impl<'a> TestTypes<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.double_singular = Some(r.read_double(bytes)?),
                Ok(21) => msg.float_singular = Some(r.read_float(bytes)?),
                Ok(24) => msg.int32_singular = Some(r.read_int32(bytes)?),
                Ok(32) => msg.int64_singular = Some(r.read_int64(bytes)?),
                Ok(40) => msg.uint32_singular = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.uint64_singular = Some(r.read_uint64(bytes)?),
                Ok(56) => msg.sint32_singular = Some(r.read_sint32(bytes)?),
                Ok(64) => msg.sint64_singular = Some(r.read_sint64(bytes)?),
                Ok(77) => msg.fixed32_singular = Some(r.read_fixed32(bytes)?),
                Ok(81) => msg.fixed64_singular = Some(r.read_fixed64(bytes)?),
                Ok(93) => msg.sfixed32_singular = Some(r.read_sfixed32(bytes)?),
                Ok(97) => msg.sfixed64_singular = Some(r.read_sfixed64(bytes)?),
                Ok(104) => msg.bool_singular = Some(r.read_bool(bytes)?),
                Ok(114) => msg.string_singular = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(122) => msg.bytes_singular = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(128) => msg.test_enum_singular = Some(r.read_enum(bytes)?),
                Ok(138) => msg.test_message_singular = Some(r.read_message(bytes, TestMessage::from_reader)?),
                Ok(249) => msg.double_repeated.push(r.read_double(bytes)?),
                Ok(261) => msg.float_repeated.push(r.read_float(bytes)?),
                Ok(264) => msg.int32_repeated.push(r.read_int32(bytes)?),
                Ok(272) => msg.int64_repeated.push(r.read_int64(bytes)?),
                Ok(280) => msg.uint32_repeated.push(r.read_uint32(bytes)?),
                Ok(288) => msg.uint64_repeated.push(r.read_uint64(bytes)?),
                Ok(296) => msg.sint32_repeated.push(r.read_sint32(bytes)?),
                Ok(304) => msg.sint64_repeated.push(r.read_sint64(bytes)?),
                Ok(317) => msg.fixed32_repeated.push(r.read_fixed32(bytes)?),
                Ok(321) => msg.fixed64_repeated.push(r.read_fixed64(bytes)?),
                Ok(333) => msg.sfixed32_repeated.push(r.read_sfixed32(bytes)?),
                Ok(337) => msg.sfixed64_repeated.push(r.read_sfixed64(bytes)?),
                Ok(344) => msg.bool_repeated.push(r.read_bool(bytes)?),
                Ok(354) => msg.string_repeated.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(362) => msg.bytes_repeated.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(368) => msg.test_enum_repeated.push(r.read_enum(bytes)?),
                Ok(378) => msg.test_message_repeated.push(r.read_message(bytes, TestMessage::from_reader)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestTypes<'a> {
    fn get_size(&self) -> usize {
        0
        + self.double_singular.as_ref().map_or(0, |_| 1 + 8)
        + self.float_singular.as_ref().map_or(0, |_| 1 + 4)
        + self.int32_singular.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.int64_singular.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uint32_singular.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uint64_singular.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sint32_singular.as_ref().map_or(0, |m| 1 + sizeof_sint32(*(m)))
        + self.sint64_singular.as_ref().map_or(0, |m| 1 + sizeof_sint64(*(m)))
        + self.fixed32_singular.as_ref().map_or(0, |_| 1 + 4)
        + self.fixed64_singular.as_ref().map_or(0, |_| 1 + 8)
        + self.sfixed32_singular.as_ref().map_or(0, |_| 1 + 4)
        + self.sfixed64_singular.as_ref().map_or(0, |_| 1 + 8)
        + self.bool_singular.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.string_singular.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.bytes_singular.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.test_enum_singular.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.test_message_singular.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + (2 + 8) * self.double_repeated.len()
        + (2 + 4) * self.float_repeated.len()
        + self.int32_repeated.iter().map(|s| 2 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.int64_repeated.iter().map(|s| 2 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.uint32_repeated.iter().map(|s| 2 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.uint64_repeated.iter().map(|s| 2 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.sint32_repeated.iter().map(|s| 2 + sizeof_sint32(*(s))).sum::<usize>()
        + self.sint64_repeated.iter().map(|s| 2 + sizeof_sint64(*(s))).sum::<usize>()
        + (2 + 4) * self.fixed32_repeated.len()
        + (2 + 8) * self.fixed64_repeated.len()
        + (2 + 4) * self.sfixed32_repeated.len()
        + (2 + 8) * self.sfixed64_repeated.len()
        + self.bool_repeated.iter().map(|s| 2 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.string_repeated.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + self.bytes_repeated.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + self.test_enum_repeated.iter().map(|s| 2 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.test_message_repeated.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.double_singular { w.write_with_tag(9, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.float_singular { w.write_with_tag(21, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.int32_singular { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.int64_singular { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.uint32_singular { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.uint64_singular { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.sint32_singular { w.write_with_tag(56, |w| w.write_sint32(*s))?; }
        if let Some(ref s) = self.sint64_singular { w.write_with_tag(64, |w| w.write_sint64(*s))?; }
        if let Some(ref s) = self.fixed32_singular { w.write_with_tag(77, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.fixed64_singular { w.write_with_tag(81, |w| w.write_fixed64(*s))?; }
        if let Some(ref s) = self.sfixed32_singular { w.write_with_tag(93, |w| w.write_sfixed32(*s))?; }
        if let Some(ref s) = self.sfixed64_singular { w.write_with_tag(97, |w| w.write_sfixed64(*s))?; }
        if let Some(ref s) = self.bool_singular { w.write_with_tag(104, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.string_singular { w.write_with_tag(114, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.bytes_singular { w.write_with_tag(122, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.test_enum_singular { w.write_with_tag(128, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.test_message_singular { w.write_with_tag(138, |w| w.write_message(s))?; }
        for s in &self.double_repeated { w.write_with_tag(249, |w| w.write_double(*s))?; }
        for s in &self.float_repeated { w.write_with_tag(261, |w| w.write_float(*s))?; }
        for s in &self.int32_repeated { w.write_with_tag(264, |w| w.write_int32(*s))?; }
        for s in &self.int64_repeated { w.write_with_tag(272, |w| w.write_int64(*s))?; }
        for s in &self.uint32_repeated { w.write_with_tag(280, |w| w.write_uint32(*s))?; }
        for s in &self.uint64_repeated { w.write_with_tag(288, |w| w.write_uint64(*s))?; }
        for s in &self.sint32_repeated { w.write_with_tag(296, |w| w.write_sint32(*s))?; }
        for s in &self.sint64_repeated { w.write_with_tag(304, |w| w.write_sint64(*s))?; }
        for s in &self.fixed32_repeated { w.write_with_tag(317, |w| w.write_fixed32(*s))?; }
        for s in &self.fixed64_repeated { w.write_with_tag(321, |w| w.write_fixed64(*s))?; }
        for s in &self.sfixed32_repeated { w.write_with_tag(333, |w| w.write_sfixed32(*s))?; }
        for s in &self.sfixed64_repeated { w.write_with_tag(337, |w| w.write_sfixed64(*s))?; }
        for s in &self.bool_repeated { w.write_with_tag(344, |w| w.write_bool(*s))?; }
        for s in &self.string_repeated { w.write_with_tag(354, |w| w.write_string(&**s))?; }
        for s in &self.bytes_repeated { w.write_with_tag(362, |w| w.write_bytes(&**s))?; }
        for s in &self.test_enum_repeated { w.write_with_tag(368, |w| w.write_enum(*s as i32))?; }
        for s in &self.test_message_repeated { w.write_with_tag(378, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestTextFormatRustIdentifier {
    pub const_pb: Option<bool>,
}

impl TestTextFormatRustIdentifier {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.const_pb = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestTextFormatRustIdentifier {
    fn get_size(&self) -> usize {
        0
        + self.const_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.const_pb { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        Ok(())
    }
}
