//! Automatically generated rust module for 'test_ident_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::io::{Write};
use std::borrow::Cow;
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Vec {
}

impl Vec {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Vec {
    fn get_size(&self) -> usize { 0 }

    fn write_message<W: Write>(&self, _: &mut Writer<W>) -> Result<()> { Ok(()) }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct String {
}

impl String {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for String {
    fn get_size(&self) -> usize { 0 }

    fn write_message<W: Write>(&self, _: &mut Writer<W>) -> Result<()> { Ok(()) }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Option {
}

impl Option {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Option {
    fn get_size(&self) -> usize { 0 }

    fn write_message<W: Write>(&self, _: &mut Writer<W>) -> Result<()> { Ok(()) }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct None {
}

impl None {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for None {
    fn get_size(&self) -> usize { 0 }

    fn write_message<W: Write>(&self, _: &mut Writer<W>) -> Result<()> { Ok(()) }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Some {
}

impl Some {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Some {
    fn get_size(&self) -> usize { 0 }

    fn write_message<W: Write>(&self, _: &mut Writer<W>) -> Result<()> { Ok(()) }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Message {
}

impl Message {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Message {
    fn get_size(&self) -> usize { 0 }

    fn write_message<W: Write>(&self, _: &mut Writer<W>) -> Result<()> { Ok(()) }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestType<'a> {
    pub struct: Vec<Cow<'a, str>>,
    pub ref: Vec<u32>,
    pub type: mod_TestType::OneOftype<'a>,
}

impl<'a> TestType<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.struct = r.read_packed(bytes, |r, bytes| r.read_string(bytes).map(Cow::Borrowed))?,
                Ok(26) => msg.ref = r.read_packed(bytes, |r, bytes| r.read_uint32(bytes))?,
                Ok(10) => msg.type = mod_TestType::OneOftype::s(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestType<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.struct.is_empty() { 0 } else { 1 + sizeof_len(self.struct.iter().map(|s| sizeof_len((s).len())).sum::<usize>()) }
        + if self.ref.is_empty() { 0 } else { 1 + sizeof_len(self.ref.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + match self.type {            mod_TestType::OneOftype::s(ref m) => 1 + sizeof_len((m).len()),
            mod_TestType::OneOftype::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(18, &self.struct, |w, m| w.write_string(&**m), &|m| sizeof_len((m).len()))?;
        w.write_packed_with_tag(26, &self.ref, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        match self.type {            mod_TestType::OneOftype::s(ref m) => { w.write_with_tag(10, |w| w.write_string(&**m))? },
            mod_TestType::OneOftype::None => {},
    }        Ok(())
    }
}

pub mod mod_TestType {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOftype<'a> {
    s(Cow<'a, str>),
    None,
}

impl<'a> Default for OneOftype<'a> {
    fn default() -> Self {
        OneOftype::None
    }
}

}
