//! Automatically generated rust module for 'test_map_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use std::borrow::Cow;
use std::collections::HashMap;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestMap<'a> {
    pub m: HashMap<Cow<'a, str>, u32>,
    pub mm: HashMap<Cow<'a, str>, TestMapEntry>,
}

impl<'a> MessageRead<'a> for TestMap<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| r.read_string(bytes).map(Cow::Borrowed), |r, bytes| r.read_uint32(bytes))?;
                    msg.m.insert(key, value);
                }
                Ok(18) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| r.read_string(bytes).map(Cow::Borrowed), |r, bytes| r.read_message::<TestMapEntry>(bytes))?;
                    msg.mm.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestMap<'a> {
    fn get_size(&self) -> usize {
        0
        + self.m.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_varint(*(v) as u64))).sum::<usize>()
        + self.mm.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).get_size()))).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for (k, v) in self.m.iter() { w.write_with_tag(10, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_varint(*(v) as u64), 10, |w| w.write_string(&**k), 16, |w| w.write_uint32(*v)))?; }
        for (k, v) in self.mm.iter() { w.write_with_tag(18, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).get_size()), 10, |w| w.write_string(&**k), 18, |w| w.write_message(v)))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestMapEntry {
    pub v: Option<i64>,
}

impl<'a> MessageRead<'a> for TestMapEntry {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.v = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestMapEntry {
    fn get_size(&self) -> usize {
        0
        + self.v.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.v { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

