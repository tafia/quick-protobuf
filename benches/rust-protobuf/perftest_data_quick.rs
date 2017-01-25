//! Automatically generated rust module for 'perftest_data_quick.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::io::{Read, Write};
use quick_protobuf::{MessageRead, MessageWrite, Reader, Writer, Result};
use quick_protobuf::sizeofs::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test1 {
    pub value: Option<i32>,
}

impl MessageRead for Test1 {
    fn from_reader<R: Read>(mut r: &mut Reader<R>) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag() {
                Ok(8) => msg.value = Some(r.read_int32()?),
                Ok(t) => { r.read_unknown(t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Test1 {
    fn get_size(&self) -> usize {
        self.value.as_ref().map_or(0, |m| 1 + sizeof_int32(*m))
    }

    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.value { r.write_int32_with_tag(8, *s)?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestRepeatedBool {
    pub values: Vec<bool>,
}

impl MessageRead for TestRepeatedBool {
    fn from_reader<R: Read>(mut r: &mut Reader<R>) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag() {
                Ok(8) => msg.values.push(r.read_bool()?),
                Ok(t) => { r.read_unknown(t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestRepeatedBool {
    fn get_size(&self) -> usize {
        self.values.iter().map(|s| 1 + sizeof_bool(*s)).sum::<usize>()
    }

    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {
        for s in &self.values { r.write_bool_with_tag(8, *s)? }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestRepeatedPackedInt32 {
    pub values: Vec<i32>,
}

impl MessageRead for TestRepeatedPackedInt32 {
    fn from_reader<R: Read>(mut r: &mut Reader<R>) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag() {
                Ok(10) => msg.values = r.read_packed_repeated_field(|r| r.read_int32())?,
                Ok(t) => { r.read_unknown(t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestRepeatedPackedInt32 {
    fn get_size(&self) -> usize {
        if self.values.is_empty() { 0 } else { 1 + sizeof_var_length(self.values.iter().map(|s| sizeof_int32(*s)).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {
        r.write_packed_repeated_field_with_tag(10, &self.values, |r, m| r.write_int32(*m), &|m| sizeof_int32(*m))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestRepeatedMessages {
    pub messages1: Vec<TestRepeatedMessages>,
    pub messages2: Vec<TestRepeatedMessages>,
    pub messages3: Vec<TestRepeatedMessages>,
}

impl MessageRead for TestRepeatedMessages {
    fn from_reader<R: Read>(mut r: &mut Reader<R>) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag() {
                Ok(10) => msg.messages1.push(r.read_message()?),
                Ok(18) => msg.messages2.push(r.read_message()?),
                Ok(26) => msg.messages3.push(r.read_message()?),
                Ok(t) => { r.read_unknown(t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestRepeatedMessages {
    fn get_size(&self) -> usize {
        self.messages1.iter().map(|s| 1 + sizeof_var_length(s.get_size())).sum::<usize>()
        + self.messages2.iter().map(|s| 1 + sizeof_var_length(s.get_size())).sum::<usize>()
        + self.messages3.iter().map(|s| 1 + sizeof_var_length(s.get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {
        for s in &self.messages1 { r.write_message_with_tag(10, s)? }
        for s in &self.messages2 { r.write_message_with_tag(18, s)? }
        for s in &self.messages3 { r.write_message_with_tag(26, s)? }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestOptionalMessages {
    pub message1: Option<Box<TestOptionalMessages>>,
    pub message2: Option<Box<TestOptionalMessages>>,
    pub message3: Option<Box<TestOptionalMessages>>,
}

impl MessageRead for TestOptionalMessages {
    fn from_reader<R: Read>(mut r: &mut Reader<R>) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag() {
                Ok(10) => msg.message1 = Some(Box::new(r.read_message()?)),
                Ok(18) => msg.message2 = Some(Box::new(r.read_message()?)),
                Ok(26) => msg.message3 = Some(Box::new(r.read_message()?)),
                Ok(t) => { r.read_unknown(t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestOptionalMessages {
    fn get_size(&self) -> usize {
        self.message1.as_ref().map_or(0, |m| 1 + sizeof_var_length(m.get_size()))
        + self.message2.as_ref().map_or(0, |m| 1 + sizeof_var_length(m.get_size()))
        + self.message3.as_ref().map_or(0, |m| 1 + sizeof_var_length(m.get_size()))
    }

    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.message1 { r.write_message_with_tag(10, &**s)?; }
        if let Some(ref s) = self.message2 { r.write_message_with_tag(18, &**s)?; }
        if let Some(ref s) = self.message3 { r.write_message_with_tag(26, &**s)?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestStrings {
    pub s1: Option<String>,
    pub s2: Option<String>,
    pub s3: Option<String>,
}

impl MessageRead for TestStrings {
    fn from_reader<R: Read>(mut r: &mut Reader<R>) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag() {
                Ok(10) => msg.s1 = Some(r.read_string()?),
                Ok(18) => msg.s2 = Some(r.read_string()?),
                Ok(26) => msg.s3 = Some(r.read_string()?),
                Ok(t) => { r.read_unknown(t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestStrings {
    fn get_size(&self) -> usize {
        self.s1.as_ref().map_or(0, |m| 1 + sizeof_var_length(m.len()))
        + self.s2.as_ref().map_or(0, |m| 1 + sizeof_var_length(m.len()))
        + self.s3.as_ref().map_or(0, |m| 1 + sizeof_var_length(m.len()))
    }

    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.s1 { r.write_string_with_tag(10, s)?; }
        if let Some(ref s) = self.s2 { r.write_string_with_tag(18, s)?; }
        if let Some(ref s) = self.s3 { r.write_string_with_tag(26, s)?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestBytes {
    pub b1: Option<Vec<u8>>,
}

impl MessageRead for TestBytes {
    fn from_reader<R: Read>(mut r: &mut Reader<R>) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag() {
                Ok(10) => msg.b1 = Some(r.read_bytes()?),
                Ok(t) => { r.read_unknown(t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestBytes {
    fn get_size(&self) -> usize {
        self.b1.as_ref().map_or(0, |m| 1 + sizeof_var_length(m.len()))
    }

    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.b1 { r.write_bytes_with_tag(10, s)?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PerftestData {
    pub test1: Vec<Test1>,
    pub test_repeated_bool: Vec<TestRepeatedBool>,
    pub test_repeated_messages: Vec<TestRepeatedMessages>,
    pub test_optional_messages: Vec<TestOptionalMessages>,
    pub test_strings: Vec<TestStrings>,
    pub test_repeated_packed_int32: Vec<TestRepeatedPackedInt32>,
    pub test_small_bytearrays: Vec<TestBytes>,
    pub test_large_bytearrays: Vec<TestBytes>,
}

impl MessageRead for PerftestData {
    fn from_reader<R: Read>(mut r: &mut Reader<R>) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag() {
                Ok(10) => msg.test1.push(r.read_message()?),
                Ok(18) => msg.test_repeated_bool.push(r.read_message()?),
                Ok(26) => msg.test_repeated_messages.push(r.read_message()?),
                Ok(34) => msg.test_optional_messages.push(r.read_message()?),
                Ok(42) => msg.test_strings.push(r.read_message()?),
                Ok(50) => msg.test_repeated_packed_int32.push(r.read_message()?),
                Ok(58) => msg.test_small_bytearrays.push(r.read_message()?),
                Ok(66) => msg.test_large_bytearrays.push(r.read_message()?),
                Ok(t) => { r.read_unknown(t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PerftestData {
    fn get_size(&self) -> usize {
        self.test1.iter().map(|s| 1 + sizeof_var_length(s.get_size())).sum::<usize>()
        + self.test_repeated_bool.iter().map(|s| 1 + sizeof_var_length(s.get_size())).sum::<usize>()
        + self.test_repeated_messages.iter().map(|s| 1 + sizeof_var_length(s.get_size())).sum::<usize>()
        + self.test_optional_messages.iter().map(|s| 1 + sizeof_var_length(s.get_size())).sum::<usize>()
        + self.test_strings.iter().map(|s| 1 + sizeof_var_length(s.get_size())).sum::<usize>()
        + self.test_repeated_packed_int32.iter().map(|s| 1 + sizeof_var_length(s.get_size())).sum::<usize>()
        + self.test_small_bytearrays.iter().map(|s| 1 + sizeof_var_length(s.get_size())).sum::<usize>()
        + self.test_large_bytearrays.iter().map(|s| 1 + sizeof_var_length(s.get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {
        for s in &self.test1 { r.write_message_with_tag(10, s)? }
        for s in &self.test_repeated_bool { r.write_message_with_tag(18, s)? }
        for s in &self.test_repeated_messages { r.write_message_with_tag(26, s)? }
        for s in &self.test_optional_messages { r.write_message_with_tag(34, s)? }
        for s in &self.test_strings { r.write_message_with_tag(42, s)? }
        for s in &self.test_repeated_packed_int32 { r.write_message_with_tag(50, s)? }
        for s in &self.test_small_bytearrays { r.write_message_with_tag(58, s)? }
        for s in &self.test_large_bytearrays { r.write_message_with_tag(66, s)? }
        Ok(())
    }
}
