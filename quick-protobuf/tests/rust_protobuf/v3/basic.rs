// Automatically generated rust module for 'test_basic_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use std::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TestEnumDescriptor {
    UNKNOWN = 0,
    RED = 1,
    BLUE = 2,
    GREEN = 3,
}

impl Default for TestEnumDescriptor {
    fn default() -> Self {
        TestEnumDescriptor::UNKNOWN
    }
}

impl From<i32> for TestEnumDescriptor {
    fn from(i: i32) -> Self {
        match i {
            0 => TestEnumDescriptor::UNKNOWN,
            1 => TestEnumDescriptor::RED,
            2 => TestEnumDescriptor::BLUE,
            3 => TestEnumDescriptor::GREEN,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for TestEnumDescriptor {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => TestEnumDescriptor::UNKNOWN,
            "RED" => TestEnumDescriptor::RED,
            "BLUE" => TestEnumDescriptor::BLUE,
            "GREEN" => TestEnumDescriptor::GREEN,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test1 {
    pub a: i32,
}

impl<'a> MessageRead<'a> for Test1 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.a = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Test1 {
    fn get_size(&self) -> usize {
        0
        + if self.a == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.a) as u64) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.a != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.a))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test2<'a> {
    pub b: &'a str,
}

impl<'a> MessageRead<'a> for Test2<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.b = r.read_string(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Test2<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.b == "" { 0 } else { 1 + sizeof_len((&self.b).len()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.b != "" { w.write_with_tag(18, |w| w.write_string(&**&self.b))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test3 {
    pub c: Option<basic::Test1>,
}

impl<'a> MessageRead<'a> for Test3 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(26) => msg.c = Some(r.read_message::<basic::Test1>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Test3 {
    fn get_size(&self) -> usize {
        0
        + self.c.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.c { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test4 {
    pub d: Vec<i32>,
}

impl<'a> MessageRead<'a> for Test4 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(34) => msg.d = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Test4 {
    fn get_size(&self) -> usize {
        0
        + if self.d.is_empty() { 0 } else { 1 + sizeof_len(self.d.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(34, &self.d, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestPackedUnpacked {
    pub unpacked: Vec<i32>,
    pub packed: Vec<i32>,
}

impl<'a> MessageRead<'a> for TestPackedUnpacked {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(32) => msg.unpacked.push(r.read_int32(bytes)?),
                Ok(42) => msg.packed = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestPackedUnpacked {
    fn get_size(&self) -> usize {
        0
        + self.unpacked.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + if self.packed.is_empty() { 0 } else { 1 + sizeof_len(self.packed.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.unpacked { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        w.write_packed_with_tag(42, &self.packed, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestEmpty {
    pub foo: i32,
}

impl<'a> MessageRead<'a> for TestEmpty {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.foo = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestEmpty {
    fn get_size(&self) -> usize {
        0
        + if self.foo == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.foo) as u64) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.foo != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.foo))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test {
    pub b: bool,
}

impl<'a> MessageRead<'a> for Test {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(40) => msg.b = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Test {
    fn get_size(&self) -> usize {
        0
        + if self.b == false { 0 } else { 1 + sizeof_varint(*(&self.b) as u64) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.b != false { w.write_with_tag(40, |w| w.write_bool(*&self.b))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestUnknownFields {
    pub a: i32,
}

impl<'a> MessageRead<'a> for TestUnknownFields {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.a = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestUnknownFields {
    fn get_size(&self) -> usize {
        0
        + if self.a == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.a) as u64) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.a != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.a))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestSelfReference {
    pub r1: Option<Box<basic::TestSelfReference>>,
    pub r2: Option<Box<basic::TestSelfReference>>,
}

impl<'a> MessageRead<'a> for TestSelfReference {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.r1 = Some(Box::new(r.read_message::<basic::TestSelfReference>(bytes)?)),
                Ok(18) => msg.r2 = Some(Box::new(r.read_message::<basic::TestSelfReference>(bytes)?)),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestSelfReference {
    fn get_size(&self) -> usize {
        0
        + self.r1.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.r2.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.r1 { w.write_with_tag(10, |w| w.write_message(&**s))?; }
        if let Some(ref s) = self.r2 { w.write_with_tag(18, |w| w.write_message(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestDefaultInstanceField<'a> {
    pub s: &'a str,
}

impl<'a> MessageRead<'a> for TestDefaultInstanceField<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.s = r.read_string(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestDefaultInstanceField<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.s == "" { 0 } else { 1 + sizeof_len((&self.s).len()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.s != "" { w.write_with_tag(10, |w| w.write_string(&**&self.s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestDefaultInstance<'a> {
    pub field: Option<basic::TestDefaultInstanceField<'a>>,
}

impl<'a> MessageRead<'a> for TestDefaultInstance<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.field = Some(r.read_message::<basic::TestDefaultInstanceField>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestDefaultInstance<'a> {
    fn get_size(&self) -> usize {
        0
        + self.field.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.field { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestDescriptor {
    pub stuff: i32,
}

impl<'a> MessageRead<'a> for TestDescriptor {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.stuff = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestDescriptor {
    fn get_size(&self) -> usize {
        0
        + if self.stuff == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.stuff) as u64) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.stuff != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.stuff))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestTypesSingular<'a> {
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
    pub string_field: &'a str,
    pub bytes_field: Cow<'a, [u8]>,
    pub enum_field: basic::TestEnumDescriptor,
}

impl<'a> MessageRead<'a> for TestTypesSingular<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
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
                Ok(114) => msg.string_field = r.read_string(bytes)?,
                Ok(122) => msg.bytes_field = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(128) => msg.enum_field = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestTypesSingular<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.double_field == 0f64 { 0 } else { 1 + 8 }
        + if self.float_field == 0f32 { 0 } else { 1 + 4 }
        + if self.int32_field == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.int32_field) as u64) }
        + if self.int64_field == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.int64_field) as u64) }
        + if self.uint32_field == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.uint32_field) as u64) }
        + if self.uint64_field == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.uint64_field) as u64) }
        + if self.sint32_field == 0i32 { 0 } else { 1 + sizeof_sint32(*(&self.sint32_field)) }
        + if self.sint64_field == 0i64 { 0 } else { 1 + sizeof_sint64(*(&self.sint64_field)) }
        + if self.fixed32_field == 0u32 { 0 } else { 1 + 4 }
        + if self.fixed64_field == 0u64 { 0 } else { 1 + 8 }
        + if self.sfixed32_field == 0i32 { 0 } else { 1 + 4 }
        + if self.sfixed64_field == 0i64 { 0 } else { 1 + 8 }
        + if self.bool_field == false { 0 } else { 1 + sizeof_varint(*(&self.bool_field) as u64) }
        + if self.string_field == "" { 0 } else { 1 + sizeof_len((&self.string_field).len()) }
        + if self.bytes_field == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.bytes_field).len()) }
        + if self.enum_field == basic::TestEnumDescriptor::UNKNOWN { 0 } else { 2 + sizeof_varint(*(&self.enum_field) as u64) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.double_field != 0f64 { w.write_with_tag(9, |w| w.write_double(*&self.double_field))?; }
        if self.float_field != 0f32 { w.write_with_tag(21, |w| w.write_float(*&self.float_field))?; }
        if self.int32_field != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.int32_field))?; }
        if self.int64_field != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.int64_field))?; }
        if self.uint32_field != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.uint32_field))?; }
        if self.uint64_field != 0u64 { w.write_with_tag(48, |w| w.write_uint64(*&self.uint64_field))?; }
        if self.sint32_field != 0i32 { w.write_with_tag(56, |w| w.write_sint32(*&self.sint32_field))?; }
        if self.sint64_field != 0i64 { w.write_with_tag(64, |w| w.write_sint64(*&self.sint64_field))?; }
        if self.fixed32_field != 0u32 { w.write_with_tag(77, |w| w.write_fixed32(*&self.fixed32_field))?; }
        if self.fixed64_field != 0u64 { w.write_with_tag(81, |w| w.write_fixed64(*&self.fixed64_field))?; }
        if self.sfixed32_field != 0i32 { w.write_with_tag(93, |w| w.write_sfixed32(*&self.sfixed32_field))?; }
        if self.sfixed64_field != 0i64 { w.write_with_tag(97, |w| w.write_sfixed64(*&self.sfixed64_field))?; }
        if self.bool_field != false { w.write_with_tag(104, |w| w.write_bool(*&self.bool_field))?; }
        if self.string_field != "" { w.write_with_tag(114, |w| w.write_string(&**&self.string_field))?; }
        if self.bytes_field != Cow::Borrowed(b"") { w.write_with_tag(122, |w| w.write_bytes(&**&self.bytes_field))?; }
        if self.enum_field != basic::TestEnumDescriptor::UNKNOWN { w.write_with_tag(128, |w| w.write_enum(*&self.enum_field as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestTypesRepeated<'a> {
    pub double_field: Vec<f64>,
    pub float_field: Vec<f32>,
    pub int32_field: Vec<i32>,
    pub int64_field: Vec<i64>,
    pub uint32_field: Vec<u32>,
    pub uint64_field: Vec<u64>,
    pub sint32_field: Vec<i32>,
    pub sint64_field: Vec<i64>,
    pub fixed32_field: Vec<u32>,
    pub fixed64_field: Vec<u64>,
    pub sfixed32_field: Vec<i32>,
    pub sfixed64_field: Vec<i64>,
    pub bool_field: Vec<bool>,
    pub string_field: Vec<&'a str>,
    pub bytes_field: Vec<Cow<'a, [u8]>>,
    pub enum_field: Vec<basic::TestEnumDescriptor>,
}

impl<'a> MessageRead<'a> for TestTypesRepeated<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.double_field.push(r.read_double(bytes)?),
                Ok(21) => msg.float_field.push(r.read_float(bytes)?),
                Ok(24) => msg.int32_field.push(r.read_int32(bytes)?),
                Ok(32) => msg.int64_field.push(r.read_int64(bytes)?),
                Ok(40) => msg.uint32_field.push(r.read_uint32(bytes)?),
                Ok(48) => msg.uint64_field.push(r.read_uint64(bytes)?),
                Ok(56) => msg.sint32_field.push(r.read_sint32(bytes)?),
                Ok(64) => msg.sint64_field.push(r.read_sint64(bytes)?),
                Ok(77) => msg.fixed32_field.push(r.read_fixed32(bytes)?),
                Ok(81) => msg.fixed64_field.push(r.read_fixed64(bytes)?),
                Ok(93) => msg.sfixed32_field.push(r.read_sfixed32(bytes)?),
                Ok(97) => msg.sfixed64_field.push(r.read_sfixed64(bytes)?),
                Ok(104) => msg.bool_field.push(r.read_bool(bytes)?),
                Ok(114) => msg.string_field.push(r.read_string(bytes)?),
                Ok(122) => msg.bytes_field.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(128) => msg.enum_field.push(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestTypesRepeated<'a> {
    fn get_size(&self) -> usize {
        0
        + (1 + 8) * self.double_field.len()
        + (1 + 4) * self.float_field.len()
        + self.int32_field.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.int64_field.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.uint32_field.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.uint64_field.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.sint32_field.iter().map(|s| 1 + sizeof_sint32(*(s))).sum::<usize>()
        + self.sint64_field.iter().map(|s| 1 + sizeof_sint64(*(s))).sum::<usize>()
        + (1 + 4) * self.fixed32_field.len()
        + (1 + 8) * self.fixed64_field.len()
        + (1 + 4) * self.sfixed32_field.len()
        + (1 + 8) * self.sfixed64_field.len()
        + self.bool_field.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.string_field.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.bytes_field.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.enum_field.iter().map(|s| 2 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.double_field { w.write_with_tag(9, |w| w.write_double(*s))?; }
        for s in &self.float_field { w.write_with_tag(21, |w| w.write_float(*s))?; }
        for s in &self.int32_field { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        for s in &self.int64_field { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        for s in &self.uint32_field { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        for s in &self.uint64_field { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        for s in &self.sint32_field { w.write_with_tag(56, |w| w.write_sint32(*s))?; }
        for s in &self.sint64_field { w.write_with_tag(64, |w| w.write_sint64(*s))?; }
        for s in &self.fixed32_field { w.write_with_tag(77, |w| w.write_fixed32(*s))?; }
        for s in &self.fixed64_field { w.write_with_tag(81, |w| w.write_fixed64(*s))?; }
        for s in &self.sfixed32_field { w.write_with_tag(93, |w| w.write_sfixed32(*s))?; }
        for s in &self.sfixed64_field { w.write_with_tag(97, |w| w.write_sfixed64(*s))?; }
        for s in &self.bool_field { w.write_with_tag(104, |w| w.write_bool(*s))?; }
        for s in &self.string_field { w.write_with_tag(114, |w| w.write_string(&**s))?; }
        for s in &self.bytes_field { w.write_with_tag(122, |w| w.write_bytes(&**s))?; }
        for s in &self.enum_field { w.write_with_tag(128, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestTypesRepeatedPacked<'a> {
    pub double_field: Cow<'a, [f64]>,
    pub float_field: Cow<'a, [f32]>,
    pub int32_field: Vec<i32>,
    pub int64_field: Vec<i64>,
    pub uint32_field: Vec<u32>,
    pub uint64_field: Vec<u64>,
    pub sint32_field: Vec<i32>,
    pub sint64_field: Vec<i64>,
    pub fixed32_field: Cow<'a, [u32]>,
    pub fixed64_field: Cow<'a, [u64]>,
    pub sfixed32_field: Cow<'a, [i32]>,
    pub sfixed64_field: Cow<'a, [i64]>,
    pub bool_field: Vec<bool>,
    pub string_field: Vec<&'a str>,
    pub bytes_field: Vec<Cow<'a, [u8]>>,
    pub enum_field: Vec<basic::TestEnumDescriptor>,
}

impl<'a> MessageRead<'a> for TestTypesRepeatedPacked<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.double_field = r.read_packed_fixed(bytes)?.into(),
                Ok(18) => msg.float_field = r.read_packed_fixed(bytes)?.into(),
                Ok(26) => msg.int32_field = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(34) => msg.int64_field = r.read_packed(bytes, |r, bytes| Ok(r.read_int64(bytes)?))?,
                Ok(42) => msg.uint32_field = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(50) => msg.uint64_field = r.read_packed(bytes, |r, bytes| Ok(r.read_uint64(bytes)?))?,
                Ok(58) => msg.sint32_field = r.read_packed(bytes, |r, bytes| Ok(r.read_sint32(bytes)?))?,
                Ok(66) => msg.sint64_field = r.read_packed(bytes, |r, bytes| Ok(r.read_sint64(bytes)?))?,
                Ok(74) => msg.fixed32_field = r.read_packed_fixed(bytes)?.into(),
                Ok(82) => msg.fixed64_field = r.read_packed_fixed(bytes)?.into(),
                Ok(90) => msg.sfixed32_field = r.read_packed_fixed(bytes)?.into(),
                Ok(98) => msg.sfixed64_field = r.read_packed_fixed(bytes)?.into(),
                Ok(106) => msg.bool_field = r.read_packed(bytes, |r, bytes| Ok(r.read_bool(bytes)?))?,
                Ok(114) => msg.string_field.push(r.read_string(bytes)?),
                Ok(122) => msg.bytes_field.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(130) => msg.enum_field = r.read_packed(bytes, |r, bytes| Ok(r.read_enum(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestTypesRepeatedPacked<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.double_field.is_empty() { 0 } else { 1 + sizeof_len(self.double_field.len() * 8) }
        + if self.float_field.is_empty() { 0 } else { 1 + sizeof_len(self.float_field.len() * 4) }
        + if self.int32_field.is_empty() { 0 } else { 1 + sizeof_len(self.int32_field.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.int64_field.is_empty() { 0 } else { 1 + sizeof_len(self.int64_field.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.uint32_field.is_empty() { 0 } else { 1 + sizeof_len(self.uint32_field.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.uint64_field.is_empty() { 0 } else { 1 + sizeof_len(self.uint64_field.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.sint32_field.is_empty() { 0 } else { 1 + sizeof_len(self.sint32_field.iter().map(|s| sizeof_sint32(*(s))).sum::<usize>()) }
        + if self.sint64_field.is_empty() { 0 } else { 1 + sizeof_len(self.sint64_field.iter().map(|s| sizeof_sint64(*(s))).sum::<usize>()) }
        + if self.fixed32_field.is_empty() { 0 } else { 1 + sizeof_len(self.fixed32_field.len() * 4) }
        + if self.fixed64_field.is_empty() { 0 } else { 1 + sizeof_len(self.fixed64_field.len() * 8) }
        + if self.sfixed32_field.is_empty() { 0 } else { 1 + sizeof_len(self.sfixed32_field.len() * 4) }
        + if self.sfixed64_field.is_empty() { 0 } else { 1 + sizeof_len(self.sfixed64_field.len() * 8) }
        + if self.bool_field.is_empty() { 0 } else { 1 + sizeof_len(self.bool_field.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + self.string_field.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.bytes_field.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.enum_field.is_empty() { 0 } else { 2 + sizeof_len(self.enum_field.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_fixed_with_tag(10, &self.double_field)?;
        w.write_packed_fixed_with_tag(18, &self.float_field)?;
        w.write_packed_with_tag(26, &self.int32_field, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(34, &self.int64_field, |w, m| w.write_int64(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(42, &self.uint32_field, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(50, &self.uint64_field, |w, m| w.write_uint64(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(58, &self.sint32_field, |w, m| w.write_sint32(*m), &|m| sizeof_sint32(*(m)))?;
        w.write_packed_with_tag(66, &self.sint64_field, |w, m| w.write_sint64(*m), &|m| sizeof_sint64(*(m)))?;
        w.write_packed_fixed_with_tag(74, &self.fixed32_field)?;
        w.write_packed_fixed_with_tag(82, &self.fixed64_field)?;
        w.write_packed_fixed_with_tag(90, &self.sfixed32_field)?;
        w.write_packed_fixed_with_tag(98, &self.sfixed64_field)?;
        w.write_packed_with_tag(106, &self.bool_field, |w, m| w.write_bool(*m), &|m| sizeof_varint(*(m) as u64))?;
        for s in &self.string_field { w.write_with_tag(114, |w| w.write_string(&**s))?; }
        for s in &self.bytes_field { w.write_with_tag(122, |w| w.write_bytes(&**s))?; }
        w.write_packed_with_tag(130, &self.enum_field, |w, m| w.write_enum(*m as i32), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestInvalidTag { }

impl<'a> MessageRead<'a> for TestInvalidTag {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for TestInvalidTag { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestTruncated<'a> {
    pub ints: Cow<'a, [u32]>,
}

impl<'a> MessageRead<'a> for TestTruncated<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.ints = r.read_packed_fixed(bytes)?.into(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestTruncated<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.ints.is_empty() { 0 } else { 1 + sizeof_len(self.ints.len() * 4) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_fixed_with_tag(18, &self.ints)?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestBugSint {
    pub s32: i32,
    pub s64: i64,
}

impl<'a> MessageRead<'a> for TestBugSint {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.s32 = r.read_sint32(bytes)?,
                Ok(16) => msg.s64 = r.read_sint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestBugSint {
    fn get_size(&self) -> usize {
        0
        + if self.s32 == 0i32 { 0 } else { 1 + sizeof_sint32(*(&self.s32)) }
        + if self.s64 == 0i64 { 0 } else { 1 + sizeof_sint64(*(&self.s64)) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.s32 != 0i32 { w.write_with_tag(8, |w| w.write_sint32(*&self.s32))?; }
        if self.s64 != 0i64 { w.write_with_tag(16, |w| w.write_sint64(*&self.s64))?; }
        Ok(())
    }
}

