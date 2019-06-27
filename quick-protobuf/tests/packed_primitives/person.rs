// Automatically generated rust module for 'person.proto' file

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
pub enum City {
    LONDON = 0,
    PARIS = 1,
}

impl Default for City {
    fn default() -> Self {
        City::LONDON
    }
}

impl From<i32> for City {
    fn from(i: i32) -> Self {
        match i {
            0 => City::LONDON,
            1 => City::PARIS,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for City {
    fn from(s: &'a str) -> Self {
        match s {
            "LONDON" => City::LONDON,
            "PARIS" => City::PARIS,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Address {
    pub city: Option<City>,
}

impl<'a> MessageRead<'a> for Address {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.city = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Address {
    fn get_size(&self) -> usize {
        0
        + self.city.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.city { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Person<'a> {
    pub address: Option<Address>,
    pub names: Vec<&'a str>,
}

impl<'a> MessageRead<'a> for Person<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.address = Some(r.read_message::<Address>(bytes)?),
                Ok(26) => msg.names.push(r.read_string(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Person<'a> {
    fn get_size(&self) -> usize {
        0
        + self.address.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.names.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.address { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.names { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PersonPacked<'a> {
    pub address: Option<Address>,
    pub names: Vec<&'a str>,
}

impl<'a> MessageRead<'a> for PersonPacked<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.address = Some(r.read_message::<Address>(bytes)?),
                Ok(26) => msg.names.push(r.read_string(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PersonPacked<'a> {
    fn get_size(&self) -> usize {
        0
        + self.address.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.names.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.address { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.names { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

