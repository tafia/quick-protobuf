// Automatically generated rust module for 'vector_tile.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Tile<'a> {
    pub layers: Vec<vector_tile::mod_Tile::Layer<'a>>,
}

impl<'a> MessageRead<'a> for Tile<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(26) => msg.layers.push(r.read_message::<vector_tile::mod_Tile::Layer>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Tile<'a> {
    fn get_size(&self) -> usize {
        0
        + self.layers.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.layers { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Tile {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Value<'a> {
    pub string_value: Option<&'a str>,
    pub float_value: Option<f32>,
    pub double_value: Option<f64>,
    pub int_value: Option<i64>,
    pub uint_value: Option<u64>,
    pub sint_value: Option<i64>,
    pub bool_value: Option<bool>,
}

impl<'a> MessageRead<'a> for Value<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.string_value = Some(r.read_string(bytes)?),
                Ok(21) => msg.float_value = Some(r.read_float(bytes)?),
                Ok(25) => msg.double_value = Some(r.read_double(bytes)?),
                Ok(32) => msg.int_value = Some(r.read_int64(bytes)?),
                Ok(40) => msg.uint_value = Some(r.read_uint64(bytes)?),
                Ok(48) => msg.sint_value = Some(r.read_sint64(bytes)?),
                Ok(56) => msg.bool_value = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Value<'a> {
    fn get_size(&self) -> usize {
        0
        + self.string_value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.float_value.as_ref().map_or(0, |_| 1 + 4)
        + self.double_value.as_ref().map_or(0, |_| 1 + 8)
        + self.int_value.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uint_value.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sint_value.as_ref().map_or(0, |m| 1 + sizeof_sint64(*(m)))
        + self.bool_value.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.string_value { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.float_value { w.write_with_tag(21, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.double_value { w.write_with_tag(25, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.int_value { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.uint_value { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.sint_value { w.write_with_tag(48, |w| w.write_sint64(*s))?; }
        if let Some(ref s) = self.bool_value { w.write_with_tag(56, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Feature {
    pub id: u64,
    pub tags: Vec<u32>,
    pub type_pb: vector_tile::mod_Tile::GeomType,
    pub geometry: Vec<u32>,
}

impl<'a> MessageRead<'a> for Feature {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint64(bytes)?,
                Ok(18) => msg.tags = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(24) => msg.type_pb = r.read_enum(bytes)?,
                Ok(34) => msg.geometry = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Feature {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.tags.is_empty() { 0 } else { 1 + sizeof_len(self.tags.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.type_pb == vector_tile::mod_Tile::GeomType::UNKNOWN { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.geometry.is_empty() { 0 } else { 1 + sizeof_len(self.geometry.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.id))?; }
        w.write_packed_with_tag(18, &self.tags, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.type_pb != vector_tile::mod_Tile::GeomType::UNKNOWN { w.write_with_tag(24, |w| w.write_enum(*&self.type_pb as i32))?; }
        w.write_packed_with_tag(34, &self.geometry, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Layer<'a> {
    pub version: u32,
    pub name: &'a str,
    pub features: Vec<vector_tile::mod_Tile::Feature>,
    pub keys: Vec<&'a str>,
    pub values: Vec<vector_tile::mod_Tile::Value<'a>>,
    pub extent: u32,
}

impl<'a> MessageRead<'a> for Layer<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Layer {
            version: 1u32,
            extent: 4096u32,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(120) => msg.version = r.read_uint32(bytes)?,
                Ok(10) => msg.name = r.read_string(bytes)?,
                Ok(18) => msg.features.push(r.read_message::<vector_tile::mod_Tile::Feature>(bytes)?),
                Ok(26) => msg.keys.push(r.read_string(bytes)?),
                Ok(34) => msg.values.push(r.read_message::<vector_tile::mod_Tile::Value>(bytes)?),
                Ok(40) => msg.extent = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Layer<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.version) as u64)
        + 1 + sizeof_len((&self.name).len())
        + self.features.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.keys.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.values.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.extent == 4096u32 { 0 } else { 1 + sizeof_varint(*(&self.extent) as u64) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(120, |w| w.write_uint32(*&self.version))?;
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        for s in &self.features { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.keys { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        for s in &self.values { w.write_with_tag(34, |w| w.write_message(s))?; }
        if self.extent != 4096u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.extent))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GeomType {
    UNKNOWN = 0,
    POINT = 1,
    LINESTRING = 2,
    POLYGON = 3,
}

impl Default for GeomType {
    fn default() -> Self {
        GeomType::UNKNOWN
    }
}

impl From<i32> for GeomType {
    fn from(i: i32) -> Self {
        match i {
            0 => GeomType::UNKNOWN,
            1 => GeomType::POINT,
            2 => GeomType::LINESTRING,
            3 => GeomType::POLYGON,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for GeomType {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => GeomType::UNKNOWN,
            "POINT" => GeomType::POINT,
            "LINESTRING" => GeomType::LINESTRING,
            "POLYGON" => GeomType::POLYGON,
            _ => Self::default(),
        }
    }
}

}

