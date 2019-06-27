// Automatically generated rust module for 'test_ident_pb.proto' file

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

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Vec_pb { }

impl<'a> MessageRead<'a> for Vec_pb {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Vec_pb { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct String_pb { }

impl<'a> MessageRead<'a> for String_pb {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for String_pb { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Option_pb { }

impl<'a> MessageRead<'a> for Option_pb {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Option_pb { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct None_pb { }

impl<'a> MessageRead<'a> for None_pb {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for None_pb { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Some_pb { }

impl<'a> MessageRead<'a> for Some_pb {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Some_pb { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Message { }

impl<'a> MessageRead<'a> for Message {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Message { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestType<'a> {
    pub struct_pb: Vec<&'a str>,
    pub ref_pb: Vec<u32>,
    pub type_pb: mod_TestType::OneOftype_pb<'a>,
}

impl<'a> MessageRead<'a> for TestType<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.struct_pb.push(r.read_string(bytes)?),
                Ok(26) => msg.ref_pb = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(10) => msg.type_pb = mod_TestType::OneOftype_pb::s(r.read_string(bytes)?),
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
        + self.struct_pb.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.ref_pb.is_empty() { 0 } else { 1 + sizeof_len(self.ref_pb.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + match self.type_pb {
            mod_TestType::OneOftype_pb::s(ref m) => 1 + sizeof_len((m).len()),
            mod_TestType::OneOftype_pb::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.struct_pb { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        w.write_packed_with_tag(26, &self.ref_pb, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        match self.type_pb {            mod_TestType::OneOftype_pb::s(ref m) => { w.write_with_tag(10, |w| w.write_string(&**m))? },
            mod_TestType::OneOftype_pb::None => {},
    }        Ok(())
    }
}

pub mod mod_TestType {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOftype_pb<'a> {
    s(&'a str),
    None,
}

impl<'a> Default for OneOftype_pb<'a> {
    fn default() -> Self {
        OneOftype_pb::None
    }
}

}

