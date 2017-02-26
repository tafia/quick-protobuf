//! Automatically generated rust module for 'test_basic_pb.proto' file

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

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test1 {
    pub a: Option<i32>,
}

impl Test1 {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.a = Some(r.read_int32(bytes)?),
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
        + self.a.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.a { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test2<'a> {
    pub b: Option<Cow<'a, str>>,
}

impl<'a> Test2<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.b = Some(r.read_string(bytes).map(Cow::Borrowed)?),
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
        + self.b.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.b { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test3 {
    pub c: Option<Test1>,
}

impl Test3 {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(26) => msg.c = Some(r.read_message(bytes, Test1::from_reader)?),
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

impl Test4 {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(34) => msg.d = r.read_packed(bytes, |r, bytes| r.read_int32(bytes))?,
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

impl TestPackedUnpacked {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(32) => msg.unpacked.push(r.read_int32(bytes)?),
                Ok(42) => msg.packed = r.read_packed(bytes, |r, bytes| r.read_int32(bytes))?,
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
    pub foo: Option<i32>,
}

impl TestEmpty {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.foo = Some(r.read_int32(bytes)?),
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
        + self.foo.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.foo { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test {
    pub b: Option<bool>,
}

impl Test {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(40) => msg.b = Some(r.read_bool(bytes)?),
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
        + self.b.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.b { w.write_with_tag(40, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestUnknownFields {
    pub a: Option<i32>,
}

impl TestUnknownFields {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.a = Some(r.read_int32(bytes)?),
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
        + self.a.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.a { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestSelfReference {
    pub r1: Option<Box<TestSelfReference>>,
    pub r2: Option<Box<TestSelfReference>>,
}

impl TestSelfReference {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.r1 = Some(Box::new(r.read_message(bytes, TestSelfReference::from_reader)?)),
                Ok(18) => msg.r2 = Some(Box::new(r.read_message(bytes, TestSelfReference::from_reader)?)),
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
    pub s: Option<Cow<'a, str>>,
}

impl<'a> TestDefaultInstanceField<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.s = Some(r.read_string(bytes).map(Cow::Borrowed)?),
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
        + self.s.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.s { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestDefaultInstance<'a> {
    pub field: Option<TestDefaultInstanceField<'a>>,
}

impl<'a> TestDefaultInstance<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.field = Some(r.read_message(bytes, TestDefaultInstanceField::from_reader)?),
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
    pub stuff: Option<i32>,
}

impl TestDescriptor {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.stuff = Some(r.read_int32(bytes)?),
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
        + self.stuff.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.stuff { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestTypesSingular<'a> {
    pub double_field: Option<f64>,
    pub float_field: Option<f32>,
    pub int32_field: Option<i32>,
    pub int64_field: Option<i64>,
    pub uint32_field: Option<u32>,
    pub uint64_field: Option<u64>,
    pub sint32_field: Option<i32>,
    pub sint64_field: Option<i64>,
    pub fixed32_field: Option<u32>,
    pub fixed64_field: Option<u64>,
    pub sfixed32_field: Option<i32>,
    pub sfixed64_field: Option<i64>,
    pub bool_field: Option<bool>,
    pub string_field: Option<Cow<'a, str>>,
    pub bytes_field: Option<Cow<'a, [u8]>>,
    pub enum_field: Option<TestEnumDescriptor>,
}

impl<'a> TestTypesSingular<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.double_field = Some(r.read_double(bytes)?),
                Ok(21) => msg.float_field = Some(r.read_float(bytes)?),
                Ok(24) => msg.int32_field = Some(r.read_int32(bytes)?),
                Ok(32) => msg.int64_field = Some(r.read_int64(bytes)?),
                Ok(40) => msg.uint32_field = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.uint64_field = Some(r.read_uint64(bytes)?),
                Ok(56) => msg.sint32_field = Some(r.read_sint32(bytes)?),
                Ok(64) => msg.sint64_field = Some(r.read_sint64(bytes)?),
                Ok(77) => msg.fixed32_field = Some(r.read_fixed32(bytes)?),
                Ok(81) => msg.fixed64_field = Some(r.read_fixed64(bytes)?),
                Ok(93) => msg.sfixed32_field = Some(r.read_sfixed32(bytes)?),
                Ok(97) => msg.sfixed64_field = Some(r.read_sfixed64(bytes)?),
                Ok(104) => msg.bool_field = Some(r.read_bool(bytes)?),
                Ok(114) => msg.string_field = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(122) => msg.bytes_field = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(128) => msg.enum_field = Some(r.read_enum(bytes)?),
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
        + self.double_field.as_ref().map_or(0, |_| 1 + 8)
        + self.float_field.as_ref().map_or(0, |_| 1 + 4)
        + self.int32_field.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.int64_field.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uint32_field.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uint64_field.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sint32_field.as_ref().map_or(0, |m| 1 + sizeof_sint32(*(m)))
        + self.sint64_field.as_ref().map_or(0, |m| 1 + sizeof_sint64(*(m)))
        + self.fixed32_field.as_ref().map_or(0, |_| 1 + 4)
        + self.fixed64_field.as_ref().map_or(0, |_| 1 + 8)
        + self.sfixed32_field.as_ref().map_or(0, |_| 1 + 4)
        + self.sfixed64_field.as_ref().map_or(0, |_| 1 + 8)
        + self.bool_field.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.string_field.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.bytes_field.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.enum_field.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.double_field { w.write_with_tag(9, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.float_field { w.write_with_tag(21, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.int32_field { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.int64_field { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.uint32_field { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.uint64_field { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.sint32_field { w.write_with_tag(56, |w| w.write_sint32(*s))?; }
        if let Some(ref s) = self.sint64_field { w.write_with_tag(64, |w| w.write_sint64(*s))?; }
        if let Some(ref s) = self.fixed32_field { w.write_with_tag(77, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.fixed64_field { w.write_with_tag(81, |w| w.write_fixed64(*s))?; }
        if let Some(ref s) = self.sfixed32_field { w.write_with_tag(93, |w| w.write_sfixed32(*s))?; }
        if let Some(ref s) = self.sfixed64_field { w.write_with_tag(97, |w| w.write_sfixed64(*s))?; }
        if let Some(ref s) = self.bool_field { w.write_with_tag(104, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.string_field { w.write_with_tag(114, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.bytes_field { w.write_with_tag(122, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.enum_field { w.write_with_tag(128, |w| w.write_enum(*s as i32))?; }
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
    pub string_field: Vec<Cow<'a, str>>,
    pub bytes_field: Vec<Cow<'a, [u8]>>,
    pub enum_field: Vec<TestEnumDescriptor>,
}

impl<'a> TestTypesRepeated<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
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
                Ok(114) => msg.string_field = r.read_packed(bytes, |r, bytes| r.read_string(bytes).map(Cow::Borrowed))?,
                Ok(122) => msg.bytes_field = r.read_packed(bytes, |r, bytes| r.read_bytes(bytes).map(Cow::Borrowed))?,
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
        + if self.string_field.is_empty() { 0 } else { 1 + sizeof_len(self.string_field.iter().map(|s| sizeof_len((s).len())).sum::<usize>()) }
        + if self.bytes_field.is_empty() { 0 } else { 1 + sizeof_len(self.bytes_field.iter().map(|s| sizeof_len((s).len())).sum::<usize>()) }
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
        w.write_packed_with_tag(114, &self.string_field, |w, m| w.write_string(&**m), &|m| sizeof_len((m).len()))?;
        w.write_packed_with_tag(122, &self.bytes_field, |w, m| w.write_bytes(&**m), &|m| sizeof_len((m).len()))?;
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
    pub string_field: Vec<Cow<'a, str>>,
    pub bytes_field: Vec<Cow<'a, [u8]>>,
    pub enum_field: Vec<TestEnumDescriptor>,
}

impl<'a> TestTypesRepeatedPacked<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.double_field = Cow::Borrowed(r.read_packed_fixed(bytes)?),
                Ok(18) => msg.float_field = Cow::Borrowed(r.read_packed_fixed(bytes)?),
                Ok(26) => msg.int32_field = r.read_packed(bytes, |r, bytes| r.read_int32(bytes))?,
                Ok(34) => msg.int64_field = r.read_packed(bytes, |r, bytes| r.read_int64(bytes))?,
                Ok(42) => msg.uint32_field = r.read_packed(bytes, |r, bytes| r.read_uint32(bytes))?,
                Ok(50) => msg.uint64_field = r.read_packed(bytes, |r, bytes| r.read_uint64(bytes))?,
                Ok(58) => msg.sint32_field = r.read_packed(bytes, |r, bytes| r.read_sint32(bytes))?,
                Ok(66) => msg.sint64_field = r.read_packed(bytes, |r, bytes| r.read_sint64(bytes))?,
                Ok(74) => msg.fixed32_field = Cow::Borrowed(r.read_packed_fixed(bytes)?),
                Ok(82) => msg.fixed64_field = Cow::Borrowed(r.read_packed_fixed(bytes)?),
                Ok(90) => msg.sfixed32_field = Cow::Borrowed(r.read_packed_fixed(bytes)?),
                Ok(98) => msg.sfixed64_field = Cow::Borrowed(r.read_packed_fixed(bytes)?),
                Ok(106) => msg.bool_field = r.read_packed(bytes, |r, bytes| r.read_bool(bytes))?,
                Ok(114) => msg.string_field = r.read_packed(bytes, |r, bytes| r.read_string(bytes).map(Cow::Borrowed))?,
                Ok(122) => msg.bytes_field = r.read_packed(bytes, |r, bytes| r.read_bytes(bytes).map(Cow::Borrowed))?,
                Ok(130) => msg.enum_field = r.read_packed(bytes, |r, bytes| r.read_enum(bytes))?,
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
        + if self.string_field.is_empty() { 0 } else { 1 + sizeof_len(self.string_field.iter().map(|s| sizeof_len((s).len())).sum::<usize>()) }
        + if self.bytes_field.is_empty() { 0 } else { 1 + sizeof_len(self.bytes_field.iter().map(|s| sizeof_len((s).len())).sum::<usize>()) }
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
        w.write_packed_with_tag(114, &self.string_field, |w, m| w.write_string(&**m), &|m| sizeof_len((m).len()))?;
        w.write_packed_with_tag(122, &self.bytes_field, |w, m| w.write_bytes(&**m), &|m| sizeof_len((m).len()))?;
        w.write_packed_with_tag(130, &self.enum_field, |w, m| w.write_enum(*m as i32), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestInvalidTag { }

impl TestInvalidTag {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for TestInvalidTag { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestTruncated<'a> {
    pub ints: Cow<'a, [u32]>,
}

impl<'a> TestTruncated<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.ints = Cow::Borrowed(r.read_packed_fixed(bytes)?),
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
    pub s32: Option<i32>,
    pub s64: Option<i64>,
}

impl TestBugSint {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.s32 = Some(r.read_sint32(bytes)?),
                Ok(16) => msg.s64 = Some(r.read_sint64(bytes)?),
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
        + self.s32.as_ref().map_or(0, |m| 1 + sizeof_sint32(*(m)))
        + self.s64.as_ref().map_or(0, |m| 1 + sizeof_sint64(*(m)))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.s32 { w.write_with_tag(8, |w| w.write_sint32(*s))?; }
        if let Some(ref s) = self.s64 { w.write_with_tag(16, |w| w.write_sint64(*s))?; }
        Ok(())
    }
}
