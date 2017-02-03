//! Automatically generated rust module for 'data_types.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::io::{Write};
use std::borrow::Cow;
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

use super::data_types_import::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FooEnum {
    FIRST_VALUE = 1,
    SECOND_VALUE = 2,
}

impl Default for FooEnum {
    fn default() -> Self {
        FooEnum::FIRST_VALUE
    }
}

impl From<i32> for FooEnum {
    fn from(i: i32) -> Self {
        match i {
            1 => FooEnum::FIRST_VALUE,
            2 => FooEnum::SECOND_VALUE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BarMessage {
    pub b_required_int32: i32,
}

impl BarMessage {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.b_required_int32 = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BarMessage {
    fn get_size(&self) -> usize {
        1 + sizeof_int32(self.b_required_int32)
    }

    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {
        r.write_int32_with_tag(8, self.b_required_int32)?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FooMessage<'a> {
    pub f_int32: Option<i32>,
    pub f_int64: Option<i64>,
    pub f_uint32: Option<u32>,
    pub f_uint64: Option<u64>,
    pub f_sint32: Option<i32>,
    pub f_sint64: Option<i64>,
    pub f_bool: Option<bool>,
    pub f_FooEnum: Option<FooEnum>,
    pub f_fixed64: Option<u64>,
    pub f_sfixed64: Option<i64>,
    pub f_fixed32: Option<u32>,
    pub f_sfixed32: Option<i32>,
    pub f_double: Option<f64>,
    pub f_float: Option<f32>,
    pub f_bytes: Option<Cow<'a, [u8]>>,
    pub f_string: Option<Cow<'a, str>>,
    pub f_self_message: Option<Box<FooMessage<'a>>>,
    pub f_bar_message: Option<BarMessage>,
    pub f_repeated_int32: Vec<i32>,
    pub f_repeated_packed_int32: Vec<i32>,
    pub f_imported: Option<ImportedMessage>,
}

impl<'a> FooMessage<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.f_int32 = Some(r.read_int32(bytes)?),
                Ok(16) => msg.f_int64 = Some(r.read_int64(bytes)?),
                Ok(24) => msg.f_uint32 = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.f_uint64 = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.f_sint32 = Some(r.read_sint32(bytes)?),
                Ok(48) => msg.f_sint64 = Some(r.read_sint64(bytes)?),
                Ok(56) => msg.f_bool = Some(r.read_bool(bytes)?),
                Ok(64) => msg.f_FooEnum = Some(r.read_enum(bytes)?),
                Ok(73) => msg.f_fixed64 = Some(r.read_fixed64(bytes)?),
                Ok(81) => msg.f_sfixed64 = Some(r.read_sfixed64(bytes)?),
                Ok(93) => msg.f_fixed32 = Some(r.read_fixed32(bytes)?),
                Ok(101) => msg.f_sfixed32 = Some(r.read_sfixed32(bytes)?),
                Ok(105) => msg.f_double = Some(r.read_double(bytes)?),
                Ok(117) => msg.f_float = Some(r.read_float(bytes)?),
                Ok(122) => msg.f_bytes = Some(Cow::Borrowed(r.read_bytes(bytes)?)),
                Ok(130) => msg.f_string = Some(Cow::Borrowed(r.read_string(bytes)?)),
                Ok(138) => msg.f_self_message = Some(Box::new(r.read_message(bytes, FooMessage::from_reader)?)),
                Ok(146) => msg.f_bar_message = Some(r.read_message(bytes, BarMessage::from_reader)?),
                Ok(152) => msg.f_repeated_int32.push(r.read_int32(bytes)?),
                Ok(162) => msg.f_repeated_packed_int32 = r.read_packed(bytes, |r, bytes| r.read_int32(bytes))?,
                Ok(168) => msg.f_imported = Some(r.read_message(bytes, ImportedMessage::from_reader)?),
//                 Ok(168) => msg.f_imported = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FooMessage<'a> {
    fn get_size(&self) -> usize {
        self.f_int32.as_ref().map_or(0, |m| 1 + sizeof_int32(*m))
        + self.f_int64.as_ref().map_or(0, |m| 1 + sizeof_int64(*m))
        + self.f_uint32.as_ref().map_or(0, |m| 1 + sizeof_uint32(*m))
        + self.f_uint64.as_ref().map_or(0, |m| 1 + sizeof_uint64(*m))
        + self.f_sint32.as_ref().map_or(0, |m| 1 + sizeof_sint32(*m))
        + self.f_sint64.as_ref().map_or(0, |m| 1 + sizeof_sint64(*m))
        + self.f_bool.as_ref().map_or(0, |m| 1 + sizeof_bool(*m))
        + self.f_FooEnum.as_ref().map_or(0, |m| 1 + sizeof_enum(*m as i32))
        + self.f_fixed64.as_ref().map_or(0, |_| 1 + 8)
        + self.f_sfixed64.as_ref().map_or(0, |_| 1 + 8)
        + self.f_fixed32.as_ref().map_or(0, |_| 1 + 4)
        + self.f_sfixed32.as_ref().map_or(0, |_| 1 + 4)
        + self.f_double.as_ref().map_or(0, |_| 1 + 8)
        + self.f_float.as_ref().map_or(0, |_| 1 + 4)
        + self.f_bytes.as_ref().map_or(0, |m| 1 + sizeof_var_length(m.len()))
        + self.f_string.as_ref().map_or(0, |m| 2 + sizeof_var_length(m.len()))
        + self.f_self_message.as_ref().map_or(0, |m| 2 + sizeof_var_length(m.get_size()))
        + self.f_bar_message.as_ref().map_or(0, |m| 2 + sizeof_var_length(m.get_size()))
        + self.f_repeated_int32.iter().map(|s| 2 + sizeof_int32(*s)).sum::<usize>()
        + if self.f_repeated_packed_int32.is_empty() { 0 } else { 2 + sizeof_var_length(self.f_repeated_packed_int32.iter().map(|s| sizeof_int32(*s)).sum::<usize>()) }
        + self.f_imported.as_ref().map_or(0, |m| 2 + sizeof_var_length(m.get_size()))
//         + self.f_imported.as_ref().map_or(0, |m| 2 + sizeof_enum(*m as i32))
    }

    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.f_int32 { r.write_int32_with_tag(8, *s)?; }
        if let Some(ref s) = self.f_int64 { r.write_int64_with_tag(16, *s)?; }
        if let Some(ref s) = self.f_uint32 { r.write_uint32_with_tag(24, *s)?; }
        if let Some(ref s) = self.f_uint64 { r.write_uint64_with_tag(32, *s)?; }
        if let Some(ref s) = self.f_sint32 { r.write_sint32_with_tag(40, *s)?; }
        if let Some(ref s) = self.f_sint64 { r.write_sint64_with_tag(48, *s)?; }
        if let Some(ref s) = self.f_bool { r.write_bool_with_tag(56, *s)?; }
        if let Some(ref s) = self.f_FooEnum { r.write_enum_with_tag(64, *s as i32)?; }
        if let Some(ref s) = self.f_fixed64 { r.write_fixed64_with_tag(73, *s)?; }
        if let Some(ref s) = self.f_sfixed64 { r.write_sfixed64_with_tag(81, *s)?; }
        if let Some(ref s) = self.f_fixed32 { r.write_fixed32_with_tag(93, *s)?; }
        if let Some(ref s) = self.f_sfixed32 { r.write_sfixed32_with_tag(101, *s)?; }
        if let Some(ref s) = self.f_double { r.write_double_with_tag(105, *s)?; }
        if let Some(ref s) = self.f_float { r.write_float_with_tag(117, *s)?; }
        if let Some(ref s) = self.f_bytes { r.write_bytes_with_tag(122, s)?; }
        if let Some(ref s) = self.f_string { r.write_string_with_tag(130, s)?; }
        if let Some(ref s) = self.f_self_message { r.write_message_with_tag(138, &**s)?; }
        if let Some(ref s) = self.f_bar_message { r.write_message_with_tag(146, s)?; }
        for s in &self.f_repeated_int32 { r.write_int32_with_tag(152, *s)? }
        r.write_packed_repeated_field_with_tag(162, &self.f_repeated_packed_int32, |r, m| r.write_int32(*m), &|m| sizeof_int32(*m))?;
        if let Some(ref s) = self.f_imported { r.write_message_with_tag(168, s)?; }
//         if let Some(ref s) = self.f_imported { r.write_enum_with_tag(168, *s as i32)?; }
        Ok(())
    }
}
