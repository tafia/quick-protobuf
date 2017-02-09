//! Automatically generated rust module for 'perftest_data_quick.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::io::{Write};
use std::borrow::Cow;
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test1 {
    pub value: Option<i32>,
}

impl Test1 {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Test1 {
    fn get_size(&self) -> usize {
        self.value.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.value { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestRepeatedBool {
    pub values: Vec<bool>,
}

impl TestRepeatedBool {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.values.push(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestRepeatedBool {
    fn get_size(&self) -> usize {
        self.values.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.values { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestRepeatedPackedInt32 {
    pub values: Vec<i32>,
}

impl TestRepeatedPackedInt32 {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.values = r.read_packed(bytes, |r, bytes| r.read_int32(bytes))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestRepeatedPackedInt32 {
    fn get_size(&self) -> usize {
        if self.values.is_empty() { 0 } else { 1 + sizeof_len(self.values.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.values, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestRepeatedMessages {
    pub messages1: Vec<TestRepeatedMessages>,
    pub messages2: Vec<TestRepeatedMessages>,
    pub messages3: Vec<TestRepeatedMessages>,
}

impl TestRepeatedMessages {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.messages1.push(r.read_message(bytes, TestRepeatedMessages::from_reader)?),
                Ok(18) => msg.messages2.push(r.read_message(bytes, TestRepeatedMessages::from_reader)?),
                Ok(26) => msg.messages3.push(r.read_message(bytes, TestRepeatedMessages::from_reader)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestRepeatedMessages {
    fn get_size(&self) -> usize {
        self.messages1.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.messages2.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.messages3.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.messages1 { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.messages2 { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.messages3 { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestOptionalMessages {
    pub message1: Option<Box<TestOptionalMessages>>,
    pub message2: Option<Box<TestOptionalMessages>>,
    pub message3: Option<Box<TestOptionalMessages>>,
}

impl TestOptionalMessages {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.message1 = Some(Box::new(r.read_message(bytes, TestOptionalMessages::from_reader)?)),
                Ok(18) => msg.message2 = Some(Box::new(r.read_message(bytes, TestOptionalMessages::from_reader)?)),
                Ok(26) => msg.message3 = Some(Box::new(r.read_message(bytes, TestOptionalMessages::from_reader)?)),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestOptionalMessages {
    fn get_size(&self) -> usize {
        self.message1.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.message2.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.message3.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.message1 { w.write_with_tag(10, |w| w.write_message(&**s))?; }
        if let Some(ref s) = self.message2 { w.write_with_tag(18, |w| w.write_message(&**s))?; }
        if let Some(ref s) = self.message3 { w.write_with_tag(26, |w| w.write_message(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestStrings<'a> {
    pub s1: Option<Cow<'a, str>>,
    pub s2: Option<Cow<'a, str>>,
    pub s3: Option<Cow<'a, str>>,
}

impl<'a> TestStrings<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.s1 = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.s2 = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.s3 = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestStrings<'a> {
    fn get_size(&self) -> usize {
        self.s1.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.s2.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.s3.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.s1 { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.s2 { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.s3 { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestBytes<'a> {
    pub b1: Option<Cow<'a, [u8]>>,
}

impl<'a> TestBytes<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.b1 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestBytes<'a> {
    fn get_size(&self) -> usize {
        self.b1.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.b1 { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PerftestData<'a> {
    pub test1: Vec<Test1>,
    pub test_repeated_bool: Vec<TestRepeatedBool>,
    pub test_repeated_messages: Vec<TestRepeatedMessages>,
    pub test_optional_messages: Vec<TestOptionalMessages>,
    pub test_strings: Vec<TestStrings<'a>>,
    pub test_repeated_packed_int32: Vec<TestRepeatedPackedInt32>,
    pub test_small_bytearrays: Vec<TestBytes<'a>>,
    pub test_large_bytearrays: Vec<TestBytes<'a>>,
}

impl<'a> PerftestData<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.test1.push(r.read_message(bytes, Test1::from_reader)?),
                Ok(18) => msg.test_repeated_bool.push(r.read_message(bytes, TestRepeatedBool::from_reader)?),
                Ok(26) => msg.test_repeated_messages.push(r.read_message(bytes, TestRepeatedMessages::from_reader)?),
                Ok(34) => msg.test_optional_messages.push(r.read_message(bytes, TestOptionalMessages::from_reader)?),
                Ok(42) => msg.test_strings.push(r.read_message(bytes, TestStrings::from_reader)?),
                Ok(50) => msg.test_repeated_packed_int32.push(r.read_message(bytes, TestRepeatedPackedInt32::from_reader)?),
                Ok(58) => msg.test_small_bytearrays.push(r.read_message(bytes, TestBytes::from_reader)?),
                Ok(66) => msg.test_large_bytearrays.push(r.read_message(bytes, TestBytes::from_reader)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PerftestData<'a> {
    fn get_size(&self) -> usize {
        self.test1.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.test_repeated_bool.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.test_repeated_messages.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.test_optional_messages.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.test_strings.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.test_repeated_packed_int32.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.test_small_bytearrays.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.test_large_bytearrays.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.test1 { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.test_repeated_bool { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.test_repeated_messages { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.test_optional_messages { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.test_strings { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.test_repeated_packed_int32 { w.write_with_tag(50, |w| w.write_message(s))?; }
        for s in &self.test_small_bytearrays { w.write_with_tag(58, |w| w.write_message(s))?; }
        for s in &self.test_large_bytearrays { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}
