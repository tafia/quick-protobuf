use super::*;
use quick_protobuf::sizeofs::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result, Writer};
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test1 {
    pub value: Option<i32>,
}

impl<'a> MessageRead<'a> for Test1 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = Some(r.read_int32(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Test1 {
    fn get_size(&self) -> usize {
        0 + self
            .value
            .as_ref()
            .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.value {
            w.write_with_tag(8, |w| w.write_int32(*s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestRepeatedBool {
    pub values: Vec<bool>,
}

impl<'a> MessageRead<'a> for TestRepeatedBool {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.values.push(r.read_bool(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestRepeatedBool {
    fn get_size(&self) -> usize {
        0 + self
            .values
            .iter()
            .map(|s| 1 + sizeof_varint(*(s) as u64))
            .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.values {
            w.write_with_tag(8, |w| w.write_bool(*s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestRepeatedPackedInt32 {
    pub values: Vec<i32>,
}

impl<'a> MessageRead<'a> for TestRepeatedPackedInt32 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.values = r.read_packed(bytes, |r, bytes| r.read_int32(bytes))?,
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestRepeatedPackedInt32 {
    fn get_size(&self) -> usize {
        0 + if self.values.is_empty() {
            0
        } else {
            1 + sizeof_len(
                self.values
                    .iter()
                    .map(|s| sizeof_varint(*(s) as u64))
                    .sum::<usize>(),
            )
        }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.values, |w, m| w.write_int32(*m), &|m| {
            sizeof_varint(*(m) as u64)
        })?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestRepeatedPackedFloat<'a> {
    pub values: Cow<'a, [f32]>,
}

impl<'a> MessageRead<'a> for TestRepeatedPackedFloat<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.values = Cow::Borrowed(r.read_packed_fixed(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestRepeatedPackedFloat<'a> {
    fn get_size(&self) -> usize {
        0 + if self.values.is_empty() {
            0
        } else {
            1 + sizeof_len(self.values.len() * 4)
        }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_fixed_with_tag(10, &self.values)?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestRepeatedMessages {
    pub messages1: Vec<TestRepeatedMessages>,
    pub messages2: Vec<TestRepeatedMessages>,
    pub messages3: Vec<TestRepeatedMessages>,
}

impl<'a> MessageRead<'a> for TestRepeatedMessages {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg
                    .messages1
                    .push(r.read_message::<TestRepeatedMessages>(bytes)?),
                Ok(18) => msg
                    .messages2
                    .push(r.read_message::<TestRepeatedMessages>(bytes)?),
                Ok(26) => msg
                    .messages3
                    .push(r.read_message::<TestRepeatedMessages>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestRepeatedMessages {
    fn get_size(&self) -> usize {
        0 + self
            .messages1
            .iter()
            .map(|s| 1 + sizeof_len((s).get_size()))
            .sum::<usize>()
            + self
                .messages2
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
            + self
                .messages3
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.messages1 {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        for s in &self.messages2 {
            w.write_with_tag(18, |w| w.write_message(s))?;
        }
        for s in &self.messages3 {
            w.write_with_tag(26, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestOptionalMessages {
    pub message1: Option<Box<TestOptionalMessages>>,
    pub message2: Option<Box<TestOptionalMessages>>,
    pub message3: Option<Box<TestOptionalMessages>>,
}

impl<'a> MessageRead<'a> for TestOptionalMessages {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    msg.message1 = Some(Box::new(r.read_message::<TestOptionalMessages>(bytes)?))
                }
                Ok(18) => {
                    msg.message2 = Some(Box::new(r.read_message::<TestOptionalMessages>(bytes)?))
                }
                Ok(26) => {
                    msg.message3 = Some(Box::new(r.read_message::<TestOptionalMessages>(bytes)?))
                }
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestOptionalMessages {
    fn get_size(&self) -> usize {
        0 + self
            .message1
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
            + self
                .message2
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).get_size()))
            + self
                .message3
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.message1 {
            w.write_with_tag(10, |w| w.write_message(&**s))?;
        }
        if let Some(ref s) = self.message2 {
            w.write_with_tag(18, |w| w.write_message(&**s))?;
        }
        if let Some(ref s) = self.message3 {
            w.write_with_tag(26, |w| w.write_message(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestStrings<'a> {
    pub s1: Option<Cow<'a, str>>,
    pub s2: Option<Cow<'a, str>>,
    pub s3: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for TestStrings<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.s1 = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.s2 = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.s3 = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestStrings<'a> {
    fn get_size(&self) -> usize {
        0 + self.s1.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
            + self.s2.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
            + self.s3.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.s1 {
            w.write_with_tag(10, |w| w.write_string(&**s))?;
        }
        if let Some(ref s) = self.s2 {
            w.write_with_tag(18, |w| w.write_string(&**s))?;
        }
        if let Some(ref s) = self.s3 {
            w.write_with_tag(26, |w| w.write_string(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestBytes<'a> {
    pub b1: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for TestBytes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.b1 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestBytes<'a> {
    fn get_size(&self) -> usize {
        0 + self.b1.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.b1 {
            w.write_with_tag(10, |w| w.write_bytes(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestMap<'a> {
    pub value: HashMap<Cow<'a, str>, u32>,
}

impl<'a> MessageRead<'a> for TestMap<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    let (key, value) = r.read_map(
                        bytes,
                        |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                        |r, bytes| r.read_uint32(bytes),
                    )?;
                    msg.value.insert(key, value);
                }
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TestMap<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .value
            .iter()
            .map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_varint(*(v) as u64)))
            .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for (k, v) in self.value.iter() {
            w.write_with_tag(10, |w| {
                w.write_map(
                    2 + sizeof_len((k).len()) + sizeof_varint(*(v) as u64),
                    10,
                    |w| w.write_string(&**k),
                    16,
                    |w| w.write_uint32(*v),
                )
            })?;
        }
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
    pub test_repeated_packed_float: Vec<TestRepeatedPackedFloat<'a>>,
    pub test_small_bytearrays: Vec<TestBytes<'a>>,
    pub test_large_bytearrays: Vec<TestBytes<'a>>,
    pub test_map: Vec<TestMap<'a>>,
}

impl<'a> MessageRead<'a> for PerftestData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.test1.push(r.read_message::<Test1>(bytes)?),
                Ok(18) => msg
                    .test_repeated_bool
                    .push(r.read_message::<TestRepeatedBool>(bytes)?),
                Ok(26) => msg
                    .test_repeated_messages
                    .push(r.read_message::<TestRepeatedMessages>(bytes)?),
                Ok(34) => msg
                    .test_optional_messages
                    .push(r.read_message::<TestOptionalMessages>(bytes)?),
                Ok(42) => msg.test_strings.push(r.read_message::<TestStrings>(bytes)?),
                Ok(50) => msg
                    .test_repeated_packed_int32
                    .push(r.read_message::<TestRepeatedPackedInt32>(bytes)?),
                Ok(58) => msg
                    .test_repeated_packed_float
                    .push(r.read_message::<TestRepeatedPackedFloat>(bytes)?),
                Ok(66) => msg
                    .test_small_bytearrays
                    .push(r.read_message::<TestBytes>(bytes)?),
                Ok(74) => msg
                    .test_large_bytearrays
                    .push(r.read_message::<TestBytes>(bytes)?),
                Ok(82) => msg.test_map.push(r.read_message::<TestMap>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PerftestData<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .test1
            .iter()
            .map(|s| 1 + sizeof_len((s).get_size()))
            .sum::<usize>()
            + self
                .test_repeated_bool
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
            + self
                .test_repeated_messages
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
            + self
                .test_optional_messages
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
            + self
                .test_strings
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
            + self
                .test_repeated_packed_int32
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
            + self
                .test_repeated_packed_float
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
            + self
                .test_small_bytearrays
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
            + self
                .test_large_bytearrays
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
            + self
                .test_map
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.test1 {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        for s in &self.test_repeated_bool {
            w.write_with_tag(18, |w| w.write_message(s))?;
        }
        for s in &self.test_repeated_messages {
            w.write_with_tag(26, |w| w.write_message(s))?;
        }
        for s in &self.test_optional_messages {
            w.write_with_tag(34, |w| w.write_message(s))?;
        }
        for s in &self.test_strings {
            w.write_with_tag(42, |w| w.write_message(s))?;
        }
        for s in &self.test_repeated_packed_int32 {
            w.write_with_tag(50, |w| w.write_message(s))?;
        }
        for s in &self.test_repeated_packed_float {
            w.write_with_tag(58, |w| w.write_message(s))?;
        }
        for s in &self.test_small_bytearrays {
            w.write_with_tag(66, |w| w.write_message(s))?;
        }
        for s in &self.test_large_bytearrays {
            w.write_with_tag(74, |w| w.write_message(s))?;
        }
        for s in &self.test_map {
            w.write_with_tag(82, |w| w.write_message(s))?;
        }
        Ok(())
    }
}
