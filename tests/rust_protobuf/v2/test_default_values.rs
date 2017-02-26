//! Automatically generated rust module for 'test_default_values_pb.proto' file

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
pub enum EnumForDefaultValue {
    ONE = 1,
    TWO = 2,
    THREE = 3,
}

impl Default for EnumForDefaultValue {
    fn default() -> Self {
        EnumForDefaultValue::ONE
    }
}

impl From<i32> for EnumForDefaultValue {
    fn from(i: i32) -> Self {
        match i {
            1 => EnumForDefaultValue::ONE,
            2 => EnumForDefaultValue::TWO,
            3 => EnumForDefaultValue::THREE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestDefaultValues<'a> {
    pub double_field: f64,
    pub float_field: f32,
    pub int32_field: i32,
    pub int64_field: i64,
    pub uint32_field: u32,
    pub uint64_field: u64,
    pub sint32_field: i32,
    pub sint64_field: i64,
    pub fixed32_field: u32,
    pub fixed64_field: u64,
    pub sfixed32_field: i32,
    pub sfixed64_field: i64,
    pub bool_field: bool,
    pub string_field: Cow<'a, str>,
    pub bytes_field: Cow<'a, [u8]>,
    pub enum_field: EnumForDefaultValue,
    pub enum_field_without_default: Option<EnumForDefaultValue>,
}

impl<'a> TestDefaultValues<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = TestDefaultValues {
            double_field: 1f64,
            float_field: 2f32,
            int32_field: 3i32,
            int64_field: 4i64,
            uint32_field: 5u32,
            uint64_field: 6u64,
            sint32_field: 7i32,
            sint64_field: 8i64,
            fixed32_field: 9u32,
            fixed64_field: 10u64,
            sfixed32_field: 11i32,
            sfixed64_field: 12i64,
            bool_field: true,
            string_field: Cow::Borrowed("abc\n22"),
            bytes_field: Cow::Borrowed(b"cde\n33"),
            enum_field: EnumForDefaultValue::TWO,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.double_field = r.read_double(bytes)?,
                Ok(21) => msg.float_field = r.read_float(bytes)?,
                Ok(24) => msg.int32_field = r.read_int32(bytes)?,
                Ok(32) => msg.int64_field = r.read_int64(bytes)?,
                Ok(40) => msg.uint32_field = r.read_uint32(bytes)?,
                Ok(48) => msg.uint64_field = r.read_uint64(bytes)?,
                Ok(56) => msg.sint32_field = r.read_sint32(bytes)?,
                Ok(64) => msg.sint64_field = r.read_sint64(bytes)?,
                Ok(77) => msg.fixed32_field = r.read_fixed32(bytes)?,
                Ok(81) => msg.fixed64_field = r.read_fixed64(bytes)?,
                Ok(93) => msg.sfixed32_field = r.read_sfixed32(bytes)?,
                Ok(97) => msg.sfixed64_field = r.read_sfixed64(bytes)?,
                Ok(104) => msg.bool_field = r.read_bool(bytes)?,
                Ok(114) => msg.string_field = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(122) => msg.bytes_field = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(128) => msg.enum_field = r.read_enum(bytes)?,
                Ok(136) => msg.enum_field_without_default = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestDefaultValues<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.double_field == 1f64 { 0 } else { 1 + 8 }
        + if self.float_field == 2f32 { 0 } else { 1 + 4 }
        + if self.int32_field == 3i32 { 0 } else { 1 + sizeof_varint(*(&self.int32_field) as u64) }
        + if self.int64_field == 4i64 { 0 } else { 1 + sizeof_varint(*(&self.int64_field) as u64) }
        + if self.uint32_field == 5u32 { 0 } else { 1 + sizeof_varint(*(&self.uint32_field) as u64) }
        + if self.uint64_field == 6u64 { 0 } else { 1 + sizeof_varint(*(&self.uint64_field) as u64) }
        + if self.sint32_field == 7i32 { 0 } else { 1 + sizeof_sint32(*(&self.sint32_field)) }
        + if self.sint64_field == 8i64 { 0 } else { 1 + sizeof_sint64(*(&self.sint64_field)) }
        + if self.fixed32_field == 9u32 { 0 } else { 1 + 4 }
        + if self.fixed64_field == 10u64 { 0 } else { 1 + 8 }
        + if self.sfixed32_field == 11i32 { 0 } else { 1 + 4 }
        + if self.sfixed64_field == 12i64 { 0 } else { 1 + 8 }
        + if self.bool_field == true { 0 } else { 1 + sizeof_varint(*(&self.bool_field) as u64) }
        + if self.string_field == Cow::Borrowed("abc\n22") { 0 } else { 1 + sizeof_len((&self.string_field).len()) }
        + if self.bytes_field == Cow::Borrowed(b"cde\n33") { 0 } else { 1 + sizeof_len((&self.bytes_field).len()) }
        + if self.enum_field == EnumForDefaultValue::TWO { 0 } else { 2 + sizeof_varint(*(&self.enum_field) as u64) }
        + self.enum_field_without_default.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.double_field != 1f64 { w.write_with_tag(9, |w| w.write_double(*&self.double_field))?; }
        if self.float_field != 2f32 { w.write_with_tag(21, |w| w.write_float(*&self.float_field))?; }
        if self.int32_field != 3i32 { w.write_with_tag(24, |w| w.write_int32(*&self.int32_field))?; }
        if self.int64_field != 4i64 { w.write_with_tag(32, |w| w.write_int64(*&self.int64_field))?; }
        if self.uint32_field != 5u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.uint32_field))?; }
        if self.uint64_field != 6u64 { w.write_with_tag(48, |w| w.write_uint64(*&self.uint64_field))?; }
        if self.sint32_field != 7i32 { w.write_with_tag(56, |w| w.write_sint32(*&self.sint32_field))?; }
        if self.sint64_field != 8i64 { w.write_with_tag(64, |w| w.write_sint64(*&self.sint64_field))?; }
        if self.fixed32_field != 9u32 { w.write_with_tag(77, |w| w.write_fixed32(*&self.fixed32_field))?; }
        if self.fixed64_field != 10u64 { w.write_with_tag(81, |w| w.write_fixed64(*&self.fixed64_field))?; }
        if self.sfixed32_field != 11i32 { w.write_with_tag(93, |w| w.write_sfixed32(*&self.sfixed32_field))?; }
        if self.sfixed64_field != 12i64 { w.write_with_tag(97, |w| w.write_sfixed64(*&self.sfixed64_field))?; }
        if self.bool_field != true { w.write_with_tag(104, |w| w.write_bool(*&self.bool_field))?; }
        if self.string_field != Cow::Borrowed("abc\n22") { w.write_with_tag(114, |w| w.write_string(&**&self.string_field))?; }
        if self.bytes_field != Cow::Borrowed(b"cde\n33") { w.write_with_tag(122, |w| w.write_bytes(&**&self.bytes_field))?; }
        if self.enum_field != EnumForDefaultValue::TWO { w.write_with_tag(128, |w| w.write_enum(*&self.enum_field as i32))?; }
        if let Some(ref s) = self.enum_field_without_default { w.write_with_tag(136, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestExtremeDefaultValues {
    pub inf_double: f64,
    pub neg_inf_double: f64,
    pub nan_double: f64,
    pub inf_float: f32,
    pub neg_inf_float: f32,
    pub nan_float: f32,
}

impl TestExtremeDefaultValues {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = TestExtremeDefaultValues {
            inf_double: ::std::f64::INFINITY,
            neg_inf_double: ::std::f64::NEG_INFINITY,
            nan_double: ::std::f64::NAN,
            inf_float: ::std::f32::INFINITY,
            neg_inf_float: ::std::f32::NEG_INFINITY,
            nan_float: ::std::f32::NAN,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(113) => msg.inf_double = r.read_double(bytes)?,
                Ok(121) => msg.neg_inf_double = r.read_double(bytes)?,
                Ok(129) => msg.nan_double = r.read_double(bytes)?,
                Ok(141) => msg.inf_float = r.read_float(bytes)?,
                Ok(149) => msg.neg_inf_float = r.read_float(bytes)?,
                Ok(157) => msg.nan_float = r.read_float(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestExtremeDefaultValues {
    fn get_size(&self) -> usize {
        0
        + if self.inf_double == ::std::f64::INFINITY { 0 } else { 1 + 8 }
        + if self.neg_inf_double == ::std::f64::NEG_INFINITY { 0 } else { 1 + 8 }
        + if self.nan_double == ::std::f64::NAN { 0 } else { 2 + 8 }
        + if self.inf_float == ::std::f32::INFINITY { 0 } else { 2 + 4 }
        + if self.neg_inf_float == ::std::f32::NEG_INFINITY { 0 } else { 2 + 4 }
        + if self.nan_float == ::std::f32::NAN { 0 } else { 2 + 4 }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.inf_double != ::std::f64::INFINITY { w.write_with_tag(113, |w| w.write_double(*&self.inf_double))?; }
        if self.neg_inf_double != ::std::f64::NEG_INFINITY { w.write_with_tag(121, |w| w.write_double(*&self.neg_inf_double))?; }
        if self.nan_double != ::std::f64::NAN { w.write_with_tag(129, |w| w.write_double(*&self.nan_double))?; }
        if self.inf_float != ::std::f32::INFINITY { w.write_with_tag(141, |w| w.write_float(*&self.inf_float))?; }
        if self.neg_inf_float != ::std::f32::NEG_INFINITY { w.write_with_tag(149, |w| w.write_float(*&self.neg_inf_float))?; }
        if self.nan_float != ::std::f32::NAN { w.write_with_tag(157, |w| w.write_float(*&self.nan_float))?; }
        Ok(())
    }
}
