//! Automatically generated rust module for 'test_oneof_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unknown_lints)]
#![allow(clippy)]
#[cfg_attr(rustfmt, rustfmt_skip)]

use std::io::Write;
use std::borrow::Cow;
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnumForOneof {
    Z = 0,
    A = 10,
}

impl Default for EnumForOneof {
    fn default() -> Self {
        EnumForOneof::Z
    }
}

impl From<i32> for EnumForOneof {
    fn from(i: i32) -> Self {
        match i {
            0 => EnumForOneof::Z,
            10 => EnumForOneof::A,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageForOneof {
    pub f: Option<i32>,
}

impl MessageForOneof {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.f = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MessageForOneof {
    fn get_size(&self) -> usize {
        0
        + self.f.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.f { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestOneof<'a> {
    pub s: Option<Cow<'a, str>>,
    pub one: mod_TestOneof::OneOfone<'a>,
}

impl<'a> TestOneof<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(234) => msg.s = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(9) => msg.one = mod_TestOneof::OneOfone::double_field(r.read_double(bytes)?),
                Ok(21) => msg.one = mod_TestOneof::OneOfone::float_field(r.read_float(bytes)?),
                Ok(24) => msg.one = mod_TestOneof::OneOfone::int32_field(r.read_int32(bytes)?),
                Ok(32) => msg.one = mod_TestOneof::OneOfone::int64_field(r.read_int64(bytes)?),
                Ok(40) => msg.one = mod_TestOneof::OneOfone::uint32_field(r.read_uint32(bytes)?),
                Ok(48) => msg.one = mod_TestOneof::OneOfone::uint64_field(r.read_uint64(bytes)?),
                Ok(56) => msg.one = mod_TestOneof::OneOfone::sint32_field(r.read_sint32(bytes)?),
                Ok(64) => msg.one = mod_TestOneof::OneOfone::sint64_field(r.read_sint64(bytes)?),
                Ok(77) => msg.one = mod_TestOneof::OneOfone::fixed32_field(r.read_fixed32(bytes)?),
                Ok(81) => msg.one = mod_TestOneof::OneOfone::fixed64_field(r.read_fixed64(bytes)?),
                Ok(93) => msg.one = mod_TestOneof::OneOfone::sfixed32_field(r.read_sfixed32(bytes)?),
                Ok(97) => msg.one = mod_TestOneof::OneOfone::sfixed64_field(r.read_sfixed64(bytes)?),
                Ok(104) => msg.one = mod_TestOneof::OneOfone::bool_field(r.read_bool(bytes)?),
                Ok(114) => msg.one = mod_TestOneof::OneOfone::string_field(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(122) => msg.one = mod_TestOneof::OneOfone::bytes_field(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(128) => msg.one = mod_TestOneof::OneOfone::enum_field(r.read_enum(bytes)?),
                Ok(138) => msg.one = mod_TestOneof::OneOfone::message_field(r.read_message(bytes, MessageForOneof::from_reader)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestOneof<'a> {
    fn get_size(&self) -> usize {
        0
        + self.s.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + match self.one {
            mod_TestOneof::OneOfone::double_field(_) => 1 + 8,
            mod_TestOneof::OneOfone::float_field(_) => 1 + 4,
            mod_TestOneof::OneOfone::int32_field(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_TestOneof::OneOfone::int64_field(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_TestOneof::OneOfone::uint32_field(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_TestOneof::OneOfone::uint64_field(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_TestOneof::OneOfone::sint32_field(ref m) => 1 + sizeof_sint32(*(m)),
            mod_TestOneof::OneOfone::sint64_field(ref m) => 1 + sizeof_sint64(*(m)),
            mod_TestOneof::OneOfone::fixed32_field(_) => 1 + 4,
            mod_TestOneof::OneOfone::fixed64_field(_) => 1 + 8,
            mod_TestOneof::OneOfone::sfixed32_field(_) => 1 + 4,
            mod_TestOneof::OneOfone::sfixed64_field(_) => 1 + 8,
            mod_TestOneof::OneOfone::bool_field(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_TestOneof::OneOfone::string_field(ref m) => 1 + sizeof_len((m).len()),
            mod_TestOneof::OneOfone::bytes_field(ref m) => 1 + sizeof_len((m).len()),
            mod_TestOneof::OneOfone::enum_field(ref m) => 2 + sizeof_varint(*(m) as u64),
            mod_TestOneof::OneOfone::message_field(ref m) => 2 + sizeof_len((m).get_size()),
            mod_TestOneof::OneOfone::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.s { w.write_with_tag(234, |w| w.write_string(&**s))?; }
        match self.one {            mod_TestOneof::OneOfone::double_field(ref m) => { w.write_with_tag(9, |w| w.write_double(*m))? },
            mod_TestOneof::OneOfone::float_field(ref m) => { w.write_with_tag(21, |w| w.write_float(*m))? },
            mod_TestOneof::OneOfone::int32_field(ref m) => { w.write_with_tag(24, |w| w.write_int32(*m))? },
            mod_TestOneof::OneOfone::int64_field(ref m) => { w.write_with_tag(32, |w| w.write_int64(*m))? },
            mod_TestOneof::OneOfone::uint32_field(ref m) => { w.write_with_tag(40, |w| w.write_uint32(*m))? },
            mod_TestOneof::OneOfone::uint64_field(ref m) => { w.write_with_tag(48, |w| w.write_uint64(*m))? },
            mod_TestOneof::OneOfone::sint32_field(ref m) => { w.write_with_tag(56, |w| w.write_sint32(*m))? },
            mod_TestOneof::OneOfone::sint64_field(ref m) => { w.write_with_tag(64, |w| w.write_sint64(*m))? },
            mod_TestOneof::OneOfone::fixed32_field(ref m) => { w.write_with_tag(77, |w| w.write_fixed32(*m))? },
            mod_TestOneof::OneOfone::fixed64_field(ref m) => { w.write_with_tag(81, |w| w.write_fixed64(*m))? },
            mod_TestOneof::OneOfone::sfixed32_field(ref m) => { w.write_with_tag(93, |w| w.write_sfixed32(*m))? },
            mod_TestOneof::OneOfone::sfixed64_field(ref m) => { w.write_with_tag(97, |w| w.write_sfixed64(*m))? },
            mod_TestOneof::OneOfone::bool_field(ref m) => { w.write_with_tag(104, |w| w.write_bool(*m))? },
            mod_TestOneof::OneOfone::string_field(ref m) => { w.write_with_tag(114, |w| w.write_string(&**m))? },
            mod_TestOneof::OneOfone::bytes_field(ref m) => { w.write_with_tag(122, |w| w.write_bytes(&**m))? },
            mod_TestOneof::OneOfone::enum_field(ref m) => { w.write_with_tag(128, |w| w.write_enum(*m as i32))? },
            mod_TestOneof::OneOfone::message_field(ref m) => { w.write_with_tag(138, |w| w.write_message(m))? },
            mod_TestOneof::OneOfone::None => {},
    }        Ok(())
    }
}

pub mod mod_TestOneof {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfone<'a> {
    double_field(f64),
    float_field(f32),
    int32_field(i32),
    int64_field(i64),
    uint32_field(u32),
    uint64_field(u64),
    sint32_field(i32),
    sint64_field(i64),
    fixed32_field(u32),
    fixed64_field(u64),
    sfixed32_field(i32),
    sfixed64_field(i64),
    bool_field(bool),
    string_field(Cow<'a, str>),
    bytes_field(Cow<'a, [u8]>),
    enum_field(EnumForOneof),
    message_field(MessageForOneof),
    None,
}

impl<'a> Default for OneOfone<'a> {
    fn default() -> Self {
        OneOfone::None
    }
}

}
