// Automatically generated rust module for 'mesos.proto' file

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
use super::super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    DRIVER_NOT_STARTED = 1,
    DRIVER_RUNNING = 2,
    DRIVER_ABORTED = 3,
    DRIVER_STOPPED = 4,
}

impl Default for Status {
    fn default() -> Self {
        Status::DRIVER_NOT_STARTED
    }
}

impl From<i32> for Status {
    fn from(i: i32) -> Self {
        match i {
            1 => Status::DRIVER_NOT_STARTED,
            2 => Status::DRIVER_RUNNING,
            3 => Status::DRIVER_ABORTED,
            4 => Status::DRIVER_STOPPED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Status {
    fn from(s: &'a str) -> Self {
        match s {
            "DRIVER_NOT_STARTED" => Status::DRIVER_NOT_STARTED,
            "DRIVER_RUNNING" => Status::DRIVER_RUNNING,
            "DRIVER_ABORTED" => Status::DRIVER_ABORTED,
            "DRIVER_STOPPED" => Status::DRIVER_STOPPED,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TaskState {
    TASK_STAGING = 6,
    TASK_STARTING = 0,
    TASK_RUNNING = 1,
    TASK_KILLING = 8,
    TASK_FINISHED = 2,
    TASK_FAILED = 3,
    TASK_KILLED = 4,
    TASK_ERROR = 7,
    TASK_LOST = 5,
    TASK_DROPPED = 9,
    TASK_UNREACHABLE = 10,
    TASK_GONE = 11,
    TASK_GONE_BY_OPERATOR = 12,
    TASK_UNKNOWN = 13,
}

impl Default for TaskState {
    fn default() -> Self {
        TaskState::TASK_STAGING
    }
}

impl From<i32> for TaskState {
    fn from(i: i32) -> Self {
        match i {
            6 => TaskState::TASK_STAGING,
            0 => TaskState::TASK_STARTING,
            1 => TaskState::TASK_RUNNING,
            8 => TaskState::TASK_KILLING,
            2 => TaskState::TASK_FINISHED,
            3 => TaskState::TASK_FAILED,
            4 => TaskState::TASK_KILLED,
            7 => TaskState::TASK_ERROR,
            5 => TaskState::TASK_LOST,
            9 => TaskState::TASK_DROPPED,
            10 => TaskState::TASK_UNREACHABLE,
            11 => TaskState::TASK_GONE,
            12 => TaskState::TASK_GONE_BY_OPERATOR,
            13 => TaskState::TASK_UNKNOWN,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for TaskState {
    fn from(s: &'a str) -> Self {
        match s {
            "TASK_STAGING" => TaskState::TASK_STAGING,
            "TASK_STARTING" => TaskState::TASK_STARTING,
            "TASK_RUNNING" => TaskState::TASK_RUNNING,
            "TASK_KILLING" => TaskState::TASK_KILLING,
            "TASK_FINISHED" => TaskState::TASK_FINISHED,
            "TASK_FAILED" => TaskState::TASK_FAILED,
            "TASK_KILLED" => TaskState::TASK_KILLED,
            "TASK_ERROR" => TaskState::TASK_ERROR,
            "TASK_LOST" => TaskState::TASK_LOST,
            "TASK_DROPPED" => TaskState::TASK_DROPPED,
            "TASK_UNREACHABLE" => TaskState::TASK_UNREACHABLE,
            "TASK_GONE" => TaskState::TASK_GONE,
            "TASK_GONE_BY_OPERATOR" => TaskState::TASK_GONE_BY_OPERATOR,
            "TASK_UNKNOWN" => TaskState::TASK_UNKNOWN,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OperationState {
    OPERATION_UNSUPPORTED = 0,
    OPERATION_PENDING = 1,
    OPERATION_FINISHED = 2,
    OPERATION_FAILED = 3,
    OPERATION_ERROR = 4,
    OPERATION_DROPPED = 5,
    OPERATION_UNREACHABLE = 6,
    OPERATION_GONE_BY_OPERATOR = 7,
    OPERATION_RECOVERING = 8,
    OPERATION_UNKNOWN = 9,
}

impl Default for OperationState {
    fn default() -> Self {
        OperationState::OPERATION_UNSUPPORTED
    }
}

impl From<i32> for OperationState {
    fn from(i: i32) -> Self {
        match i {
            0 => OperationState::OPERATION_UNSUPPORTED,
            1 => OperationState::OPERATION_PENDING,
            2 => OperationState::OPERATION_FINISHED,
            3 => OperationState::OPERATION_FAILED,
            4 => OperationState::OPERATION_ERROR,
            5 => OperationState::OPERATION_DROPPED,
            6 => OperationState::OPERATION_UNREACHABLE,
            7 => OperationState::OPERATION_GONE_BY_OPERATOR,
            8 => OperationState::OPERATION_RECOVERING,
            9 => OperationState::OPERATION_UNKNOWN,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for OperationState {
    fn from(s: &'a str) -> Self {
        match s {
            "OPERATION_UNSUPPORTED" => OperationState::OPERATION_UNSUPPORTED,
            "OPERATION_PENDING" => OperationState::OPERATION_PENDING,
            "OPERATION_FINISHED" => OperationState::OPERATION_FINISHED,
            "OPERATION_FAILED" => OperationState::OPERATION_FAILED,
            "OPERATION_ERROR" => OperationState::OPERATION_ERROR,
            "OPERATION_DROPPED" => OperationState::OPERATION_DROPPED,
            "OPERATION_UNREACHABLE" => OperationState::OPERATION_UNREACHABLE,
            "OPERATION_GONE_BY_OPERATOR" => OperationState::OPERATION_GONE_BY_OPERATOR,
            "OPERATION_RECOVERING" => OperationState::OPERATION_RECOVERING,
            "OPERATION_UNKNOWN" => OperationState::OPERATION_UNKNOWN,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FrameworkID<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for FrameworkID<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FrameworkID<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OfferID<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for OfferID<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OfferID<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AgentID<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for AgentID<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AgentID<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TaskID<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for TaskID<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TaskID<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExecutorID<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ExecutorID<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ExecutorID<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ContainerID<'a> {
    pub value: Cow<'a, str>,
    pub parent: Option<Box<mesos::v1::ContainerID<'a>>>,
}

impl<'a> MessageRead<'a> for ContainerID<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.parent = Some(Box::new(r.read_message::<mesos::v1::ContainerID>(bytes)?)),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ContainerID<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.value).len())
        + self.parent.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.value))?;
        if let Some(ref s) = self.parent { w.write_with_tag(18, |w| w.write_message(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResourceProviderID<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ResourceProviderID<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResourceProviderID<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationID<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for OperationID<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OperationID<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TimeInfo {
    pub nanoseconds: i64,
}

impl<'a> MessageRead<'a> for TimeInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.nanoseconds = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TimeInfo {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.nanoseconds) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.nanoseconds))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DurationInfo {
    pub nanoseconds: i64,
}

impl<'a> MessageRead<'a> for DurationInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.nanoseconds = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DurationInfo {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.nanoseconds) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.nanoseconds))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Address<'a> {
    pub hostname: Option<Cow<'a, str>>,
    pub ip: Option<Cow<'a, str>>,
    pub port: i32,
}

impl<'a> MessageRead<'a> for Address<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hostname = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.ip = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.port = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Address<'a> {
    fn get_size(&self) -> usize {
        0
        + self.hostname.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.ip.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + 1 + sizeof_varint(*(&self.port) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.hostname { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.ip { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        w.write_with_tag(24, |w| w.write_int32(*&self.port))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct URL<'a> {
    pub scheme: Cow<'a, str>,
    pub address: mesos::v1::Address<'a>,
    pub path: Option<Cow<'a, str>>,
    pub query: Vec<mesos::v1::Parameter<'a>>,
    pub fragment: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for URL<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.scheme = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.address = r.read_message::<mesos::v1::Address>(bytes)?,
                Ok(26) => msg.path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.query.push(r.read_message::<mesos::v1::Parameter>(bytes)?),
                Ok(42) => msg.fragment = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for URL<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.scheme).len())
        + 1 + sizeof_len((&self.address).get_size())
        + self.path.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.query.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.fragment.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.scheme))?;
        w.write_with_tag(18, |w| w.write_message(&self.address))?;
        if let Some(ref s) = self.path { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        for s in &self.query { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fragment { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Unavailability {
    pub start: mesos::v1::TimeInfo,
    pub duration: Option<mesos::v1::DurationInfo>,
}

impl<'a> MessageRead<'a> for Unavailability {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.start = r.read_message::<mesos::v1::TimeInfo>(bytes)?,
                Ok(18) => msg.duration = Some(r.read_message::<mesos::v1::DurationInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Unavailability {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.start).get_size())
        + self.duration.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.start))?;
        if let Some(ref s) = self.duration { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MachineID<'a> {
    pub hostname: Option<Cow<'a, str>>,
    pub ip: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for MachineID<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hostname = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.ip = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MachineID<'a> {
    fn get_size(&self) -> usize {
        0
        + self.hostname.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.ip.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.hostname { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.ip { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MachineInfo<'a> {
    pub id: mesos::v1::MachineID<'a>,
    pub mode: Option<mesos::v1::mod_MachineInfo::Mode>,
    pub unavailability: Option<mesos::v1::Unavailability>,
}

impl<'a> MessageRead<'a> for MachineInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_message::<mesos::v1::MachineID>(bytes)?,
                Ok(16) => msg.mode = Some(r.read_enum(bytes)?),
                Ok(26) => msg.unavailability = Some(r.read_message::<mesos::v1::Unavailability>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MachineInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).get_size())
        + self.mode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.unavailability.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.id))?;
        if let Some(ref s) = self.mode { w.write_with_tag(16, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.unavailability { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_MachineInfo {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    UP = 1,
    DRAINING = 2,
    DOWN = 3,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::UP
    }
}

impl From<i32> for Mode {
    fn from(i: i32) -> Self {
        match i {
            1 => Mode::UP,
            2 => Mode::DRAINING,
            3 => Mode::DOWN,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Mode {
    fn from(s: &'a str) -> Self {
        match s {
            "UP" => Mode::UP,
            "DRAINING" => Mode::DRAINING,
            "DOWN" => Mode::DOWN,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FrameworkInfo<'a> {
    pub user: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub id: Option<mesos::v1::FrameworkID<'a>>,
    pub failover_timeout: f64,
    pub checkpoint: bool,
    pub role: Cow<'a, str>,
    pub roles: Vec<Cow<'a, str>>,
    pub hostname: Option<Cow<'a, str>>,
    pub principal: Option<Cow<'a, str>>,
    pub webui_url: Option<Cow<'a, str>>,
    pub capabilities: Vec<mesos::v1::mod_FrameworkInfo::Capability>,
    pub labels: Option<mesos::v1::Labels<'a>>,
}

impl<'a> MessageRead<'a> for FrameworkInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = FrameworkInfo {
            failover_timeout: 0.0f64,
            role: Cow::Borrowed("*", deprecated=true),
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.user = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.id = Some(r.read_message::<mesos::v1::FrameworkID>(bytes)?),
                Ok(33) => msg.failover_timeout = r.read_double(bytes)?,
                Ok(40) => msg.checkpoint = r.read_bool(bytes)?,
                Ok(50) => msg.role = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(98) => msg.roles.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.hostname = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.principal = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(74) => msg.webui_url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.capabilities.push(r.read_message::<mesos::v1::mod_FrameworkInfo::Capability>(bytes)?),
                Ok(90) => msg.labels = Some(r.read_message::<mesos::v1::Labels>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FrameworkInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.user).len())
        + 1 + sizeof_len((&self.name).len())
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.failover_timeout == 0.0f64 { 0 } else { 1 + 8 }
        + if self.checkpoint == false { 0 } else { 1 + sizeof_varint(*(&self.checkpoint) as u64) }
        + if self.role == Cow::Borrowed("*", deprecated=true) { 0 } else { 1 + sizeof_len((&self.role).len()) }
        + self.roles.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.hostname.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.principal.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.webui_url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.capabilities.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.labels.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.user))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.name))?;
        if let Some(ref s) = self.id { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.failover_timeout != 0.0f64 { w.write_with_tag(33, |w| w.write_double(*&self.failover_timeout))?; }
        if self.checkpoint != false { w.write_with_tag(40, |w| w.write_bool(*&self.checkpoint))?; }
        if self.role != Cow::Borrowed("*", deprecated=true) { w.write_with_tag(50, |w| w.write_string(&**&self.role))?; }
        for s in &self.roles { w.write_with_tag(98, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.hostname { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.principal { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.webui_url { w.write_with_tag(74, |w| w.write_string(&**s))?; }
        for s in &self.capabilities { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.labels { w.write_with_tag(90, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_FrameworkInfo {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Capability {
    pub type_pb: Option<mesos::v1::mod_FrameworkInfo::mod_Capability::Type>,
}

impl<'a> MessageRead<'a> for Capability {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Capability {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

pub mod mod_Capability {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    REVOCABLE_RESOURCES = 1,
    TASK_KILLING_STATE = 2,
    GPU_RESOURCES = 3,
    SHARED_RESOURCES = 4,
    PARTITION_AWARE = 5,
    MULTI_ROLE = 6,
    RESERVATION_REFINEMENT = 7,
    REGION_AWARE = 8,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::REVOCABLE_RESOURCES,
            2 => Type::TASK_KILLING_STATE,
            3 => Type::GPU_RESOURCES,
            4 => Type::SHARED_RESOURCES,
            5 => Type::PARTITION_AWARE,
            6 => Type::MULTI_ROLE,
            7 => Type::RESERVATION_REFINEMENT,
            8 => Type::REGION_AWARE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "REVOCABLE_RESOURCES" => Type::REVOCABLE_RESOURCES,
            "TASK_KILLING_STATE" => Type::TASK_KILLING_STATE,
            "GPU_RESOURCES" => Type::GPU_RESOURCES,
            "SHARED_RESOURCES" => Type::SHARED_RESOURCES,
            "PARTITION_AWARE" => Type::PARTITION_AWARE,
            "MULTI_ROLE" => Type::MULTI_ROLE,
            "RESERVATION_REFINEMENT" => Type::RESERVATION_REFINEMENT,
            "REGION_AWARE" => Type::REGION_AWARE,
            _ => Self::default(),
        }
    }
}

}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CheckInfo<'a> {
    pub type_pb: Option<mesos::v1::mod_CheckInfo::Type>,
    pub command: Option<mesos::v1::mod_CheckInfo::Command<'a>>,
    pub http: Option<mesos::v1::mod_CheckInfo::Http<'a>>,
    pub tcp: Option<mesos::v1::mod_CheckInfo::Tcp>,
    pub delay_seconds: f64,
    pub interval_seconds: f64,
    pub timeout_seconds: f64,
}

impl<'a> MessageRead<'a> for CheckInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = CheckInfo {
            delay_seconds: 15.0f64,
            interval_seconds: 10.0f64,
            timeout_seconds: 20.0f64,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(18) => msg.command = Some(r.read_message::<mesos::v1::mod_CheckInfo::Command>(bytes)?),
                Ok(26) => msg.http = Some(r.read_message::<mesos::v1::mod_CheckInfo::Http>(bytes)?),
                Ok(58) => msg.tcp = Some(r.read_message::<mesos::v1::mod_CheckInfo::Tcp>(bytes)?),
                Ok(33) => msg.delay_seconds = r.read_double(bytes)?,
                Ok(41) => msg.interval_seconds = r.read_double(bytes)?,
                Ok(49) => msg.timeout_seconds = r.read_double(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CheckInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.command.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.http.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.tcp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.delay_seconds == 15.0f64 { 0 } else { 1 + 8 }
        + if self.interval_seconds == 10.0f64 { 0 } else { 1 + 8 }
        + if self.timeout_seconds == 20.0f64 { 0 } else { 1 + 8 }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.command { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.http { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.tcp { w.write_with_tag(58, |w| w.write_message(s))?; }
        if self.delay_seconds != 15.0f64 { w.write_with_tag(33, |w| w.write_double(*&self.delay_seconds))?; }
        if self.interval_seconds != 10.0f64 { w.write_with_tag(41, |w| w.write_double(*&self.interval_seconds))?; }
        if self.timeout_seconds != 20.0f64 { w.write_with_tag(49, |w| w.write_double(*&self.timeout_seconds))?; }
        Ok(())
    }
}

pub mod mod_CheckInfo {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Command<'a> {
    pub command: mesos::v1::CommandInfo<'a>,
}

impl<'a> MessageRead<'a> for Command<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.command = r.read_message::<mesos::v1::CommandInfo>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Command<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.command).get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.command))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Http<'a> {
    pub port: u32,
    pub path: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Http<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.port = r.read_uint32(bytes)?,
                Ok(18) => msg.path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Http<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.port) as u64)
        + self.path.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.port))?;
        if let Some(ref s) = self.path { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Tcp {
    pub port: u32,
}

impl<'a> MessageRead<'a> for Tcp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.port = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Tcp {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.port) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.port))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    COMMAND = 1,
    HTTP = 2,
    TCP = 3,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::COMMAND,
            2 => Type::HTTP,
            3 => Type::TCP,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "COMMAND" => Type::COMMAND,
            "HTTP" => Type::HTTP,
            "TCP" => Type::TCP,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HealthCheck<'a> {
    pub delay_seconds: f64,
    pub interval_seconds: f64,
    pub timeout_seconds: f64,
    pub consecutive_failures: u32,
    pub grace_period_seconds: f64,
    pub type_pb: Option<mesos::v1::mod_HealthCheck::Type>,
    pub command: Option<mesos::v1::CommandInfo<'a>>,
    pub http: Option<mesos::v1::mod_HealthCheck::HTTPCheckInfo<'a>>,
    pub tcp: Option<mesos::v1::mod_HealthCheck::TCPCheckInfo>,
}

impl<'a> MessageRead<'a> for HealthCheck<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = HealthCheck {
            delay_seconds: 15.0f64,
            interval_seconds: 10.0f64,
            timeout_seconds: 20.0f64,
            consecutive_failures: 3u32,
            grace_period_seconds: 10.0f64,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(17) => msg.delay_seconds = r.read_double(bytes)?,
                Ok(25) => msg.interval_seconds = r.read_double(bytes)?,
                Ok(33) => msg.timeout_seconds = r.read_double(bytes)?,
                Ok(40) => msg.consecutive_failures = r.read_uint32(bytes)?,
                Ok(49) => msg.grace_period_seconds = r.read_double(bytes)?,
                Ok(64) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(58) => msg.command = Some(r.read_message::<mesos::v1::CommandInfo>(bytes)?),
                Ok(10) => msg.http = Some(r.read_message::<mesos::v1::mod_HealthCheck::HTTPCheckInfo>(bytes)?),
                Ok(74) => msg.tcp = Some(r.read_message::<mesos::v1::mod_HealthCheck::TCPCheckInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HealthCheck<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.delay_seconds == 15.0f64 { 0 } else { 1 + 8 }
        + if self.interval_seconds == 10.0f64 { 0 } else { 1 + 8 }
        + if self.timeout_seconds == 20.0f64 { 0 } else { 1 + 8 }
        + if self.consecutive_failures == 3u32 { 0 } else { 1 + sizeof_varint(*(&self.consecutive_failures) as u64) }
        + if self.grace_period_seconds == 10.0f64 { 0 } else { 1 + 8 }
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.command.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.http.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.tcp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.delay_seconds != 15.0f64 { w.write_with_tag(17, |w| w.write_double(*&self.delay_seconds))?; }
        if self.interval_seconds != 10.0f64 { w.write_with_tag(25, |w| w.write_double(*&self.interval_seconds))?; }
        if self.timeout_seconds != 20.0f64 { w.write_with_tag(33, |w| w.write_double(*&self.timeout_seconds))?; }
        if self.consecutive_failures != 3u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.consecutive_failures))?; }
        if self.grace_period_seconds != 10.0f64 { w.write_with_tag(49, |w| w.write_double(*&self.grace_period_seconds))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(64, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.command { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.http { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.tcp { w.write_with_tag(74, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_HealthCheck {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HTTPCheckInfo<'a> {
    pub protocol: mesos::v1::mod_NetworkInfo::Protocol,
    pub scheme: Option<Cow<'a, str>>,
    pub port: u32,
    pub path: Option<Cow<'a, str>>,
    pub statuses: Vec<u32>,
}

impl<'a> MessageRead<'a> for HTTPCheckInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(40) => msg.protocol = r.read_enum(bytes)?,
                Ok(26) => msg.scheme = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(8) => msg.port = r.read_uint32(bytes)?,
                Ok(18) => msg.path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.statuses.push(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HTTPCheckInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.protocol == mesos::v1::mod_NetworkInfo::Protocol::IPv4 { 0 } else { 1 + sizeof_varint(*(&self.protocol) as u64) }
        + self.scheme.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + 1 + sizeof_varint(*(&self.port) as u64)
        + self.path.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.statuses.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.protocol != mesos::v1::mod_NetworkInfo::Protocol::IPv4 { w.write_with_tag(40, |w| w.write_enum(*&self.protocol as i32))?; }
        if let Some(ref s) = self.scheme { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        w.write_with_tag(8, |w| w.write_uint32(*&self.port))?;
        if let Some(ref s) = self.path { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        for s in &self.statuses { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TCPCheckInfo {
    pub protocol: mesos::v1::mod_NetworkInfo::Protocol,
    pub port: u32,
}

impl<'a> MessageRead<'a> for TCPCheckInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.protocol = r.read_enum(bytes)?,
                Ok(8) => msg.port = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TCPCheckInfo {
    fn get_size(&self) -> usize {
        0
        + if self.protocol == mesos::v1::mod_NetworkInfo::Protocol::IPv4 { 0 } else { 1 + sizeof_varint(*(&self.protocol) as u64) }
        + 1 + sizeof_varint(*(&self.port) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.protocol != mesos::v1::mod_NetworkInfo::Protocol::IPv4 { w.write_with_tag(16, |w| w.write_enum(*&self.protocol as i32))?; }
        w.write_with_tag(8, |w| w.write_uint32(*&self.port))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    COMMAND = 1,
    HTTP = 2,
    TCP = 3,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::COMMAND,
            2 => Type::HTTP,
            3 => Type::TCP,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "COMMAND" => Type::COMMAND,
            "HTTP" => Type::HTTP,
            "TCP" => Type::TCP,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct KillPolicy {
    pub grace_period: Option<mesos::v1::DurationInfo>,
}

impl<'a> MessageRead<'a> for KillPolicy {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.grace_period = Some(r.read_message::<mesos::v1::DurationInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for KillPolicy {
    fn get_size(&self) -> usize {
        0
        + self.grace_period.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.grace_period { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CommandInfo<'a> {
    pub uris: Vec<mesos::v1::mod_CommandInfo::URI<'a>>,
    pub environment: Option<mesos::v1::Environment<'a>>,
    pub shell: bool,
    pub value: Option<Cow<'a, str>>,
    pub arguments: Vec<Cow<'a, str>>,
    pub user: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for CommandInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = CommandInfo {
            shell: true,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.uris.push(r.read_message::<mesos::v1::mod_CommandInfo::URI>(bytes)?),
                Ok(18) => msg.environment = Some(r.read_message::<mesos::v1::Environment>(bytes)?),
                Ok(48) => msg.shell = r.read_bool(bytes)?,
                Ok(26) => msg.value = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.arguments.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.user = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CommandInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uris.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.environment.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.shell == true { 0 } else { 1 + sizeof_varint(*(&self.shell) as u64) }
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.arguments.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.user.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.uris { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.environment { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.shell != true { w.write_with_tag(48, |w| w.write_bool(*&self.shell))?; }
        if let Some(ref s) = self.value { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        for s in &self.arguments { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.user { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

pub mod mod_CommandInfo {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct URI<'a> {
    pub value: Cow<'a, str>,
    pub executable: Option<bool>,
    pub extract: bool,
    pub cache: Option<bool>,
    pub output_file: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for URI<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = URI {
            extract: true,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.executable = Some(r.read_bool(bytes)?),
                Ok(24) => msg.extract = r.read_bool(bytes)?,
                Ok(32) => msg.cache = Some(r.read_bool(bytes)?),
                Ok(42) => msg.output_file = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for URI<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.value).len())
        + self.executable.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + if self.extract == true { 0 } else { 1 + sizeof_varint(*(&self.extract) as u64) }
        + self.cache.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.output_file.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.value))?;
        if let Some(ref s) = self.executable { w.write_with_tag(16, |w| w.write_bool(*s))?; }
        if self.extract != true { w.write_with_tag(24, |w| w.write_bool(*&self.extract))?; }
        if let Some(ref s) = self.cache { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.output_file { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExecutorInfo<'a> {
    pub type_pb: Option<mesos::v1::mod_ExecutorInfo::Type>,
    pub executor_id: mesos::v1::ExecutorID<'a>,
    pub framework_id: Option<mesos::v1::FrameworkID<'a>>,
    pub command: Option<mesos::v1::CommandInfo<'a>>,
    pub container: Option<mesos::v1::ContainerInfo<'a>>,
    pub resources: Vec<mesos::v1::Resource<'a>>,
    pub name: Option<Cow<'a, str>>,
    pub data: Option<Cow<'a, [u8]>>,
    pub discovery: Option<mesos::v1::DiscoveryInfo<'a>>,
    pub shutdown_grace_period: Option<mesos::v1::DurationInfo>,
    pub labels: Option<mesos::v1::Labels<'a>>,
}

impl<'a> MessageRead<'a> for ExecutorInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(120) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(10) => msg.executor_id = r.read_message::<mesos::v1::ExecutorID>(bytes)?,
                Ok(66) => msg.framework_id = Some(r.read_message::<mesos::v1::FrameworkID>(bytes)?),
                Ok(58) => msg.command = Some(r.read_message::<mesos::v1::CommandInfo>(bytes)?),
                Ok(90) => msg.container = Some(r.read_message::<mesos::v1::ContainerInfo>(bytes)?),
                Ok(42) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(74) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.discovery = Some(r.read_message::<mesos::v1::DiscoveryInfo>(bytes)?),
                Ok(106) => msg.shutdown_grace_period = Some(r.read_message::<mesos::v1::DurationInfo>(bytes)?),
                Ok(114) => msg.labels = Some(r.read_message::<mesos::v1::Labels>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ExecutorInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + 1 + sizeof_len((&self.executor_id).get_size())
        + self.framework_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.command.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.container.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.discovery.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.shutdown_grace_period.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.labels.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(120, |w| w.write_enum(*s as i32))?; }
        w.write_with_tag(10, |w| w.write_message(&self.executor_id))?;
        if let Some(ref s) = self.framework_id { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.command { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.container { w.write_with_tag(90, |w| w.write_message(s))?; }
        for s in &self.resources { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(74, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.data { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.discovery { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.shutdown_grace_period { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.labels { w.write_with_tag(114, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_ExecutorInfo {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    DEFAULT = 1,
    CUSTOM = 2,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::DEFAULT,
            2 => Type::CUSTOM,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "DEFAULT" => Type::DEFAULT,
            "CUSTOM" => Type::CUSTOM,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DomainInfo<'a> {
    pub fault_domain: Option<mesos::v1::mod_DomainInfo::FaultDomain<'a>>,
}

impl<'a> MessageRead<'a> for DomainInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fault_domain = Some(r.read_message::<mesos::v1::mod_DomainInfo::FaultDomain>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DomainInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fault_domain.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fault_domain { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_DomainInfo {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FaultDomain<'a> {
    pub region: mesos::v1::mod_DomainInfo::mod_FaultDomain::RegionInfo<'a>,
    pub zone: mesos::v1::mod_DomainInfo::mod_FaultDomain::ZoneInfo<'a>,
}

impl<'a> MessageRead<'a> for FaultDomain<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.region = r.read_message::<mesos::v1::mod_DomainInfo::mod_FaultDomain::RegionInfo>(bytes)?,
                Ok(18) => msg.zone = r.read_message::<mesos::v1::mod_DomainInfo::mod_FaultDomain::ZoneInfo>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FaultDomain<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.region).get_size())
        + 1 + sizeof_len((&self.zone).get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.region))?;
        w.write_with_tag(18, |w| w.write_message(&self.zone))?;
        Ok(())
    }
}

pub mod mod_FaultDomain {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RegionInfo<'a> {
    pub name: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for RegionInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RegionInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ZoneInfo<'a> {
    pub name: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ZoneInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ZoneInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        Ok(())
    }
}

}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MasterInfo<'a> {
    pub id: Cow<'a, str>,
    pub ip: u32,
    pub port: u32,
    pub pid: Option<Cow<'a, str>>,
    pub hostname: Option<Cow<'a, str>>,
    pub version: Option<Cow<'a, str>>,
    pub address: Option<mesos::v1::Address<'a>>,
    pub domain: Option<mesos::v1::DomainInfo<'a>>,
    pub capabilities: Vec<mesos::v1::mod_MasterInfo::Capability>,
}

impl<'a> MessageRead<'a> for MasterInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = MasterInfo {
            port: 5050u32,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.ip = r.read_uint32(bytes)?,
                Ok(24) => msg.port = r.read_uint32(bytes)?,
                Ok(34) => msg.pid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.hostname = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.version = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.address = Some(r.read_message::<mesos::v1::Address>(bytes)?),
                Ok(66) => msg.domain = Some(r.read_message::<mesos::v1::DomainInfo>(bytes)?),
                Ok(74) => msg.capabilities.push(r.read_message::<mesos::v1::mod_MasterInfo::Capability>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MasterInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
        + 1 + sizeof_varint(*(&self.ip) as u64)
        + 1 + sizeof_varint(*(&self.port) as u64)
        + self.pid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.hostname.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.version.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.address.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.domain.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.capabilities.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        w.write_with_tag(16, |w| w.write_uint32(*&self.ip))?;
        w.write_with_tag(24, |w| w.write_uint32(*&self.port))?;
        if let Some(ref s) = self.pid { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.hostname { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.version { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.address { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.domain { w.write_with_tag(66, |w| w.write_message(s))?; }
        for s in &self.capabilities { w.write_with_tag(74, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_MasterInfo {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Capability {
    pub type_pb: Option<mesos::v1::mod_MasterInfo::mod_Capability::Type>,
}

impl<'a> MessageRead<'a> for Capability {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Capability {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

pub mod mod_Capability {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    AGENT_UPDATE = 1,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::AGENT_UPDATE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "AGENT_UPDATE" => Type::AGENT_UPDATE,
            _ => Self::default(),
        }
    }
}

}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AgentInfo<'a> {
    pub hostname: Cow<'a, str>,
    pub port: i32,
    pub resources: Vec<mesos::v1::Resource<'a>>,
    pub attributes: Vec<mesos::v1::Attribute<'a>>,
    pub id: Option<mesos::v1::AgentID<'a>>,
    pub domain: Option<mesos::v1::DomainInfo<'a>>,
}

impl<'a> MessageRead<'a> for AgentInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = AgentInfo {
            port: 5051i32,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hostname = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(64) => msg.port = r.read_int32(bytes)?,
                Ok(26) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(42) => msg.attributes.push(r.read_message::<mesos::v1::Attribute>(bytes)?),
                Ok(50) => msg.id = Some(r.read_message::<mesos::v1::AgentID>(bytes)?),
                Ok(82) => msg.domain = Some(r.read_message::<mesos::v1::DomainInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AgentInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.hostname).len())
        + if self.port == 5051i32 { 0 } else { 1 + sizeof_varint(*(&self.port) as u64) }
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.attributes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.domain.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.hostname))?;
        if self.port != 5051i32 { w.write_with_tag(64, |w| w.write_int32(*&self.port))?; }
        for s in &self.resources { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.attributes { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.id { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.domain { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_AgentInfo {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Capability {
    pub type_pb: Option<mesos::v1::mod_AgentInfo::mod_Capability::Type>,
}

impl<'a> MessageRead<'a> for Capability {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Capability {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

pub mod mod_Capability {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    MULTI_ROLE = 1,
    HIERARCHICAL_ROLE = 2,
    RESERVATION_REFINEMENT = 3,
    RESOURCE_PROVIDER = 4,
    RESIZE_VOLUME = 5,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::MULTI_ROLE,
            2 => Type::HIERARCHICAL_ROLE,
            3 => Type::RESERVATION_REFINEMENT,
            4 => Type::RESOURCE_PROVIDER,
            5 => Type::RESIZE_VOLUME,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "MULTI_ROLE" => Type::MULTI_ROLE,
            "HIERARCHICAL_ROLE" => Type::HIERARCHICAL_ROLE,
            "RESERVATION_REFINEMENT" => Type::RESERVATION_REFINEMENT,
            "RESOURCE_PROVIDER" => Type::RESOURCE_PROVIDER,
            "RESIZE_VOLUME" => Type::RESIZE_VOLUME,
            _ => Self::default(),
        }
    }
}

}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CSIPluginContainerInfo<'a> {
    pub services: Vec<mesos::v1::mod_CSIPluginContainerInfo::Service>,
    pub command: Option<mesos::v1::CommandInfo<'a>>,
    pub resources: Vec<mesos::v1::Resource<'a>>,
    pub container: Option<mesos::v1::ContainerInfo<'a>>,
}

impl<'a> MessageRead<'a> for CSIPluginContainerInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.services.push(r.read_enum(bytes)?),
                Ok(18) => msg.command = Some(r.read_message::<mesos::v1::CommandInfo>(bytes)?),
                Ok(26) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(34) => msg.container = Some(r.read_message::<mesos::v1::ContainerInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CSIPluginContainerInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.services.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.command.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.container.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.services { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.command { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.resources { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.container { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_CSIPluginContainerInfo {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Service {
    UNKNOWN = 0,
    CONTROLLER_SERVICE = 1,
    NODE_SERVICE = 2,
}

impl Default for Service {
    fn default() -> Self {
        Service::UNKNOWN
    }
}

impl From<i32> for Service {
    fn from(i: i32) -> Self {
        match i {
            0 => Service::UNKNOWN,
            1 => Service::CONTROLLER_SERVICE,
            2 => Service::NODE_SERVICE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Service {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Service::UNKNOWN,
            "CONTROLLER_SERVICE" => Service::CONTROLLER_SERVICE,
            "NODE_SERVICE" => Service::NODE_SERVICE,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CSIPluginInfo<'a> {
    pub type_pb: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub containers: Vec<mesos::v1::CSIPluginContainerInfo<'a>>,
}

impl<'a> MessageRead<'a> for CSIPluginInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.type_pb = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.containers.push(r.read_message::<mesos::v1::CSIPluginContainerInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CSIPluginInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.type_pb).len())
        + 1 + sizeof_len((&self.name).len())
        + self.containers.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.type_pb))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.name))?;
        for s in &self.containers { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResourceProviderInfo<'a> {
    pub id: Option<mesos::v1::ResourceProviderID<'a>>,
    pub attributes: Vec<mesos::v1::Attribute<'a>>,
    pub type_pb: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub default_reservations: Vec<mesos::v1::mod_Resource::ReservationInfo<'a>>,
    pub storage: Option<mesos::v1::mod_ResourceProviderInfo::Storage<'a>>,
}

impl<'a> MessageRead<'a> for ResourceProviderInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = Some(r.read_message::<mesos::v1::ResourceProviderID>(bytes)?),
                Ok(18) => msg.attributes.push(r.read_message::<mesos::v1::Attribute>(bytes)?),
                Ok(26) => msg.type_pb = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.default_reservations.push(r.read_message::<mesos::v1::mod_Resource::ReservationInfo>(bytes)?),
                Ok(50) => msg.storage = Some(r.read_message::<mesos::v1::mod_ResourceProviderInfo::Storage>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResourceProviderInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.attributes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + 1 + sizeof_len((&self.type_pb).len())
        + 1 + sizeof_len((&self.name).len())
        + self.default_reservations.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.storage.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.id { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.attributes { w.write_with_tag(18, |w| w.write_message(s))?; }
        w.write_with_tag(26, |w| w.write_string(&**&self.type_pb))?;
        w.write_with_tag(34, |w| w.write_string(&**&self.name))?;
        for s in &self.default_reservations { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.storage { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_ResourceProviderInfo {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Storage<'a> {
    pub plugin: mesos::v1::CSIPluginInfo<'a>,
}

impl<'a> MessageRead<'a> for Storage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.plugin = r.read_message::<mesos::v1::CSIPluginInfo>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Storage<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.plugin).get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.plugin))?;
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Value<'a> {
    pub type_pb: mesos::v1::mod_Value::Type,
    pub scalar: Option<mesos::v1::mod_Value::Scalar>,
    pub ranges: Option<mesos::v1::mod_Value::Ranges>,
    pub set: Option<mesos::v1::mod_Value::Set<'a>>,
    pub text: Option<mesos::v1::mod_Value::Text<'a>>,
}

impl<'a> MessageRead<'a> for Value<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(18) => msg.scalar = Some(r.read_message::<mesos::v1::mod_Value::Scalar>(bytes)?),
                Ok(26) => msg.ranges = Some(r.read_message::<mesos::v1::mod_Value::Ranges>(bytes)?),
                Ok(34) => msg.set = Some(r.read_message::<mesos::v1::mod_Value::Set>(bytes)?),
                Ok(42) => msg.text = Some(r.read_message::<mesos::v1::mod_Value::Text>(bytes)?),
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
        + 1 + sizeof_varint(*(&self.type_pb) as u64)
        + self.scalar.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.ranges.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.set.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.text.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?;
        if let Some(ref s) = self.scalar { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.ranges { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.set { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.text { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Value {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Scalar {
    pub value: f64,
}

impl<'a> MessageRead<'a> for Scalar {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.value = r.read_double(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Scalar {
    fn get_size(&self) -> usize {
        0
        + 1 + 8
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(9, |w| w.write_double(*&self.value))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Range {
    pub begin: u64,
    pub end: u64,
}

impl<'a> MessageRead<'a> for Range {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.begin = r.read_uint64(bytes)?,
                Ok(16) => msg.end = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Range {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.begin) as u64)
        + 1 + sizeof_varint(*(&self.end) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint64(*&self.begin))?;
        w.write_with_tag(16, |w| w.write_uint64(*&self.end))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ranges {
    pub range: Vec<mesos::v1::mod_Value::Range>,
}

impl<'a> MessageRead<'a> for Ranges {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.range.push(r.read_message::<mesos::v1::mod_Value::Range>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Ranges {
    fn get_size(&self) -> usize {
        0
        + self.range.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.range { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Set<'a> {
    pub item: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Set<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.item.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Set<'a> {
    fn get_size(&self) -> usize {
        0
        + self.item.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.item { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Text<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Text<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Text<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    SCALAR = 0,
    RANGES = 1,
    SET = 2,
    TEXT = 3,
}

impl Default for Type {
    fn default() -> Self {
        Type::SCALAR
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::SCALAR,
            1 => Type::RANGES,
            2 => Type::SET,
            3 => Type::TEXT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "SCALAR" => Type::SCALAR,
            "RANGES" => Type::RANGES,
            "SET" => Type::SET,
            "TEXT" => Type::TEXT,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Attribute<'a> {
    pub name: Cow<'a, str>,
    pub type_pb: mesos::v1::mod_Value::Type,
    pub scalar: Option<mesos::v1::mod_Value::Scalar>,
    pub ranges: Option<mesos::v1::mod_Value::Ranges>,
    pub set: Option<mesos::v1::mod_Value::Set<'a>>,
    pub text: Option<mesos::v1::mod_Value::Text<'a>>,
}

impl<'a> MessageRead<'a> for Attribute<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(26) => msg.scalar = Some(r.read_message::<mesos::v1::mod_Value::Scalar>(bytes)?),
                Ok(34) => msg.ranges = Some(r.read_message::<mesos::v1::mod_Value::Ranges>(bytes)?),
                Ok(50) => msg.set = Some(r.read_message::<mesos::v1::mod_Value::Set>(bytes)?),
                Ok(42) => msg.text = Some(r.read_message::<mesos::v1::mod_Value::Text>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Attribute<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_varint(*(&self.type_pb) as u64)
        + self.scalar.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.ranges.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.set.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.text.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?;
        if let Some(ref s) = self.scalar { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.ranges { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.set { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.text { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Resource<'a> {
    pub provider_id: Option<mesos::v1::ResourceProviderID<'a>>,
    pub name: Cow<'a, str>,
    pub type_pb: mesos::v1::mod_Value::Type,
    pub scalar: Option<mesos::v1::mod_Value::Scalar>,
    pub ranges: Option<mesos::v1::mod_Value::Ranges>,
    pub set: Option<mesos::v1::mod_Value::Set<'a>>,
    pub role: Cow<'a, str>,
    pub allocation_info: Option<mesos::v1::mod_Resource::AllocationInfo<'a>>,
    pub reservation: Option<mesos::v1::mod_Resource::ReservationInfo<'a>>,
    pub reservations: Vec<mesos::v1::mod_Resource::ReservationInfo<'a>>,
    pub disk: Option<mesos::v1::mod_Resource::DiskInfo<'a>>,
    pub revocable: Option<mesos::v1::mod_Resource::RevocableInfo>,
    pub shared: Option<mesos::v1::mod_Resource::SharedInfo>,
}

impl<'a> MessageRead<'a> for Resource<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Resource {
            role: Cow::Borrowed("*", deprecated=true),
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(98) => msg.provider_id = Some(r.read_message::<mesos::v1::ResourceProviderID>(bytes)?),
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(26) => msg.scalar = Some(r.read_message::<mesos::v1::mod_Value::Scalar>(bytes)?),
                Ok(34) => msg.ranges = Some(r.read_message::<mesos::v1::mod_Value::Ranges>(bytes)?),
                Ok(42) => msg.set = Some(r.read_message::<mesos::v1::mod_Value::Set>(bytes)?),
                Ok(50) => msg.role = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(90) => msg.allocation_info = Some(r.read_message::<mesos::v1::mod_Resource::AllocationInfo>(bytes)?),
                Ok(66) => msg.reservation = Some(r.read_message::<mesos::v1::mod_Resource::ReservationInfo>(bytes)?),
                Ok(106) => msg.reservations.push(r.read_message::<mesos::v1::mod_Resource::ReservationInfo>(bytes)?),
                Ok(58) => msg.disk = Some(r.read_message::<mesos::v1::mod_Resource::DiskInfo>(bytes)?),
                Ok(74) => msg.revocable = Some(r.read_message::<mesos::v1::mod_Resource::RevocableInfo>(bytes)?),
                Ok(82) => msg.shared = Some(r.read_message::<mesos::v1::mod_Resource::SharedInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Resource<'a> {
    fn get_size(&self) -> usize {
        0
        + self.provider_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_varint(*(&self.type_pb) as u64)
        + self.scalar.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.ranges.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.set.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.role == Cow::Borrowed("*", deprecated=true) { 0 } else { 1 + sizeof_len((&self.role).len()) }
        + self.allocation_info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.reservation.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.reservations.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.disk.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.revocable.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.shared.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.provider_id { w.write_with_tag(98, |w| w.write_message(s))?; }
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?;
        if let Some(ref s) = self.scalar { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.ranges { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.set { w.write_with_tag(42, |w| w.write_message(s))?; }
        if self.role != Cow::Borrowed("*", deprecated=true) { w.write_with_tag(50, |w| w.write_string(&**&self.role))?; }
        if let Some(ref s) = self.allocation_info { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.reservation { w.write_with_tag(66, |w| w.write_message(s))?; }
        for s in &self.reservations { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.disk { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.revocable { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.shared { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Resource {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AllocationInfo<'a> {
    pub role: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for AllocationInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.role = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AllocationInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.role.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.role { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReservationInfo<'a> {
    pub type_pb: Option<mesos::v1::mod_Resource::mod_ReservationInfo::Type>,
    pub role: Option<Cow<'a, str>>,
    pub principal: Option<Cow<'a, str>>,
    pub labels: Option<mesos::v1::Labels<'a>>,
}

impl<'a> MessageRead<'a> for ReservationInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(32) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(26) => msg.role = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(10) => msg.principal = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.labels = Some(r.read_message::<mesos::v1::Labels>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ReservationInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.role.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.principal.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.labels.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(32, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.role { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.principal { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.labels { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_ReservationInfo {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    STATIC = 1,
    DYNAMIC = 2,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::STATIC,
            2 => Type::DYNAMIC,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "STATIC" => Type::STATIC,
            "DYNAMIC" => Type::DYNAMIC,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DiskInfo<'a> {
    pub persistence: Option<mesos::v1::mod_Resource::mod_DiskInfo::Persistence<'a>>,
    pub volume: Option<mesos::v1::Volume<'a>>,
    pub source: Option<mesos::v1::mod_Resource::mod_DiskInfo::Source<'a>>,
}

impl<'a> MessageRead<'a> for DiskInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.persistence = Some(r.read_message::<mesos::v1::mod_Resource::mod_DiskInfo::Persistence>(bytes)?),
                Ok(18) => msg.volume = Some(r.read_message::<mesos::v1::Volume>(bytes)?),
                Ok(26) => msg.source = Some(r.read_message::<mesos::v1::mod_Resource::mod_DiskInfo::Source>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DiskInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.persistence.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.volume.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.source.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.persistence { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.volume { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.source { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_DiskInfo {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Persistence<'a> {
    pub id: Cow<'a, str>,
    pub principal: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Persistence<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.principal = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Persistence<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
        + self.principal.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        if let Some(ref s) = self.principal { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Source<'a> {
    pub type_pb: mesos::v1::mod_Resource::mod_DiskInfo::mod_Source::Type,
    pub path: Option<mesos::v1::mod_Resource::mod_DiskInfo::mod_Source::Path<'a>>,
    pub mount: Option<mesos::v1::mod_Resource::mod_DiskInfo::mod_Source::Mount<'a>>,
    pub id: Option<Cow<'a, str>>,
    pub metadata: Option<mesos::v1::Labels<'a>>,
    pub profile: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Source<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(18) => msg.path = Some(r.read_message::<mesos::v1::mod_Resource::mod_DiskInfo::mod_Source::Path>(bytes)?),
                Ok(26) => msg.mount = Some(r.read_message::<mesos::v1::mod_Resource::mod_DiskInfo::mod_Source::Mount>(bytes)?),
                Ok(34) => msg.id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.metadata = Some(r.read_message::<mesos::v1::Labels>(bytes)?),
                Ok(50) => msg.profile = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Source<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.type_pb) as u64)
        + self.path.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.mount.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.metadata.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.profile.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?;
        if let Some(ref s) = self.path { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.mount { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.id { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.metadata { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.profile { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

pub mod mod_Source {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Path<'a> {
    pub root: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Path<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.root = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Path<'a> {
    fn get_size(&self) -> usize {
        0
        + self.root.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.root { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Mount<'a> {
    pub root: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Mount<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.root = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Mount<'a> {
    fn get_size(&self) -> usize {
        0
        + self.root.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.root { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    PATH = 1,
    MOUNT = 2,
    BLOCK = 3,
    RAW = 4,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::PATH,
            2 => Type::MOUNT,
            3 => Type::BLOCK,
            4 => Type::RAW,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "PATH" => Type::PATH,
            "MOUNT" => Type::MOUNT,
            "BLOCK" => Type::BLOCK,
            "RAW" => Type::RAW,
            _ => Self::default(),
        }
    }
}

}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RevocableInfo { }

impl<'a> MessageRead<'a> for RevocableInfo {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RevocableInfo { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SharedInfo { }

impl<'a> MessageRead<'a> for SharedInfo {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for SharedInfo { }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TrafficControlStatistics<'a> {
    pub id: Cow<'a, str>,
    pub backlog: Option<u64>,
    pub bytes: Option<u64>,
    pub drops: Option<u64>,
    pub overlimits: Option<u64>,
    pub packets: Option<u64>,
    pub qlen: Option<u64>,
    pub ratebps: Option<u64>,
    pub ratepps: Option<u64>,
    pub requeues: Option<u64>,
}

impl<'a> MessageRead<'a> for TrafficControlStatistics<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.backlog = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.bytes = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.drops = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.overlimits = Some(r.read_uint64(bytes)?),
                Ok(48) => msg.packets = Some(r.read_uint64(bytes)?),
                Ok(56) => msg.qlen = Some(r.read_uint64(bytes)?),
                Ok(64) => msg.ratebps = Some(r.read_uint64(bytes)?),
                Ok(72) => msg.ratepps = Some(r.read_uint64(bytes)?),
                Ok(80) => msg.requeues = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TrafficControlStatistics<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
        + self.backlog.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bytes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.drops.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.overlimits.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.packets.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.qlen.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ratebps.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ratepps.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.requeues.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        if let Some(ref s) = self.backlog { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.bytes { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.drops { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.overlimits { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.packets { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.qlen { w.write_with_tag(56, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.ratebps { w.write_with_tag(64, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.ratepps { w.write_with_tag(72, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.requeues { w.write_with_tag(80, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct IpStatistics {
    pub Forwarding: Option<i64>,
    pub DefaultTTL: Option<i64>,
    pub InReceives: Option<i64>,
    pub InHdrErrors: Option<i64>,
    pub InAddrErrors: Option<i64>,
    pub ForwDatagrams: Option<i64>,
    pub InUnknownProtos: Option<i64>,
    pub InDiscards: Option<i64>,
    pub InDelivers: Option<i64>,
    pub OutRequests: Option<i64>,
    pub OutDiscards: Option<i64>,
    pub OutNoRoutes: Option<i64>,
    pub ReasmTimeout: Option<i64>,
    pub ReasmReqds: Option<i64>,
    pub ReasmOKs: Option<i64>,
    pub ReasmFails: Option<i64>,
    pub FragOKs: Option<i64>,
    pub FragFails: Option<i64>,
    pub FragCreates: Option<i64>,
}

impl<'a> MessageRead<'a> for IpStatistics {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Forwarding = Some(r.read_int64(bytes)?),
                Ok(16) => msg.DefaultTTL = Some(r.read_int64(bytes)?),
                Ok(24) => msg.InReceives = Some(r.read_int64(bytes)?),
                Ok(32) => msg.InHdrErrors = Some(r.read_int64(bytes)?),
                Ok(40) => msg.InAddrErrors = Some(r.read_int64(bytes)?),
                Ok(48) => msg.ForwDatagrams = Some(r.read_int64(bytes)?),
                Ok(56) => msg.InUnknownProtos = Some(r.read_int64(bytes)?),
                Ok(64) => msg.InDiscards = Some(r.read_int64(bytes)?),
                Ok(72) => msg.InDelivers = Some(r.read_int64(bytes)?),
                Ok(80) => msg.OutRequests = Some(r.read_int64(bytes)?),
                Ok(88) => msg.OutDiscards = Some(r.read_int64(bytes)?),
                Ok(96) => msg.OutNoRoutes = Some(r.read_int64(bytes)?),
                Ok(104) => msg.ReasmTimeout = Some(r.read_int64(bytes)?),
                Ok(112) => msg.ReasmReqds = Some(r.read_int64(bytes)?),
                Ok(120) => msg.ReasmOKs = Some(r.read_int64(bytes)?),
                Ok(128) => msg.ReasmFails = Some(r.read_int64(bytes)?),
                Ok(136) => msg.FragOKs = Some(r.read_int64(bytes)?),
                Ok(144) => msg.FragFails = Some(r.read_int64(bytes)?),
                Ok(152) => msg.FragCreates = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for IpStatistics {
    fn get_size(&self) -> usize {
        0
        + self.Forwarding.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.DefaultTTL.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InReceives.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InHdrErrors.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InAddrErrors.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ForwDatagrams.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InUnknownProtos.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InDiscards.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InDelivers.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.OutRequests.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.OutDiscards.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.OutNoRoutes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ReasmTimeout.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ReasmReqds.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ReasmOKs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ReasmFails.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.FragOKs.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.FragFails.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.FragCreates.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Forwarding { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.DefaultTTL { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InReceives { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InHdrErrors { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InAddrErrors { w.write_with_tag(40, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.ForwDatagrams { w.write_with_tag(48, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InUnknownProtos { w.write_with_tag(56, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InDiscards { w.write_with_tag(64, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InDelivers { w.write_with_tag(72, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutRequests { w.write_with_tag(80, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutDiscards { w.write_with_tag(88, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutNoRoutes { w.write_with_tag(96, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.ReasmTimeout { w.write_with_tag(104, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.ReasmReqds { w.write_with_tag(112, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.ReasmOKs { w.write_with_tag(120, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.ReasmFails { w.write_with_tag(128, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.FragOKs { w.write_with_tag(136, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.FragFails { w.write_with_tag(144, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.FragCreates { w.write_with_tag(152, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct IcmpStatistics {
    pub InMsgs: Option<i64>,
    pub InErrors: Option<i64>,
    pub InCsumErrors: Option<i64>,
    pub InDestUnreachs: Option<i64>,
    pub InTimeExcds: Option<i64>,
    pub InParmProbs: Option<i64>,
    pub InSrcQuenchs: Option<i64>,
    pub InRedirects: Option<i64>,
    pub InEchos: Option<i64>,
    pub InEchoReps: Option<i64>,
    pub InTimestamps: Option<i64>,
    pub InTimestampReps: Option<i64>,
    pub InAddrMasks: Option<i64>,
    pub InAddrMaskReps: Option<i64>,
    pub OutMsgs: Option<i64>,
    pub OutErrors: Option<i64>,
    pub OutDestUnreachs: Option<i64>,
    pub OutTimeExcds: Option<i64>,
    pub OutParmProbs: Option<i64>,
    pub OutSrcQuenchs: Option<i64>,
    pub OutRedirects: Option<i64>,
    pub OutEchos: Option<i64>,
    pub OutEchoReps: Option<i64>,
    pub OutTimestamps: Option<i64>,
    pub OutTimestampReps: Option<i64>,
    pub OutAddrMasks: Option<i64>,
    pub OutAddrMaskReps: Option<i64>,
}

impl<'a> MessageRead<'a> for IcmpStatistics {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.InMsgs = Some(r.read_int64(bytes)?),
                Ok(16) => msg.InErrors = Some(r.read_int64(bytes)?),
                Ok(24) => msg.InCsumErrors = Some(r.read_int64(bytes)?),
                Ok(32) => msg.InDestUnreachs = Some(r.read_int64(bytes)?),
                Ok(40) => msg.InTimeExcds = Some(r.read_int64(bytes)?),
                Ok(48) => msg.InParmProbs = Some(r.read_int64(bytes)?),
                Ok(56) => msg.InSrcQuenchs = Some(r.read_int64(bytes)?),
                Ok(64) => msg.InRedirects = Some(r.read_int64(bytes)?),
                Ok(72) => msg.InEchos = Some(r.read_int64(bytes)?),
                Ok(80) => msg.InEchoReps = Some(r.read_int64(bytes)?),
                Ok(88) => msg.InTimestamps = Some(r.read_int64(bytes)?),
                Ok(96) => msg.InTimestampReps = Some(r.read_int64(bytes)?),
                Ok(104) => msg.InAddrMasks = Some(r.read_int64(bytes)?),
                Ok(112) => msg.InAddrMaskReps = Some(r.read_int64(bytes)?),
                Ok(120) => msg.OutMsgs = Some(r.read_int64(bytes)?),
                Ok(128) => msg.OutErrors = Some(r.read_int64(bytes)?),
                Ok(136) => msg.OutDestUnreachs = Some(r.read_int64(bytes)?),
                Ok(144) => msg.OutTimeExcds = Some(r.read_int64(bytes)?),
                Ok(152) => msg.OutParmProbs = Some(r.read_int64(bytes)?),
                Ok(160) => msg.OutSrcQuenchs = Some(r.read_int64(bytes)?),
                Ok(168) => msg.OutRedirects = Some(r.read_int64(bytes)?),
                Ok(176) => msg.OutEchos = Some(r.read_int64(bytes)?),
                Ok(184) => msg.OutEchoReps = Some(r.read_int64(bytes)?),
                Ok(192) => msg.OutTimestamps = Some(r.read_int64(bytes)?),
                Ok(200) => msg.OutTimestampReps = Some(r.read_int64(bytes)?),
                Ok(208) => msg.OutAddrMasks = Some(r.read_int64(bytes)?),
                Ok(216) => msg.OutAddrMaskReps = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for IcmpStatistics {
    fn get_size(&self) -> usize {
        0
        + self.InMsgs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InErrors.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InCsumErrors.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InDestUnreachs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InTimeExcds.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InParmProbs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InSrcQuenchs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InRedirects.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InEchos.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InEchoReps.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InTimestamps.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InTimestampReps.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InAddrMasks.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InAddrMaskReps.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.OutMsgs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.OutErrors.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.OutDestUnreachs.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.OutTimeExcds.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.OutParmProbs.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.OutSrcQuenchs.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.OutRedirects.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.OutEchos.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.OutEchoReps.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.OutTimestamps.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.OutTimestampReps.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.OutAddrMasks.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.OutAddrMaskReps.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.InMsgs { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InErrors { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InCsumErrors { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InDestUnreachs { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InTimeExcds { w.write_with_tag(40, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InParmProbs { w.write_with_tag(48, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InSrcQuenchs { w.write_with_tag(56, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InRedirects { w.write_with_tag(64, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InEchos { w.write_with_tag(72, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InEchoReps { w.write_with_tag(80, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InTimestamps { w.write_with_tag(88, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InTimestampReps { w.write_with_tag(96, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InAddrMasks { w.write_with_tag(104, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InAddrMaskReps { w.write_with_tag(112, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutMsgs { w.write_with_tag(120, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutErrors { w.write_with_tag(128, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutDestUnreachs { w.write_with_tag(136, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutTimeExcds { w.write_with_tag(144, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutParmProbs { w.write_with_tag(152, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutSrcQuenchs { w.write_with_tag(160, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutRedirects { w.write_with_tag(168, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutEchos { w.write_with_tag(176, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutEchoReps { w.write_with_tag(184, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutTimestamps { w.write_with_tag(192, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutTimestampReps { w.write_with_tag(200, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutAddrMasks { w.write_with_tag(208, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutAddrMaskReps { w.write_with_tag(216, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TcpStatistics {
    pub RtoAlgorithm: Option<i64>,
    pub RtoMin: Option<i64>,
    pub RtoMax: Option<i64>,
    pub MaxConn: Option<i64>,
    pub ActiveOpens: Option<i64>,
    pub PassiveOpens: Option<i64>,
    pub AttemptFails: Option<i64>,
    pub EstabResets: Option<i64>,
    pub CurrEstab: Option<i64>,
    pub InSegs: Option<i64>,
    pub OutSegs: Option<i64>,
    pub RetransSegs: Option<i64>,
    pub InErrs: Option<i64>,
    pub OutRsts: Option<i64>,
    pub InCsumErrors: Option<i64>,
}

impl<'a> MessageRead<'a> for TcpStatistics {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.RtoAlgorithm = Some(r.read_int64(bytes)?),
                Ok(16) => msg.RtoMin = Some(r.read_int64(bytes)?),
                Ok(24) => msg.RtoMax = Some(r.read_int64(bytes)?),
                Ok(32) => msg.MaxConn = Some(r.read_int64(bytes)?),
                Ok(40) => msg.ActiveOpens = Some(r.read_int64(bytes)?),
                Ok(48) => msg.PassiveOpens = Some(r.read_int64(bytes)?),
                Ok(56) => msg.AttemptFails = Some(r.read_int64(bytes)?),
                Ok(64) => msg.EstabResets = Some(r.read_int64(bytes)?),
                Ok(72) => msg.CurrEstab = Some(r.read_int64(bytes)?),
                Ok(80) => msg.InSegs = Some(r.read_int64(bytes)?),
                Ok(88) => msg.OutSegs = Some(r.read_int64(bytes)?),
                Ok(96) => msg.RetransSegs = Some(r.read_int64(bytes)?),
                Ok(104) => msg.InErrs = Some(r.read_int64(bytes)?),
                Ok(112) => msg.OutRsts = Some(r.read_int64(bytes)?),
                Ok(120) => msg.InCsumErrors = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TcpStatistics {
    fn get_size(&self) -> usize {
        0
        + self.RtoAlgorithm.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.RtoMin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.RtoMax.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.MaxConn.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ActiveOpens.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.PassiveOpens.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.AttemptFails.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.EstabResets.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.CurrEstab.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InSegs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.OutSegs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.RetransSegs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InErrs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.OutRsts.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InCsumErrors.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.RtoAlgorithm { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.RtoMin { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.RtoMax { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.MaxConn { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.ActiveOpens { w.write_with_tag(40, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.PassiveOpens { w.write_with_tag(48, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.AttemptFails { w.write_with_tag(56, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.EstabResets { w.write_with_tag(64, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.CurrEstab { w.write_with_tag(72, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InSegs { w.write_with_tag(80, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutSegs { w.write_with_tag(88, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.RetransSegs { w.write_with_tag(96, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InErrs { w.write_with_tag(104, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutRsts { w.write_with_tag(112, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InCsumErrors { w.write_with_tag(120, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UdpStatistics {
    pub InDatagrams: Option<i64>,
    pub NoPorts: Option<i64>,
    pub InErrors: Option<i64>,
    pub OutDatagrams: Option<i64>,
    pub RcvbufErrors: Option<i64>,
    pub SndbufErrors: Option<i64>,
    pub InCsumErrors: Option<i64>,
    pub IgnoredMulti: Option<i64>,
}

impl<'a> MessageRead<'a> for UdpStatistics {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.InDatagrams = Some(r.read_int64(bytes)?),
                Ok(16) => msg.NoPorts = Some(r.read_int64(bytes)?),
                Ok(24) => msg.InErrors = Some(r.read_int64(bytes)?),
                Ok(32) => msg.OutDatagrams = Some(r.read_int64(bytes)?),
                Ok(40) => msg.RcvbufErrors = Some(r.read_int64(bytes)?),
                Ok(48) => msg.SndbufErrors = Some(r.read_int64(bytes)?),
                Ok(56) => msg.InCsumErrors = Some(r.read_int64(bytes)?),
                Ok(64) => msg.IgnoredMulti = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UdpStatistics {
    fn get_size(&self) -> usize {
        0
        + self.InDatagrams.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.NoPorts.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InErrors.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.OutDatagrams.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.RcvbufErrors.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.SndbufErrors.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.InCsumErrors.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.IgnoredMulti.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.InDatagrams { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.NoPorts { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InErrors { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.OutDatagrams { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.RcvbufErrors { w.write_with_tag(40, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.SndbufErrors { w.write_with_tag(48, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.InCsumErrors { w.write_with_tag(56, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.IgnoredMulti { w.write_with_tag(64, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SNMPStatistics {
    pub ip_stats: Option<mesos::v1::IpStatistics>,
    pub icmp_stats: Option<mesos::v1::IcmpStatistics>,
    pub tcp_stats: Option<mesos::v1::TcpStatistics>,
    pub udp_stats: Option<mesos::v1::UdpStatistics>,
}

impl<'a> MessageRead<'a> for SNMPStatistics {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ip_stats = Some(r.read_message::<mesos::v1::IpStatistics>(bytes)?),
                Ok(18) => msg.icmp_stats = Some(r.read_message::<mesos::v1::IcmpStatistics>(bytes)?),
                Ok(26) => msg.tcp_stats = Some(r.read_message::<mesos::v1::TcpStatistics>(bytes)?),
                Ok(34) => msg.udp_stats = Some(r.read_message::<mesos::v1::UdpStatistics>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SNMPStatistics {
    fn get_size(&self) -> usize {
        0
        + self.ip_stats.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.icmp_stats.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.tcp_stats.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.udp_stats.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ip_stats { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.icmp_stats { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.tcp_stats { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.udp_stats { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DiskStatistics<'a> {
    pub source: Option<mesos::v1::mod_Resource::mod_DiskInfo::Source<'a>>,
    pub persistence: Option<mesos::v1::mod_Resource::mod_DiskInfo::Persistence<'a>>,
    pub limit_bytes: Option<u64>,
    pub used_bytes: Option<u64>,
}

impl<'a> MessageRead<'a> for DiskStatistics<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.source = Some(r.read_message::<mesos::v1::mod_Resource::mod_DiskInfo::Source>(bytes)?),
                Ok(18) => msg.persistence = Some(r.read_message::<mesos::v1::mod_Resource::mod_DiskInfo::Persistence>(bytes)?),
                Ok(24) => msg.limit_bytes = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.used_bytes = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DiskStatistics<'a> {
    fn get_size(&self) -> usize {
        0
        + self.source.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.persistence.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.limit_bytes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.used_bytes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.source { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.persistence { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.limit_bytes { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.used_bytes { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResourceStatistics<'a> {
    pub timestamp: f64,
    pub processes: Option<u32>,
    pub threads: Option<u32>,
    pub cpus_user_time_secs: Option<f64>,
    pub cpus_system_time_secs: Option<f64>,
    pub cpus_limit: Option<f64>,
    pub cpus_nr_periods: Option<u32>,
    pub cpus_nr_throttled: Option<u32>,
    pub cpus_throttled_time_secs: Option<f64>,
    pub mem_total_bytes: Option<u64>,
    pub mem_total_memsw_bytes: Option<u64>,
    pub mem_limit_bytes: Option<u64>,
    pub mem_soft_limit_bytes: Option<u64>,
    pub mem_file_bytes: Option<u64>,
    pub mem_anon_bytes: Option<u64>,
    pub mem_cache_bytes: Option<u64>,
    pub mem_rss_bytes: Option<u64>,
    pub mem_mapped_file_bytes: Option<u64>,
    pub mem_swap_bytes: Option<u64>,
    pub mem_unevictable_bytes: Option<u64>,
    pub mem_low_pressure_counter: Option<u64>,
    pub mem_medium_pressure_counter: Option<u64>,
    pub mem_critical_pressure_counter: Option<u64>,
    pub disk_limit_bytes: Option<u64>,
    pub disk_used_bytes: Option<u64>,
    pub disk_statistics: Vec<mesos::v1::DiskStatistics<'a>>,
    pub blkio_statistics: Option<mesos::v1::mod_CgroupInfo::mod_Blkio::Statistics<'a>>,
    pub perf: Option<mesos::v1::PerfStatistics>,
    pub net_rx_packets: Option<u64>,
    pub net_rx_bytes: Option<u64>,
    pub net_rx_errors: Option<u64>,
    pub net_rx_dropped: Option<u64>,
    pub net_tx_packets: Option<u64>,
    pub net_tx_bytes: Option<u64>,
    pub net_tx_errors: Option<u64>,
    pub net_tx_dropped: Option<u64>,
    pub net_tcp_rtt_microsecs_p50: Option<f64>,
    pub net_tcp_rtt_microsecs_p90: Option<f64>,
    pub net_tcp_rtt_microsecs_p95: Option<f64>,
    pub net_tcp_rtt_microsecs_p99: Option<f64>,
    pub net_tcp_active_connections: Option<f64>,
    pub net_tcp_time_wait_connections: Option<f64>,
    pub net_traffic_control_statistics: Vec<mesos::v1::TrafficControlStatistics<'a>>,
    pub net_snmp_statistics: Option<mesos::v1::SNMPStatistics>,
}

impl<'a> MessageRead<'a> for ResourceStatistics<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.timestamp = r.read_double(bytes)?,
                Ok(240) => msg.processes = Some(r.read_uint32(bytes)?),
                Ok(248) => msg.threads = Some(r.read_uint32(bytes)?),
                Ok(17) => msg.cpus_user_time_secs = Some(r.read_double(bytes)?),
                Ok(25) => msg.cpus_system_time_secs = Some(r.read_double(bytes)?),
                Ok(33) => msg.cpus_limit = Some(r.read_double(bytes)?),
                Ok(56) => msg.cpus_nr_periods = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.cpus_nr_throttled = Some(r.read_uint32(bytes)?),
                Ok(73) => msg.cpus_throttled_time_secs = Some(r.read_double(bytes)?),
                Ok(288) => msg.mem_total_bytes = Some(r.read_uint64(bytes)?),
                Ok(296) => msg.mem_total_memsw_bytes = Some(r.read_uint64(bytes)?),
                Ok(48) => msg.mem_limit_bytes = Some(r.read_uint64(bytes)?),
                Ok(304) => msg.mem_soft_limit_bytes = Some(r.read_uint64(bytes)?),
                Ok(80) => msg.mem_file_bytes = Some(r.read_uint64(bytes)?),
                Ok(88) => msg.mem_anon_bytes = Some(r.read_uint64(bytes)?),
                Ok(312) => msg.mem_cache_bytes = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.mem_rss_bytes = Some(r.read_uint64(bytes)?),
                Ok(96) => msg.mem_mapped_file_bytes = Some(r.read_uint64(bytes)?),
                Ok(320) => msg.mem_swap_bytes = Some(r.read_uint64(bytes)?),
                Ok(328) => msg.mem_unevictable_bytes = Some(r.read_uint64(bytes)?),
                Ok(256) => msg.mem_low_pressure_counter = Some(r.read_uint64(bytes)?),
                Ok(264) => msg.mem_medium_pressure_counter = Some(r.read_uint64(bytes)?),
                Ok(272) => msg.mem_critical_pressure_counter = Some(r.read_uint64(bytes)?),
                Ok(208) => msg.disk_limit_bytes = Some(r.read_uint64(bytes)?),
                Ok(216) => msg.disk_used_bytes = Some(r.read_uint64(bytes)?),
                Ok(346) => msg.disk_statistics.push(r.read_message::<mesos::v1::DiskStatistics>(bytes)?),
                Ok(354) => msg.blkio_statistics = Some(r.read_message::<mesos::v1::mod_CgroupInfo::mod_Blkio::Statistics>(bytes)?),
                Ok(106) => msg.perf = Some(r.read_message::<mesos::v1::PerfStatistics>(bytes)?),
                Ok(112) => msg.net_rx_packets = Some(r.read_uint64(bytes)?),
                Ok(120) => msg.net_rx_bytes = Some(r.read_uint64(bytes)?),
                Ok(128) => msg.net_rx_errors = Some(r.read_uint64(bytes)?),
                Ok(136) => msg.net_rx_dropped = Some(r.read_uint64(bytes)?),
                Ok(144) => msg.net_tx_packets = Some(r.read_uint64(bytes)?),
                Ok(152) => msg.net_tx_bytes = Some(r.read_uint64(bytes)?),
                Ok(160) => msg.net_tx_errors = Some(r.read_uint64(bytes)?),
                Ok(168) => msg.net_tx_dropped = Some(r.read_uint64(bytes)?),
                Ok(177) => msg.net_tcp_rtt_microsecs_p50 = Some(r.read_double(bytes)?),
                Ok(185) => msg.net_tcp_rtt_microsecs_p90 = Some(r.read_double(bytes)?),
                Ok(193) => msg.net_tcp_rtt_microsecs_p95 = Some(r.read_double(bytes)?),
                Ok(201) => msg.net_tcp_rtt_microsecs_p99 = Some(r.read_double(bytes)?),
                Ok(225) => msg.net_tcp_active_connections = Some(r.read_double(bytes)?),
                Ok(233) => msg.net_tcp_time_wait_connections = Some(r.read_double(bytes)?),
                Ok(282) => msg.net_traffic_control_statistics.push(r.read_message::<mesos::v1::TrafficControlStatistics>(bytes)?),
                Ok(338) => msg.net_snmp_statistics = Some(r.read_message::<mesos::v1::SNMPStatistics>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResourceStatistics<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + 8
        + self.processes.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.threads.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cpus_user_time_secs.as_ref().map_or(0, |_| 1 + 8)
        + self.cpus_system_time_secs.as_ref().map_or(0, |_| 1 + 8)
        + self.cpus_limit.as_ref().map_or(0, |_| 1 + 8)
        + self.cpus_nr_periods.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cpus_nr_throttled.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cpus_throttled_time_secs.as_ref().map_or(0, |_| 1 + 8)
        + self.mem_total_bytes.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.mem_total_memsw_bytes.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.mem_limit_bytes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.mem_soft_limit_bytes.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.mem_file_bytes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.mem_anon_bytes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.mem_cache_bytes.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.mem_rss_bytes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.mem_mapped_file_bytes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.mem_swap_bytes.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.mem_unevictable_bytes.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.mem_low_pressure_counter.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.mem_medium_pressure_counter.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.mem_critical_pressure_counter.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.disk_limit_bytes.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.disk_used_bytes.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.disk_statistics.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.blkio_statistics.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.perf.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.net_rx_packets.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.net_rx_bytes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.net_rx_errors.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.net_rx_dropped.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.net_tx_packets.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.net_tx_bytes.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.net_tx_errors.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.net_tx_dropped.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.net_tcp_rtt_microsecs_p50.as_ref().map_or(0, |_| 2 + 8)
        + self.net_tcp_rtt_microsecs_p90.as_ref().map_or(0, |_| 2 + 8)
        + self.net_tcp_rtt_microsecs_p95.as_ref().map_or(0, |_| 2 + 8)
        + self.net_tcp_rtt_microsecs_p99.as_ref().map_or(0, |_| 2 + 8)
        + self.net_tcp_active_connections.as_ref().map_or(0, |_| 2 + 8)
        + self.net_tcp_time_wait_connections.as_ref().map_or(0, |_| 2 + 8)
        + self.net_traffic_control_statistics.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.net_snmp_statistics.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(9, |w| w.write_double(*&self.timestamp))?;
        if let Some(ref s) = self.processes { w.write_with_tag(240, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.threads { w.write_with_tag(248, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cpus_user_time_secs { w.write_with_tag(17, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.cpus_system_time_secs { w.write_with_tag(25, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.cpus_limit { w.write_with_tag(33, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.cpus_nr_periods { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cpus_nr_throttled { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cpus_throttled_time_secs { w.write_with_tag(73, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.mem_total_bytes { w.write_with_tag(288, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_total_memsw_bytes { w.write_with_tag(296, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_limit_bytes { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_soft_limit_bytes { w.write_with_tag(304, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_file_bytes { w.write_with_tag(80, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_anon_bytes { w.write_with_tag(88, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_cache_bytes { w.write_with_tag(312, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_rss_bytes { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_mapped_file_bytes { w.write_with_tag(96, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_swap_bytes { w.write_with_tag(320, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_unevictable_bytes { w.write_with_tag(328, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_low_pressure_counter { w.write_with_tag(256, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_medium_pressure_counter { w.write_with_tag(264, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mem_critical_pressure_counter { w.write_with_tag(272, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.disk_limit_bytes { w.write_with_tag(208, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.disk_used_bytes { w.write_with_tag(216, |w| w.write_uint64(*s))?; }
        for s in &self.disk_statistics { w.write_with_tag(346, |w| w.write_message(s))?; }
        if let Some(ref s) = self.blkio_statistics { w.write_with_tag(354, |w| w.write_message(s))?; }
        if let Some(ref s) = self.perf { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.net_rx_packets { w.write_with_tag(112, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.net_rx_bytes { w.write_with_tag(120, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.net_rx_errors { w.write_with_tag(128, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.net_rx_dropped { w.write_with_tag(136, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.net_tx_packets { w.write_with_tag(144, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.net_tx_bytes { w.write_with_tag(152, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.net_tx_errors { w.write_with_tag(160, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.net_tx_dropped { w.write_with_tag(168, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.net_tcp_rtt_microsecs_p50 { w.write_with_tag(177, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.net_tcp_rtt_microsecs_p90 { w.write_with_tag(185, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.net_tcp_rtt_microsecs_p95 { w.write_with_tag(193, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.net_tcp_rtt_microsecs_p99 { w.write_with_tag(201, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.net_tcp_active_connections { w.write_with_tag(225, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.net_tcp_time_wait_connections { w.write_with_tag(233, |w| w.write_double(*s))?; }
        for s in &self.net_traffic_control_statistics { w.write_with_tag(282, |w| w.write_message(s))?; }
        if let Some(ref s) = self.net_snmp_statistics { w.write_with_tag(338, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResourceUsage<'a> {
    pub executors: Vec<mesos::v1::mod_ResourceUsage::Executor<'a>>,
    pub total: Vec<mesos::v1::Resource<'a>>,
}

impl<'a> MessageRead<'a> for ResourceUsage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.executors.push(r.read_message::<mesos::v1::mod_ResourceUsage::Executor>(bytes)?),
                Ok(18) => msg.total.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResourceUsage<'a> {
    fn get_size(&self) -> usize {
        0
        + self.executors.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.total.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.executors { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.total { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_ResourceUsage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Executor<'a> {
    pub executor_info: mesos::v1::ExecutorInfo<'a>,
    pub allocated: Vec<mesos::v1::Resource<'a>>,
    pub statistics: Option<mesos::v1::ResourceStatistics<'a>>,
    pub container_id: mesos::v1::ContainerID<'a>,
    pub tasks: Vec<mesos::v1::Task<'a>>,
}

impl<'a> MessageRead<'a> for Executor<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.executor_info = r.read_message::<mesos::v1::ExecutorInfo>(bytes)?,
                Ok(18) => msg.allocated.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(26) => msg.statistics = Some(r.read_message::<mesos::v1::ResourceStatistics>(bytes)?),
                Ok(34) => msg.container_id = r.read_message::<mesos::v1::ContainerID>(bytes)?,
                Ok(42) => msg.tasks.push(r.read_message::<mesos::v1::Task>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Executor<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.executor_info).get_size())
        + self.allocated.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.statistics.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + 1 + sizeof_len((&self.container_id).get_size())
        + self.tasks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.executor_info))?;
        for s in &self.allocated { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.statistics { w.write_with_tag(26, |w| w.write_message(s))?; }
        w.write_with_tag(34, |w| w.write_message(&self.container_id))?;
        for s in &self.tasks { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Executor {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Task<'a> {
    pub name: Cow<'a, str>,
    pub id: mesos::v1::TaskID<'a>,
    pub resources: Vec<mesos::v1::Resource<'a>>,
    pub labels: Option<mesos::v1::Labels<'a>>,
}

impl<'a> MessageRead<'a> for Task<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.id = r.read_message::<mesos::v1::TaskID>(bytes)?,
                Ok(26) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(34) => msg.labels = Some(r.read_message::<mesos::v1::Labels>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Task<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_len((&self.id).get_size())
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.labels.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(18, |w| w.write_message(&self.id))?;
        for s in &self.resources { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.labels { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PerfStatistics {
    pub timestamp: f64,
    pub duration: f64,
    pub cycles: Option<u64>,
    pub stalled_cycles_frontend: Option<u64>,
    pub stalled_cycles_backend: Option<u64>,
    pub instructions: Option<u64>,
    pub cache_references: Option<u64>,
    pub cache_misses: Option<u64>,
    pub branches: Option<u64>,
    pub branch_misses: Option<u64>,
    pub bus_cycles: Option<u64>,
    pub ref_cycles: Option<u64>,
    pub cpu_clock: Option<f64>,
    pub task_clock: Option<f64>,
    pub page_faults: Option<u64>,
    pub minor_faults: Option<u64>,
    pub major_faults: Option<u64>,
    pub context_switches: Option<u64>,
    pub cpu_migrations: Option<u64>,
    pub alignment_faults: Option<u64>,
    pub emulation_faults: Option<u64>,
    pub l1_dcache_loads: Option<u64>,
    pub l1_dcache_load_misses: Option<u64>,
    pub l1_dcache_stores: Option<u64>,
    pub l1_dcache_store_misses: Option<u64>,
    pub l1_dcache_prefetches: Option<u64>,
    pub l1_dcache_prefetch_misses: Option<u64>,
    pub l1_icache_loads: Option<u64>,
    pub l1_icache_load_misses: Option<u64>,
    pub l1_icache_prefetches: Option<u64>,
    pub l1_icache_prefetch_misses: Option<u64>,
    pub llc_loads: Option<u64>,
    pub llc_load_misses: Option<u64>,
    pub llc_stores: Option<u64>,
    pub llc_store_misses: Option<u64>,
    pub llc_prefetches: Option<u64>,
    pub llc_prefetch_misses: Option<u64>,
    pub dtlb_loads: Option<u64>,
    pub dtlb_load_misses: Option<u64>,
    pub dtlb_stores: Option<u64>,
    pub dtlb_store_misses: Option<u64>,
    pub dtlb_prefetches: Option<u64>,
    pub dtlb_prefetch_misses: Option<u64>,
    pub itlb_loads: Option<u64>,
    pub itlb_load_misses: Option<u64>,
    pub branch_loads: Option<u64>,
    pub branch_load_misses: Option<u64>,
    pub node_loads: Option<u64>,
    pub node_load_misses: Option<u64>,
    pub node_stores: Option<u64>,
    pub node_store_misses: Option<u64>,
    pub node_prefetches: Option<u64>,
    pub node_prefetch_misses: Option<u64>,
}

impl<'a> MessageRead<'a> for PerfStatistics {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.timestamp = r.read_double(bytes)?,
                Ok(17) => msg.duration = r.read_double(bytes)?,
                Ok(24) => msg.cycles = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.stalled_cycles_frontend = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.stalled_cycles_backend = Some(r.read_uint64(bytes)?),
                Ok(48) => msg.instructions = Some(r.read_uint64(bytes)?),
                Ok(56) => msg.cache_references = Some(r.read_uint64(bytes)?),
                Ok(64) => msg.cache_misses = Some(r.read_uint64(bytes)?),
                Ok(72) => msg.branches = Some(r.read_uint64(bytes)?),
                Ok(80) => msg.branch_misses = Some(r.read_uint64(bytes)?),
                Ok(88) => msg.bus_cycles = Some(r.read_uint64(bytes)?),
                Ok(96) => msg.ref_cycles = Some(r.read_uint64(bytes)?),
                Ok(105) => msg.cpu_clock = Some(r.read_double(bytes)?),
                Ok(113) => msg.task_clock = Some(r.read_double(bytes)?),
                Ok(120) => msg.page_faults = Some(r.read_uint64(bytes)?),
                Ok(128) => msg.minor_faults = Some(r.read_uint64(bytes)?),
                Ok(136) => msg.major_faults = Some(r.read_uint64(bytes)?),
                Ok(144) => msg.context_switches = Some(r.read_uint64(bytes)?),
                Ok(152) => msg.cpu_migrations = Some(r.read_uint64(bytes)?),
                Ok(160) => msg.alignment_faults = Some(r.read_uint64(bytes)?),
                Ok(168) => msg.emulation_faults = Some(r.read_uint64(bytes)?),
                Ok(176) => msg.l1_dcache_loads = Some(r.read_uint64(bytes)?),
                Ok(184) => msg.l1_dcache_load_misses = Some(r.read_uint64(bytes)?),
                Ok(192) => msg.l1_dcache_stores = Some(r.read_uint64(bytes)?),
                Ok(200) => msg.l1_dcache_store_misses = Some(r.read_uint64(bytes)?),
                Ok(208) => msg.l1_dcache_prefetches = Some(r.read_uint64(bytes)?),
                Ok(216) => msg.l1_dcache_prefetch_misses = Some(r.read_uint64(bytes)?),
                Ok(224) => msg.l1_icache_loads = Some(r.read_uint64(bytes)?),
                Ok(232) => msg.l1_icache_load_misses = Some(r.read_uint64(bytes)?),
                Ok(240) => msg.l1_icache_prefetches = Some(r.read_uint64(bytes)?),
                Ok(248) => msg.l1_icache_prefetch_misses = Some(r.read_uint64(bytes)?),
                Ok(256) => msg.llc_loads = Some(r.read_uint64(bytes)?),
                Ok(264) => msg.llc_load_misses = Some(r.read_uint64(bytes)?),
                Ok(272) => msg.llc_stores = Some(r.read_uint64(bytes)?),
                Ok(280) => msg.llc_store_misses = Some(r.read_uint64(bytes)?),
                Ok(288) => msg.llc_prefetches = Some(r.read_uint64(bytes)?),
                Ok(296) => msg.llc_prefetch_misses = Some(r.read_uint64(bytes)?),
                Ok(304) => msg.dtlb_loads = Some(r.read_uint64(bytes)?),
                Ok(312) => msg.dtlb_load_misses = Some(r.read_uint64(bytes)?),
                Ok(320) => msg.dtlb_stores = Some(r.read_uint64(bytes)?),
                Ok(328) => msg.dtlb_store_misses = Some(r.read_uint64(bytes)?),
                Ok(336) => msg.dtlb_prefetches = Some(r.read_uint64(bytes)?),
                Ok(344) => msg.dtlb_prefetch_misses = Some(r.read_uint64(bytes)?),
                Ok(352) => msg.itlb_loads = Some(r.read_uint64(bytes)?),
                Ok(360) => msg.itlb_load_misses = Some(r.read_uint64(bytes)?),
                Ok(368) => msg.branch_loads = Some(r.read_uint64(bytes)?),
                Ok(376) => msg.branch_load_misses = Some(r.read_uint64(bytes)?),
                Ok(384) => msg.node_loads = Some(r.read_uint64(bytes)?),
                Ok(392) => msg.node_load_misses = Some(r.read_uint64(bytes)?),
                Ok(400) => msg.node_stores = Some(r.read_uint64(bytes)?),
                Ok(408) => msg.node_store_misses = Some(r.read_uint64(bytes)?),
                Ok(416) => msg.node_prefetches = Some(r.read_uint64(bytes)?),
                Ok(424) => msg.node_prefetch_misses = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PerfStatistics {
    fn get_size(&self) -> usize {
        0
        + 1 + 8
        + 1 + 8
        + self.cycles.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.stalled_cycles_frontend.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.stalled_cycles_backend.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.instructions.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cache_references.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cache_misses.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.branches.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.branch_misses.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bus_cycles.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ref_cycles.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cpu_clock.as_ref().map_or(0, |_| 1 + 8)
        + self.task_clock.as_ref().map_or(0, |_| 1 + 8)
        + self.page_faults.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.minor_faults.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.major_faults.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.context_switches.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cpu_migrations.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.alignment_faults.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.emulation_faults.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.l1_dcache_loads.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.l1_dcache_load_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.l1_dcache_stores.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.l1_dcache_store_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.l1_dcache_prefetches.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.l1_dcache_prefetch_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.l1_icache_loads.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.l1_icache_load_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.l1_icache_prefetches.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.l1_icache_prefetch_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.llc_loads.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.llc_load_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.llc_stores.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.llc_store_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.llc_prefetches.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.llc_prefetch_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.dtlb_loads.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.dtlb_load_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.dtlb_stores.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.dtlb_store_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.dtlb_prefetches.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.dtlb_prefetch_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.itlb_loads.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.itlb_load_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.branch_loads.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.branch_load_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.node_loads.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.node_load_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.node_stores.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.node_store_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.node_prefetches.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.node_prefetch_misses.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(9, |w| w.write_double(*&self.timestamp))?;
        w.write_with_tag(17, |w| w.write_double(*&self.duration))?;
        if let Some(ref s) = self.cycles { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.stalled_cycles_frontend { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.stalled_cycles_backend { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.instructions { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.cache_references { w.write_with_tag(56, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.cache_misses { w.write_with_tag(64, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.branches { w.write_with_tag(72, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.branch_misses { w.write_with_tag(80, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.bus_cycles { w.write_with_tag(88, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.ref_cycles { w.write_with_tag(96, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.cpu_clock { w.write_with_tag(105, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.task_clock { w.write_with_tag(113, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.page_faults { w.write_with_tag(120, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.minor_faults { w.write_with_tag(128, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.major_faults { w.write_with_tag(136, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.context_switches { w.write_with_tag(144, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.cpu_migrations { w.write_with_tag(152, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.alignment_faults { w.write_with_tag(160, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.emulation_faults { w.write_with_tag(168, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.l1_dcache_loads { w.write_with_tag(176, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.l1_dcache_load_misses { w.write_with_tag(184, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.l1_dcache_stores { w.write_with_tag(192, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.l1_dcache_store_misses { w.write_with_tag(200, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.l1_dcache_prefetches { w.write_with_tag(208, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.l1_dcache_prefetch_misses { w.write_with_tag(216, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.l1_icache_loads { w.write_with_tag(224, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.l1_icache_load_misses { w.write_with_tag(232, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.l1_icache_prefetches { w.write_with_tag(240, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.l1_icache_prefetch_misses { w.write_with_tag(248, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.llc_loads { w.write_with_tag(256, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.llc_load_misses { w.write_with_tag(264, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.llc_stores { w.write_with_tag(272, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.llc_store_misses { w.write_with_tag(280, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.llc_prefetches { w.write_with_tag(288, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.llc_prefetch_misses { w.write_with_tag(296, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.dtlb_loads { w.write_with_tag(304, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.dtlb_load_misses { w.write_with_tag(312, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.dtlb_stores { w.write_with_tag(320, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.dtlb_store_misses { w.write_with_tag(328, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.dtlb_prefetches { w.write_with_tag(336, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.dtlb_prefetch_misses { w.write_with_tag(344, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.itlb_loads { w.write_with_tag(352, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.itlb_load_misses { w.write_with_tag(360, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.branch_loads { w.write_with_tag(368, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.branch_load_misses { w.write_with_tag(376, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.node_loads { w.write_with_tag(384, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.node_load_misses { w.write_with_tag(392, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.node_stores { w.write_with_tag(400, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.node_store_misses { w.write_with_tag(408, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.node_prefetches { w.write_with_tag(416, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.node_prefetch_misses { w.write_with_tag(424, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Request<'a> {
    pub agent_id: Option<mesos::v1::AgentID<'a>>,
    pub resources: Vec<mesos::v1::Resource<'a>>,
}

impl<'a> MessageRead<'a> for Request<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.agent_id = Some(r.read_message::<mesos::v1::AgentID>(bytes)?),
                Ok(18) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Request<'a> {
    fn get_size(&self) -> usize {
        0
        + self.agent_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.agent_id { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.resources { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Offer<'a> {
    pub id: mesos::v1::OfferID<'a>,
    pub framework_id: mesos::v1::FrameworkID<'a>,
    pub agent_id: mesos::v1::AgentID<'a>,
    pub hostname: Cow<'a, str>,
    pub url: Option<mesos::v1::URL<'a>>,
    pub domain: Option<mesos::v1::DomainInfo<'a>>,
    pub resources: Vec<mesos::v1::Resource<'a>>,
    pub attributes: Vec<mesos::v1::Attribute<'a>>,
    pub executor_ids: Vec<mesos::v1::ExecutorID<'a>>,
    pub unavailability: Option<mesos::v1::Unavailability>,
    pub allocation_info: Option<mesos::v1::mod_Resource::AllocationInfo<'a>>,
}

impl<'a> MessageRead<'a> for Offer<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_message::<mesos::v1::OfferID>(bytes)?,
                Ok(18) => msg.framework_id = r.read_message::<mesos::v1::FrameworkID>(bytes)?,
                Ok(26) => msg.agent_id = r.read_message::<mesos::v1::AgentID>(bytes)?,
                Ok(34) => msg.hostname = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.url = Some(r.read_message::<mesos::v1::URL>(bytes)?),
                Ok(90) => msg.domain = Some(r.read_message::<mesos::v1::DomainInfo>(bytes)?),
                Ok(42) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(58) => msg.attributes.push(r.read_message::<mesos::v1::Attribute>(bytes)?),
                Ok(50) => msg.executor_ids.push(r.read_message::<mesos::v1::ExecutorID>(bytes)?),
                Ok(74) => msg.unavailability = Some(r.read_message::<mesos::v1::Unavailability>(bytes)?),
                Ok(82) => msg.allocation_info = Some(r.read_message::<mesos::v1::mod_Resource::AllocationInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Offer<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).get_size())
        + 1 + sizeof_len((&self.framework_id).get_size())
        + 1 + sizeof_len((&self.agent_id).get_size())
        + 1 + sizeof_len((&self.hostname).len())
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.domain.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.attributes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.executor_ids.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.unavailability.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.allocation_info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.id))?;
        w.write_with_tag(18, |w| w.write_message(&self.framework_id))?;
        w.write_with_tag(26, |w| w.write_message(&self.agent_id))?;
        w.write_with_tag(34, |w| w.write_string(&**&self.hostname))?;
        if let Some(ref s) = self.url { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.domain { w.write_with_tag(90, |w| w.write_message(s))?; }
        for s in &self.resources { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.attributes { w.write_with_tag(58, |w| w.write_message(s))?; }
        for s in &self.executor_ids { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unavailability { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.allocation_info { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Offer {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Operation<'a> {
    pub type_pb: Option<mesos::v1::mod_Offer::mod_Operation::Type>,
    pub id: Option<mesos::v1::OperationID<'a>>,
    pub launch: Option<mesos::v1::mod_Offer::mod_Operation::Launch<'a>>,
    pub launch_group: Option<mesos::v1::mod_Offer::mod_Operation::LaunchGroup<'a>>,
    pub reserve: Option<mesos::v1::mod_Offer::mod_Operation::Reserve<'a>>,
    pub unreserve: Option<mesos::v1::mod_Offer::mod_Operation::Unreserve<'a>>,
    pub create: Option<mesos::v1::mod_Offer::mod_Operation::Create<'a>>,
    pub destroy: Option<mesos::v1::mod_Offer::mod_Operation::Destroy<'a>>,
    pub grow_volume: Option<mesos::v1::mod_Offer::mod_Operation::GrowVolume<'a>>,
    pub shrink_volume: Option<mesos::v1::mod_Offer::mod_Operation::ShrinkVolume<'a>>,
    pub create_disk: Option<mesos::v1::mod_Offer::mod_Operation::CreateDisk<'a>>,
    pub destroy_disk: Option<mesos::v1::mod_Offer::mod_Operation::DestroyDisk<'a>>,
}

impl<'a> MessageRead<'a> for Operation<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(98) => msg.id = Some(r.read_message::<mesos::v1::OperationID>(bytes)?),
                Ok(18) => msg.launch = Some(r.read_message::<mesos::v1::mod_Offer::mod_Operation::Launch>(bytes)?),
                Ok(58) => msg.launch_group = Some(r.read_message::<mesos::v1::mod_Offer::mod_Operation::LaunchGroup>(bytes)?),
                Ok(26) => msg.reserve = Some(r.read_message::<mesos::v1::mod_Offer::mod_Operation::Reserve>(bytes)?),
                Ok(34) => msg.unreserve = Some(r.read_message::<mesos::v1::mod_Offer::mod_Operation::Unreserve>(bytes)?),
                Ok(42) => msg.create = Some(r.read_message::<mesos::v1::mod_Offer::mod_Operation::Create>(bytes)?),
                Ok(50) => msg.destroy = Some(r.read_message::<mesos::v1::mod_Offer::mod_Operation::Destroy>(bytes)?),
                Ok(106) => msg.grow_volume = Some(r.read_message::<mesos::v1::mod_Offer::mod_Operation::GrowVolume>(bytes)?),
                Ok(114) => msg.shrink_volume = Some(r.read_message::<mesos::v1::mod_Offer::mod_Operation::ShrinkVolume>(bytes)?),
                Ok(122) => msg.create_disk = Some(r.read_message::<mesos::v1::mod_Offer::mod_Operation::CreateDisk>(bytes)?),
                Ok(130) => msg.destroy_disk = Some(r.read_message::<mesos::v1::mod_Offer::mod_Operation::DestroyDisk>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Operation<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.launch.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.launch_group.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.reserve.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.unreserve.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.create.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.destroy.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.grow_volume.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.shrink_volume.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.create_disk.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.destroy_disk.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.id { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.launch { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.launch_group { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.reserve { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unreserve { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.create { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.destroy { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.grow_volume { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.shrink_volume { w.write_with_tag(114, |w| w.write_message(s))?; }
        if let Some(ref s) = self.create_disk { w.write_with_tag(122, |w| w.write_message(s))?; }
        if let Some(ref s) = self.destroy_disk { w.write_with_tag(130, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Operation {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Launch<'a> {
    pub task_infos: Vec<mesos::v1::TaskInfo<'a>>,
}

impl<'a> MessageRead<'a> for Launch<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.task_infos.push(r.read_message::<mesos::v1::TaskInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Launch<'a> {
    fn get_size(&self) -> usize {
        0
        + self.task_infos.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.task_infos { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LaunchGroup<'a> {
    pub executor: mesos::v1::ExecutorInfo<'a>,
    pub task_group: mesos::v1::TaskGroupInfo<'a>,
}

impl<'a> MessageRead<'a> for LaunchGroup<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.executor = r.read_message::<mesos::v1::ExecutorInfo>(bytes)?,
                Ok(18) => msg.task_group = r.read_message::<mesos::v1::TaskGroupInfo>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LaunchGroup<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.executor).get_size())
        + 1 + sizeof_len((&self.task_group).get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.executor))?;
        w.write_with_tag(18, |w| w.write_message(&self.task_group))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Reserve<'a> {
    pub resources: Vec<mesos::v1::Resource<'a>>,
}

impl<'a> MessageRead<'a> for Reserve<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Reserve<'a> {
    fn get_size(&self) -> usize {
        0
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.resources { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Unreserve<'a> {
    pub resources: Vec<mesos::v1::Resource<'a>>,
}

impl<'a> MessageRead<'a> for Unreserve<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Unreserve<'a> {
    fn get_size(&self) -> usize {
        0
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.resources { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Create<'a> {
    pub volumes: Vec<mesos::v1::Resource<'a>>,
}

impl<'a> MessageRead<'a> for Create<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.volumes.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Create<'a> {
    fn get_size(&self) -> usize {
        0
        + self.volumes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.volumes { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Destroy<'a> {
    pub volumes: Vec<mesos::v1::Resource<'a>>,
}

impl<'a> MessageRead<'a> for Destroy<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.volumes.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Destroy<'a> {
    fn get_size(&self) -> usize {
        0
        + self.volumes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.volumes { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GrowVolume<'a> {
    pub volume: mesos::v1::Resource<'a>,
    pub addition: mesos::v1::Resource<'a>,
}

impl<'a> MessageRead<'a> for GrowVolume<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.volume = r.read_message::<mesos::v1::Resource>(bytes)?,
                Ok(18) => msg.addition = r.read_message::<mesos::v1::Resource>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GrowVolume<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.volume).get_size())
        + 1 + sizeof_len((&self.addition).get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.volume))?;
        w.write_with_tag(18, |w| w.write_message(&self.addition))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShrinkVolume<'a> {
    pub volume: mesos::v1::Resource<'a>,
    pub subtract: mesos::v1::mod_Value::Scalar,
}

impl<'a> MessageRead<'a> for ShrinkVolume<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.volume = r.read_message::<mesos::v1::Resource>(bytes)?,
                Ok(18) => msg.subtract = r.read_message::<mesos::v1::mod_Value::Scalar>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShrinkVolume<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.volume).get_size())
        + 1 + sizeof_len((&self.subtract).get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.volume))?;
        w.write_with_tag(18, |w| w.write_message(&self.subtract))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreateDisk<'a> {
    pub source: mesos::v1::Resource<'a>,
    pub target_type: mesos::v1::mod_Resource::mod_DiskInfo::mod_Source::Type,
}

impl<'a> MessageRead<'a> for CreateDisk<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.source = r.read_message::<mesos::v1::Resource>(bytes)?,
                Ok(16) => msg.target_type = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CreateDisk<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.source).get_size())
        + 1 + sizeof_varint(*(&self.target_type) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.source))?;
        w.write_with_tag(16, |w| w.write_enum(*&self.target_type as i32))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DestroyDisk<'a> {
    pub source: mesos::v1::Resource<'a>,
}

impl<'a> MessageRead<'a> for DestroyDisk<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.source = r.read_message::<mesos::v1::Resource>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DestroyDisk<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.source).get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.source))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    LAUNCH = 1,
    LAUNCH_GROUP = 6,
    RESERVE = 2,
    UNRESERVE = 3,
    CREATE = 4,
    DESTROY = 5,
    GROW_VOLUME = 11,
    SHRINK_VOLUME = 12,
    CREATE_DISK = 13,
    DESTROY_DISK = 14,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::LAUNCH,
            6 => Type::LAUNCH_GROUP,
            2 => Type::RESERVE,
            3 => Type::UNRESERVE,
            4 => Type::CREATE,
            5 => Type::DESTROY,
            11 => Type::GROW_VOLUME,
            12 => Type::SHRINK_VOLUME,
            13 => Type::CREATE_DISK,
            14 => Type::DESTROY_DISK,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "LAUNCH" => Type::LAUNCH,
            "LAUNCH_GROUP" => Type::LAUNCH_GROUP,
            "RESERVE" => Type::RESERVE,
            "UNRESERVE" => Type::UNRESERVE,
            "CREATE" => Type::CREATE,
            "DESTROY" => Type::DESTROY,
            "GROW_VOLUME" => Type::GROW_VOLUME,
            "SHRINK_VOLUME" => Type::SHRINK_VOLUME,
            "CREATE_DISK" => Type::CREATE_DISK,
            "DESTROY_DISK" => Type::DESTROY_DISK,
            _ => Self::default(),
        }
    }
}

}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct InverseOffer<'a> {
    pub id: mesos::v1::OfferID<'a>,
    pub url: Option<mesos::v1::URL<'a>>,
    pub framework_id: mesos::v1::FrameworkID<'a>,
    pub agent_id: Option<mesos::v1::AgentID<'a>>,
    pub unavailability: mesos::v1::Unavailability,
    pub resources: Vec<mesos::v1::Resource<'a>>,
}

impl<'a> MessageRead<'a> for InverseOffer<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_message::<mesos::v1::OfferID>(bytes)?,
                Ok(18) => msg.url = Some(r.read_message::<mesos::v1::URL>(bytes)?),
                Ok(26) => msg.framework_id = r.read_message::<mesos::v1::FrameworkID>(bytes)?,
                Ok(34) => msg.agent_id = Some(r.read_message::<mesos::v1::AgentID>(bytes)?),
                Ok(42) => msg.unavailability = r.read_message::<mesos::v1::Unavailability>(bytes)?,
                Ok(50) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for InverseOffer<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).get_size())
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + 1 + sizeof_len((&self.framework_id).get_size())
        + self.agent_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + 1 + sizeof_len((&self.unavailability).get_size())
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.id))?;
        if let Some(ref s) = self.url { w.write_with_tag(18, |w| w.write_message(s))?; }
        w.write_with_tag(26, |w| w.write_message(&self.framework_id))?;
        if let Some(ref s) = self.agent_id { w.write_with_tag(34, |w| w.write_message(s))?; }
        w.write_with_tag(42, |w| w.write_message(&self.unavailability))?;
        for s in &self.resources { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TaskInfo<'a> {
    pub name: Cow<'a, str>,
    pub task_id: mesos::v1::TaskID<'a>,
    pub agent_id: mesos::v1::AgentID<'a>,
    pub resources: Vec<mesos::v1::Resource<'a>>,
    pub executor: Option<mesos::v1::ExecutorInfo<'a>>,
    pub command: Option<mesos::v1::CommandInfo<'a>>,
    pub container: Option<mesos::v1::ContainerInfo<'a>>,
    pub health_check: Option<mesos::v1::HealthCheck<'a>>,
    pub check: Option<mesos::v1::CheckInfo<'a>>,
    pub kill_policy: Option<mesos::v1::KillPolicy>,
    pub data: Option<Cow<'a, [u8]>>,
    pub labels: Option<mesos::v1::Labels<'a>>,
    pub discovery: Option<mesos::v1::DiscoveryInfo<'a>>,
    pub max_completion_time: Option<mesos::v1::DurationInfo>,
}

impl<'a> MessageRead<'a> for TaskInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.task_id = r.read_message::<mesos::v1::TaskID>(bytes)?,
                Ok(26) => msg.agent_id = r.read_message::<mesos::v1::AgentID>(bytes)?,
                Ok(34) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(42) => msg.executor = Some(r.read_message::<mesos::v1::ExecutorInfo>(bytes)?),
                Ok(58) => msg.command = Some(r.read_message::<mesos::v1::CommandInfo>(bytes)?),
                Ok(74) => msg.container = Some(r.read_message::<mesos::v1::ContainerInfo>(bytes)?),
                Ok(66) => msg.health_check = Some(r.read_message::<mesos::v1::HealthCheck>(bytes)?),
                Ok(106) => msg.check = Some(r.read_message::<mesos::v1::CheckInfo>(bytes)?),
                Ok(98) => msg.kill_policy = Some(r.read_message::<mesos::v1::KillPolicy>(bytes)?),
                Ok(50) => msg.data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.labels = Some(r.read_message::<mesos::v1::Labels>(bytes)?),
                Ok(90) => msg.discovery = Some(r.read_message::<mesos::v1::DiscoveryInfo>(bytes)?),
                Ok(114) => msg.max_completion_time = Some(r.read_message::<mesos::v1::DurationInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TaskInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_len((&self.task_id).get_size())
        + 1 + sizeof_len((&self.agent_id).get_size())
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.executor.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.command.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.container.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.health_check.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.check.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.kill_policy.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.labels.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.discovery.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.max_completion_time.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(18, |w| w.write_message(&self.task_id))?;
        w.write_with_tag(26, |w| w.write_message(&self.agent_id))?;
        for s in &self.resources { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.executor { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.command { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.container { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.health_check { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.check { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.kill_policy { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.data { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.labels { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.discovery { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.max_completion_time { w.write_with_tag(114, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TaskGroupInfo<'a> {
    pub tasks: Vec<mesos::v1::TaskInfo<'a>>,
}

impl<'a> MessageRead<'a> for TaskGroupInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tasks.push(r.read_message::<mesos::v1::TaskInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TaskGroupInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.tasks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.tasks { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Task<'a> {
    pub name: Cow<'a, str>,
    pub task_id: mesos::v1::TaskID<'a>,
    pub framework_id: mesos::v1::FrameworkID<'a>,
    pub executor_id: Option<mesos::v1::ExecutorID<'a>>,
    pub agent_id: mesos::v1::AgentID<'a>,
    pub state: mesos::v1::TaskState,
    pub resources: Vec<mesos::v1::Resource<'a>>,
    pub statuses: Vec<mesos::v1::TaskStatus<'a>>,
    pub status_update_state: Option<mesos::v1::TaskState>,
    pub status_update_uuid: Option<Cow<'a, [u8]>>,
    pub labels: Option<mesos::v1::Labels<'a>>,
    pub discovery: Option<mesos::v1::DiscoveryInfo<'a>>,
    pub container: Option<mesos::v1::ContainerInfo<'a>>,
    pub user: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Task<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.task_id = r.read_message::<mesos::v1::TaskID>(bytes)?,
                Ok(26) => msg.framework_id = r.read_message::<mesos::v1::FrameworkID>(bytes)?,
                Ok(34) => msg.executor_id = Some(r.read_message::<mesos::v1::ExecutorID>(bytes)?),
                Ok(42) => msg.agent_id = r.read_message::<mesos::v1::AgentID>(bytes)?,
                Ok(48) => msg.state = r.read_enum(bytes)?,
                Ok(58) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(66) => msg.statuses.push(r.read_message::<mesos::v1::TaskStatus>(bytes)?),
                Ok(72) => msg.status_update_state = Some(r.read_enum(bytes)?),
                Ok(82) => msg.status_update_uuid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.labels = Some(r.read_message::<mesos::v1::Labels>(bytes)?),
                Ok(98) => msg.discovery = Some(r.read_message::<mesos::v1::DiscoveryInfo>(bytes)?),
                Ok(106) => msg.container = Some(r.read_message::<mesos::v1::ContainerInfo>(bytes)?),
                Ok(114) => msg.user = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Task<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_len((&self.task_id).get_size())
        + 1 + sizeof_len((&self.framework_id).get_size())
        + self.executor_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + 1 + sizeof_len((&self.agent_id).get_size())
        + 1 + sizeof_varint(*(&self.state) as u64)
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.statuses.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.status_update_state.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.status_update_uuid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.labels.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.discovery.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.container.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.user.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(18, |w| w.write_message(&self.task_id))?;
        w.write_with_tag(26, |w| w.write_message(&self.framework_id))?;
        if let Some(ref s) = self.executor_id { w.write_with_tag(34, |w| w.write_message(s))?; }
        w.write_with_tag(42, |w| w.write_message(&self.agent_id))?;
        w.write_with_tag(48, |w| w.write_enum(*&self.state as i32))?;
        for s in &self.resources { w.write_with_tag(58, |w| w.write_message(s))?; }
        for s in &self.statuses { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.status_update_state { w.write_with_tag(72, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.status_update_uuid { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.labels { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.discovery { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.container { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.user { w.write_with_tag(114, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TaskResourceLimitation<'a> {
    pub resources: Vec<mesos::v1::Resource<'a>>,
}

impl<'a> MessageRead<'a> for TaskResourceLimitation<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TaskResourceLimitation<'a> {
    fn get_size(&self) -> usize {
        0
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.resources { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UUID<'a> {
    pub value: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for UUID<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UUID<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_bytes(&**&self.value))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Operation<'a> {
    pub framework_id: Option<mesos::v1::FrameworkID<'a>>,
    pub agent_id: Option<mesos::v1::AgentID<'a>>,
    pub info: mesos::v1::mod_Offer::Operation<'a>,
    pub latest_status: mesos::v1::OperationStatus<'a>,
    pub statuses: Vec<mesos::v1::OperationStatus<'a>>,
    pub uuid: mesos::v1::UUID<'a>,
}

impl<'a> MessageRead<'a> for Operation<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.framework_id = Some(r.read_message::<mesos::v1::FrameworkID>(bytes)?),
                Ok(18) => msg.agent_id = Some(r.read_message::<mesos::v1::AgentID>(bytes)?),
                Ok(26) => msg.info = r.read_message::<mesos::v1::mod_Offer::Operation>(bytes)?,
                Ok(34) => msg.latest_status = r.read_message::<mesos::v1::OperationStatus>(bytes)?,
                Ok(42) => msg.statuses.push(r.read_message::<mesos::v1::OperationStatus>(bytes)?),
                Ok(50) => msg.uuid = r.read_message::<mesos::v1::UUID>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Operation<'a> {
    fn get_size(&self) -> usize {
        0
        + self.framework_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.agent_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + 1 + sizeof_len((&self.info).get_size())
        + 1 + sizeof_len((&self.latest_status).get_size())
        + self.statuses.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + 1 + sizeof_len((&self.uuid).get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.framework_id { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.agent_id { w.write_with_tag(18, |w| w.write_message(s))?; }
        w.write_with_tag(26, |w| w.write_message(&self.info))?;
        w.write_with_tag(34, |w| w.write_message(&self.latest_status))?;
        for s in &self.statuses { w.write_with_tag(42, |w| w.write_message(s))?; }
        w.write_with_tag(50, |w| w.write_message(&self.uuid))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationStatus<'a> {
    pub operation_id: Option<mesos::v1::OperationID<'a>>,
    pub state: mesos::v1::OperationState,
    pub message: Option<Cow<'a, str>>,
    pub converted_resources: Vec<mesos::v1::Resource<'a>>,
    pub uuid: Option<mesos::v1::UUID<'a>>,
}

impl<'a> MessageRead<'a> for OperationStatus<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.operation_id = Some(r.read_message::<mesos::v1::OperationID>(bytes)?),
                Ok(16) => msg.state = r.read_enum(bytes)?,
                Ok(26) => msg.message = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.converted_resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(42) => msg.uuid = Some(r.read_message::<mesos::v1::UUID>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OperationStatus<'a> {
    fn get_size(&self) -> usize {
        0
        + self.operation_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + 1 + sizeof_varint(*(&self.state) as u64)
        + self.message.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.converted_resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.uuid.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.operation_id { w.write_with_tag(10, |w| w.write_message(s))?; }
        w.write_with_tag(16, |w| w.write_enum(*&self.state as i32))?;
        if let Some(ref s) = self.message { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        for s in &self.converted_resources { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.uuid { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CheckStatusInfo {
    pub type_pb: Option<mesos::v1::mod_CheckInfo::Type>,
    pub command: Option<mesos::v1::mod_CheckStatusInfo::Command>,
    pub http: Option<mesos::v1::mod_CheckStatusInfo::Http>,
    pub tcp: Option<mesos::v1::mod_CheckStatusInfo::Tcp>,
}

impl<'a> MessageRead<'a> for CheckStatusInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(18) => msg.command = Some(r.read_message::<mesos::v1::mod_CheckStatusInfo::Command>(bytes)?),
                Ok(26) => msg.http = Some(r.read_message::<mesos::v1::mod_CheckStatusInfo::Http>(bytes)?),
                Ok(34) => msg.tcp = Some(r.read_message::<mesos::v1::mod_CheckStatusInfo::Tcp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CheckStatusInfo {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.command.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.http.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.tcp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.command { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.http { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.tcp { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_CheckStatusInfo {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Command {
    pub exit_code: Option<i32>,
}

impl<'a> MessageRead<'a> for Command {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.exit_code = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Command {
    fn get_size(&self) -> usize {
        0
        + self.exit_code.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.exit_code { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Http {
    pub status_code: Option<u32>,
}

impl<'a> MessageRead<'a> for Http {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status_code = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Http {
    fn get_size(&self) -> usize {
        0
        + self.status_code.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.status_code { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Tcp {
    pub succeeded: Option<bool>,
}

impl<'a> MessageRead<'a> for Tcp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.succeeded = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Tcp {
    fn get_size(&self) -> usize {
        0
        + self.succeeded.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.succeeded { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TaskStatus<'a> {
    pub task_id: mesos::v1::TaskID<'a>,
    pub state: mesos::v1::TaskState,
    pub message: Option<Cow<'a, str>>,
    pub source: Option<mesos::v1::mod_TaskStatus::Source>,
    pub reason: Option<mesos::v1::mod_TaskStatus::Reason>,
    pub data: Option<Cow<'a, [u8]>>,
    pub agent_id: Option<mesos::v1::AgentID<'a>>,
    pub executor_id: Option<mesos::v1::ExecutorID<'a>>,
    pub timestamp: Option<f64>,
    pub uuid: Option<Cow<'a, [u8]>>,
    pub healthy: Option<bool>,
    pub check_status: Option<mesos::v1::CheckStatusInfo>,
    pub labels: Option<mesos::v1::Labels<'a>>,
    pub container_status: Option<mesos::v1::ContainerStatus<'a>>,
    pub unreachable_time: Option<mesos::v1::TimeInfo>,
    pub limitation: Option<mesos::v1::TaskResourceLimitation<'a>>,
}

impl<'a> MessageRead<'a> for TaskStatus<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.task_id = r.read_message::<mesos::v1::TaskID>(bytes)?,
                Ok(16) => msg.state = r.read_enum(bytes)?,
                Ok(34) => msg.message = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.source = Some(r.read_enum(bytes)?),
                Ok(80) => msg.reason = Some(r.read_enum(bytes)?),
                Ok(26) => msg.data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.agent_id = Some(r.read_message::<mesos::v1::AgentID>(bytes)?),
                Ok(58) => msg.executor_id = Some(r.read_message::<mesos::v1::ExecutorID>(bytes)?),
                Ok(49) => msg.timestamp = Some(r.read_double(bytes)?),
                Ok(90) => msg.uuid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(64) => msg.healthy = Some(r.read_bool(bytes)?),
                Ok(122) => msg.check_status = Some(r.read_message::<mesos::v1::CheckStatusInfo>(bytes)?),
                Ok(98) => msg.labels = Some(r.read_message::<mesos::v1::Labels>(bytes)?),
                Ok(106) => msg.container_status = Some(r.read_message::<mesos::v1::ContainerStatus>(bytes)?),
                Ok(114) => msg.unreachable_time = Some(r.read_message::<mesos::v1::TimeInfo>(bytes)?),
                Ok(130) => msg.limitation = Some(r.read_message::<mesos::v1::TaskResourceLimitation>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TaskStatus<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.task_id).get_size())
        + 1 + sizeof_varint(*(&self.state) as u64)
        + self.message.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.source.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reason.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.agent_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.executor_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.timestamp.as_ref().map_or(0, |_| 1 + 8)
        + self.uuid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.healthy.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.check_status.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.labels.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.container_status.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.unreachable_time.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.limitation.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.task_id))?;
        w.write_with_tag(16, |w| w.write_enum(*&self.state as i32))?;
        if let Some(ref s) = self.message { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.source { w.write_with_tag(72, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.reason { w.write_with_tag(80, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.data { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.agent_id { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.executor_id { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.timestamp { w.write_with_tag(49, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.uuid { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.healthy { w.write_with_tag(64, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.check_status { w.write_with_tag(122, |w| w.write_message(s))?; }
        if let Some(ref s) = self.labels { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.container_status { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unreachable_time { w.write_with_tag(114, |w| w.write_message(s))?; }
        if let Some(ref s) = self.limitation { w.write_with_tag(130, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_TaskStatus {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Source {
    SOURCE_MASTER = 0,
    SOURCE_AGENT = 1,
    SOURCE_EXECUTOR = 2,
}

impl Default for Source {
    fn default() -> Self {
        Source::SOURCE_MASTER
    }
}

impl From<i32> for Source {
    fn from(i: i32) -> Self {
        match i {
            0 => Source::SOURCE_MASTER,
            1 => Source::SOURCE_AGENT,
            2 => Source::SOURCE_EXECUTOR,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Source {
    fn from(s: &'a str) -> Self {
        match s {
            "SOURCE_MASTER" => Source::SOURCE_MASTER,
            "SOURCE_AGENT" => Source::SOURCE_AGENT,
            "SOURCE_EXECUTOR" => Source::SOURCE_EXECUTOR,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Reason {
    REASON_COMMAND_EXECUTOR_FAILED = 0,
    REASON_CONTAINER_LAUNCH_FAILED = 21,
    REASON_CONTAINER_LIMITATION = 19,
    REASON_CONTAINER_LIMITATION_DISK = 20,
    REASON_CONTAINER_LIMITATION_MEMORY = 8,
    REASON_CONTAINER_PREEMPTED = 17,
    REASON_CONTAINER_UPDATE_FAILED = 22,
    REASON_MAX_COMPLETION_TIME_REACHED = 33,
    REASON_EXECUTOR_REGISTRATION_TIMEOUT = 23,
    REASON_EXECUTOR_REREGISTRATION_TIMEOUT = 24,
    REASON_EXECUTOR_TERMINATED = 1,
    REASON_EXECUTOR_UNREGISTERED = 2,
    REASON_FRAMEWORK_REMOVED = 3,
    REASON_GC_ERROR = 4,
    REASON_INVALID_FRAMEWORKID = 5,
    REASON_INVALID_OFFERS = 6,
    REASON_IO_SWITCHBOARD_EXITED = 27,
    REASON_MASTER_DISCONNECTED = 7,
    REASON_RECONCILIATION = 9,
    REASON_RESOURCES_UNKNOWN = 18,
    REASON_AGENT_DISCONNECTED = 10,
    REASON_AGENT_REMOVED = 11,
    REASON_AGENT_REMOVED_BY_OPERATOR = 31,
    REASON_AGENT_REREGISTERED = 32,
    REASON_AGENT_RESTARTED = 12,
    REASON_AGENT_UNKNOWN = 13,
    REASON_TASK_KILLED_DURING_LAUNCH = 30,
    REASON_TASK_CHECK_STATUS_UPDATED = 28,
    REASON_TASK_HEALTH_CHECK_STATUS_UPDATED = 29,
    REASON_TASK_GROUP_INVALID = 25,
    REASON_TASK_GROUP_UNAUTHORIZED = 26,
    REASON_TASK_INVALID = 14,
    REASON_TASK_UNAUTHORIZED = 15,
    REASON_TASK_UNKNOWN = 16,
}

impl Default for Reason {
    fn default() -> Self {
        Reason::REASON_COMMAND_EXECUTOR_FAILED
    }
}

impl From<i32> for Reason {
    fn from(i: i32) -> Self {
        match i {
            0 => Reason::REASON_COMMAND_EXECUTOR_FAILED,
            21 => Reason::REASON_CONTAINER_LAUNCH_FAILED,
            19 => Reason::REASON_CONTAINER_LIMITATION,
            20 => Reason::REASON_CONTAINER_LIMITATION_DISK,
            8 => Reason::REASON_CONTAINER_LIMITATION_MEMORY,
            17 => Reason::REASON_CONTAINER_PREEMPTED,
            22 => Reason::REASON_CONTAINER_UPDATE_FAILED,
            33 => Reason::REASON_MAX_COMPLETION_TIME_REACHED,
            23 => Reason::REASON_EXECUTOR_REGISTRATION_TIMEOUT,
            24 => Reason::REASON_EXECUTOR_REREGISTRATION_TIMEOUT,
            1 => Reason::REASON_EXECUTOR_TERMINATED,
            2 => Reason::REASON_EXECUTOR_UNREGISTERED,
            3 => Reason::REASON_FRAMEWORK_REMOVED,
            4 => Reason::REASON_GC_ERROR,
            5 => Reason::REASON_INVALID_FRAMEWORKID,
            6 => Reason::REASON_INVALID_OFFERS,
            27 => Reason::REASON_IO_SWITCHBOARD_EXITED,
            7 => Reason::REASON_MASTER_DISCONNECTED,
            9 => Reason::REASON_RECONCILIATION,
            18 => Reason::REASON_RESOURCES_UNKNOWN,
            10 => Reason::REASON_AGENT_DISCONNECTED,
            11 => Reason::REASON_AGENT_REMOVED,
            31 => Reason::REASON_AGENT_REMOVED_BY_OPERATOR,
            32 => Reason::REASON_AGENT_REREGISTERED,
            12 => Reason::REASON_AGENT_RESTARTED,
            13 => Reason::REASON_AGENT_UNKNOWN,
            30 => Reason::REASON_TASK_KILLED_DURING_LAUNCH,
            28 => Reason::REASON_TASK_CHECK_STATUS_UPDATED,
            29 => Reason::REASON_TASK_HEALTH_CHECK_STATUS_UPDATED,
            25 => Reason::REASON_TASK_GROUP_INVALID,
            26 => Reason::REASON_TASK_GROUP_UNAUTHORIZED,
            14 => Reason::REASON_TASK_INVALID,
            15 => Reason::REASON_TASK_UNAUTHORIZED,
            16 => Reason::REASON_TASK_UNKNOWN,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Reason {
    fn from(s: &'a str) -> Self {
        match s {
            "REASON_COMMAND_EXECUTOR_FAILED" => Reason::REASON_COMMAND_EXECUTOR_FAILED,
            "REASON_CONTAINER_LAUNCH_FAILED" => Reason::REASON_CONTAINER_LAUNCH_FAILED,
            "REASON_CONTAINER_LIMITATION" => Reason::REASON_CONTAINER_LIMITATION,
            "REASON_CONTAINER_LIMITATION_DISK" => Reason::REASON_CONTAINER_LIMITATION_DISK,
            "REASON_CONTAINER_LIMITATION_MEMORY" => Reason::REASON_CONTAINER_LIMITATION_MEMORY,
            "REASON_CONTAINER_PREEMPTED" => Reason::REASON_CONTAINER_PREEMPTED,
            "REASON_CONTAINER_UPDATE_FAILED" => Reason::REASON_CONTAINER_UPDATE_FAILED,
            "REASON_MAX_COMPLETION_TIME_REACHED" => Reason::REASON_MAX_COMPLETION_TIME_REACHED,
            "REASON_EXECUTOR_REGISTRATION_TIMEOUT" => Reason::REASON_EXECUTOR_REGISTRATION_TIMEOUT,
            "REASON_EXECUTOR_REREGISTRATION_TIMEOUT" => Reason::REASON_EXECUTOR_REREGISTRATION_TIMEOUT,
            "REASON_EXECUTOR_TERMINATED" => Reason::REASON_EXECUTOR_TERMINATED,
            "REASON_EXECUTOR_UNREGISTERED" => Reason::REASON_EXECUTOR_UNREGISTERED,
            "REASON_FRAMEWORK_REMOVED" => Reason::REASON_FRAMEWORK_REMOVED,
            "REASON_GC_ERROR" => Reason::REASON_GC_ERROR,
            "REASON_INVALID_FRAMEWORKID" => Reason::REASON_INVALID_FRAMEWORKID,
            "REASON_INVALID_OFFERS" => Reason::REASON_INVALID_OFFERS,
            "REASON_IO_SWITCHBOARD_EXITED" => Reason::REASON_IO_SWITCHBOARD_EXITED,
            "REASON_MASTER_DISCONNECTED" => Reason::REASON_MASTER_DISCONNECTED,
            "REASON_RECONCILIATION" => Reason::REASON_RECONCILIATION,
            "REASON_RESOURCES_UNKNOWN" => Reason::REASON_RESOURCES_UNKNOWN,
            "REASON_AGENT_DISCONNECTED" => Reason::REASON_AGENT_DISCONNECTED,
            "REASON_AGENT_REMOVED" => Reason::REASON_AGENT_REMOVED,
            "REASON_AGENT_REMOVED_BY_OPERATOR" => Reason::REASON_AGENT_REMOVED_BY_OPERATOR,
            "REASON_AGENT_REREGISTERED" => Reason::REASON_AGENT_REREGISTERED,
            "REASON_AGENT_RESTARTED" => Reason::REASON_AGENT_RESTARTED,
            "REASON_AGENT_UNKNOWN" => Reason::REASON_AGENT_UNKNOWN,
            "REASON_TASK_KILLED_DURING_LAUNCH" => Reason::REASON_TASK_KILLED_DURING_LAUNCH,
            "REASON_TASK_CHECK_STATUS_UPDATED" => Reason::REASON_TASK_CHECK_STATUS_UPDATED,
            "REASON_TASK_HEALTH_CHECK_STATUS_UPDATED" => Reason::REASON_TASK_HEALTH_CHECK_STATUS_UPDATED,
            "REASON_TASK_GROUP_INVALID" => Reason::REASON_TASK_GROUP_INVALID,
            "REASON_TASK_GROUP_UNAUTHORIZED" => Reason::REASON_TASK_GROUP_UNAUTHORIZED,
            "REASON_TASK_INVALID" => Reason::REASON_TASK_INVALID,
            "REASON_TASK_UNAUTHORIZED" => Reason::REASON_TASK_UNAUTHORIZED,
            "REASON_TASK_UNKNOWN" => Reason::REASON_TASK_UNKNOWN,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Filters {
    pub refuse_seconds: f64,
}

impl<'a> MessageRead<'a> for Filters {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Filters {
            refuse_seconds: 5.0f64,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.refuse_seconds = r.read_double(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Filters {
    fn get_size(&self) -> usize {
        0
        + if self.refuse_seconds == 5.0f64 { 0 } else { 1 + 8 }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.refuse_seconds != 5.0f64 { w.write_with_tag(9, |w| w.write_double(*&self.refuse_seconds))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Environment<'a> {
    pub variables: Vec<mesos::v1::mod_Environment::Variable<'a>>,
}

impl<'a> MessageRead<'a> for Environment<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.variables.push(r.read_message::<mesos::v1::mod_Environment::Variable>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Environment<'a> {
    fn get_size(&self) -> usize {
        0
        + self.variables.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.variables { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Environment {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Variable<'a> {
    pub name: Cow<'a, str>,
    pub type_pb: mesos::v1::mod_Environment::mod_Variable::Type,
    pub value: Option<Cow<'a, str>>,
    pub secret: Option<mesos::v1::Secret<'a>>,
}

impl<'a> MessageRead<'a> for Variable<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Variable {
            type_pb: mesos::v1::mod_Environment::mod_Variable::Type::VALUE,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.type_pb = r.read_enum(bytes)?,
                Ok(18) => msg.value = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.secret = Some(r.read_message::<mesos::v1::Secret>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Variable<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + if self.type_pb == mesos::v1::mod_Environment::mod_Variable::Type::VALUE { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.secret.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        if self.type_pb != mesos::v1::mod_Environment::mod_Variable::Type::VALUE { w.write_with_tag(24, |w| w.write_enum(*&self.type_pb as i32))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.secret { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Variable {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    VALUE = 1,
    SECRET = 2,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::VALUE,
            2 => Type::SECRET,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "VALUE" => Type::VALUE,
            "SECRET" => Type::SECRET,
            _ => Self::default(),
        }
    }
}

}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Parameter<'a> {
    pub key: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Parameter<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Parameter<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.key).len())
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.key))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Parameters<'a> {
    pub parameter: Vec<mesos::v1::Parameter<'a>>,
}

impl<'a> MessageRead<'a> for Parameters<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.parameter.push(r.read_message::<mesos::v1::Parameter>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Parameters<'a> {
    fn get_size(&self) -> usize {
        0
        + self.parameter.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.parameter { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Credential<'a> {
    pub principal: Cow<'a, str>,
    pub secret: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Credential<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.principal = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.secret = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Credential<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.principal).len())
        + self.secret.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.principal))?;
        if let Some(ref s) = self.secret { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Credentials<'a> {
    pub credentials: Vec<mesos::v1::Credential<'a>>,
}

impl<'a> MessageRead<'a> for Credentials<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.credentials.push(r.read_message::<mesos::v1::Credential>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Credentials<'a> {
    fn get_size(&self) -> usize {
        0
        + self.credentials.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.credentials { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Secret<'a> {
    pub type_pb: Option<mesos::v1::mod_Secret::Type>,
    pub reference: Option<mesos::v1::mod_Secret::Reference<'a>>,
    pub value: Option<mesos::v1::Value<'a>>,
}

impl<'a> MessageRead<'a> for Secret<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(18) => msg.reference = Some(r.read_message::<mesos::v1::mod_Secret::Reference>(bytes)?),
                Ok(26) => msg.value = Some(r.read_message::<mesos::v1::Value>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Secret<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reference.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.reference { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Secret {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Reference<'a> {
    pub name: Cow<'a, str>,
    pub key: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Reference<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.key = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Reference<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + self.key.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        if let Some(ref s) = self.key { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Value<'a> {
    pub data: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for Value<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.data = r.read_bytes(bytes).map(Cow::Borrowed)?,
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
        + 1 + sizeof_len((&self.data).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_bytes(&**&self.data))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    REFERENCE = 1,
    VALUE = 2,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::REFERENCE,
            2 => Type::VALUE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "REFERENCE" => Type::REFERENCE,
            "VALUE" => Type::VALUE,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RateLimit<'a> {
    pub qps: Option<f64>,
    pub principal: Cow<'a, str>,
    pub capacity: Option<u64>,
}

impl<'a> MessageRead<'a> for RateLimit<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.qps = Some(r.read_double(bytes)?),
                Ok(18) => msg.principal = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.capacity = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RateLimit<'a> {
    fn get_size(&self) -> usize {
        0
        + self.qps.as_ref().map_or(0, |_| 1 + 8)
        + 1 + sizeof_len((&self.principal).len())
        + self.capacity.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.qps { w.write_with_tag(9, |w| w.write_double(*s))?; }
        w.write_with_tag(18, |w| w.write_string(&**&self.principal))?;
        if let Some(ref s) = self.capacity { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RateLimits<'a> {
    pub limits: Vec<mesos::v1::RateLimit<'a>>,
    pub aggregate_default_qps: Option<f64>,
    pub aggregate_default_capacity: Option<u64>,
}

impl<'a> MessageRead<'a> for RateLimits<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.limits.push(r.read_message::<mesos::v1::RateLimit>(bytes)?),
                Ok(17) => msg.aggregate_default_qps = Some(r.read_double(bytes)?),
                Ok(24) => msg.aggregate_default_capacity = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RateLimits<'a> {
    fn get_size(&self) -> usize {
        0
        + self.limits.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.aggregate_default_qps.as_ref().map_or(0, |_| 1 + 8)
        + self.aggregate_default_capacity.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.limits { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.aggregate_default_qps { w.write_with_tag(17, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.aggregate_default_capacity { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Image<'a> {
    pub type_pb: mesos::v1::mod_Image::Type,
    pub appc: Option<mesos::v1::mod_Image::Appc<'a>>,
    pub docker: Option<mesos::v1::mod_Image::Docker<'a>>,
    pub cached: bool,
}

impl<'a> MessageRead<'a> for Image<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Image {
            cached: true,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(18) => msg.appc = Some(r.read_message::<mesos::v1::mod_Image::Appc>(bytes)?),
                Ok(26) => msg.docker = Some(r.read_message::<mesos::v1::mod_Image::Docker>(bytes)?),
                Ok(32) => msg.cached = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Image<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.type_pb) as u64)
        + self.appc.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.docker.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.cached == true { 0 } else { 1 + sizeof_varint(*(&self.cached) as u64) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?;
        if let Some(ref s) = self.appc { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.docker { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.cached != true { w.write_with_tag(32, |w| w.write_bool(*&self.cached))?; }
        Ok(())
    }
}

pub mod mod_Image {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Appc<'a> {
    pub name: Cow<'a, str>,
    pub id: Option<Cow<'a, str>>,
    pub labels: Option<mesos::v1::Labels<'a>>,
}

impl<'a> MessageRead<'a> for Appc<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.labels = Some(r.read_message::<mesos::v1::Labels>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Appc<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.labels.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        if let Some(ref s) = self.id { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.labels { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Docker<'a> {
    pub name: Cow<'a, str>,
    pub config: Option<mesos::v1::Secret<'a>>,
}

impl<'a> MessageRead<'a> for Docker<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.config = Some(r.read_message::<mesos::v1::Secret>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Docker<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + self.config.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        if let Some(ref s) = self.config { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    APPC = 1,
    DOCKER = 2,
}

impl Default for Type {
    fn default() -> Self {
        Type::APPC
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            1 => Type::APPC,
            2 => Type::DOCKER,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "APPC" => Type::APPC,
            "DOCKER" => Type::DOCKER,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MountPropagation {
    pub mode: Option<mesos::v1::mod_MountPropagation::Mode>,
}

impl<'a> MessageRead<'a> for MountPropagation {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.mode = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MountPropagation {
    fn get_size(&self) -> usize {
        0
        + self.mode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.mode { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

pub mod mod_MountPropagation {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    UNKNOWN = 0,
    HOST_TO_CONTAINER = 1,
    BIDIRECTIONAL = 2,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::UNKNOWN
    }
}

impl From<i32> for Mode {
    fn from(i: i32) -> Self {
        match i {
            0 => Mode::UNKNOWN,
            1 => Mode::HOST_TO_CONTAINER,
            2 => Mode::BIDIRECTIONAL,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Mode {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Mode::UNKNOWN,
            "HOST_TO_CONTAINER" => Mode::HOST_TO_CONTAINER,
            "BIDIRECTIONAL" => Mode::BIDIRECTIONAL,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Volume<'a> {
    pub mode: mesos::v1::mod_Volume::Mode,
    pub container_path: Cow<'a, str>,
    pub host_path: Option<Cow<'a, str>>,
    pub image: Option<mesos::v1::Image<'a>>,
    pub source: Option<mesos::v1::mod_Volume::Source<'a>>,
}

impl<'a> MessageRead<'a> for Volume<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(24) => msg.mode = r.read_enum(bytes)?,
                Ok(10) => msg.container_path = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.host_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.image = Some(r.read_message::<mesos::v1::Image>(bytes)?),
                Ok(42) => msg.source = Some(r.read_message::<mesos::v1::mod_Volume::Source>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Volume<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.mode) as u64)
        + 1 + sizeof_len((&self.container_path).len())
        + self.host_path.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.image.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.source.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(24, |w| w.write_enum(*&self.mode as i32))?;
        w.write_with_tag(10, |w| w.write_string(&**&self.container_path))?;
        if let Some(ref s) = self.host_path { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.image { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.source { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Volume {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Source<'a> {
    pub type_pb: Option<mesos::v1::mod_Volume::mod_Source::Type>,
    pub docker_volume: Option<mesos::v1::mod_Volume::mod_Source::DockerVolume<'a>>,
    pub host_path: Option<mesos::v1::mod_Volume::mod_Source::HostPath<'a>>,
    pub sandbox_path: Option<mesos::v1::mod_Volume::mod_Source::SandboxPath<'a>>,
    pub secret: Option<mesos::v1::Secret<'a>>,
}

impl<'a> MessageRead<'a> for Source<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(18) => msg.docker_volume = Some(r.read_message::<mesos::v1::mod_Volume::mod_Source::DockerVolume>(bytes)?),
                Ok(42) => msg.host_path = Some(r.read_message::<mesos::v1::mod_Volume::mod_Source::HostPath>(bytes)?),
                Ok(26) => msg.sandbox_path = Some(r.read_message::<mesos::v1::mod_Volume::mod_Source::SandboxPath>(bytes)?),
                Ok(34) => msg.secret = Some(r.read_message::<mesos::v1::Secret>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Source<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.docker_volume.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.host_path.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.sandbox_path.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.secret.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.docker_volume { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.host_path { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.sandbox_path { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.secret { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Source {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DockerVolume<'a> {
    pub driver: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
    pub driver_options: Option<mesos::v1::Parameters<'a>>,
}

impl<'a> MessageRead<'a> for DockerVolume<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.driver = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.driver_options = Some(r.read_message::<mesos::v1::Parameters>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DockerVolume<'a> {
    fn get_size(&self) -> usize {
        0
        + self.driver.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + 1 + sizeof_len((&self.name).len())
        + self.driver_options.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.driver { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        w.write_with_tag(18, |w| w.write_string(&**&self.name))?;
        if let Some(ref s) = self.driver_options { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HostPath<'a> {
    pub path: Cow<'a, str>,
    pub mount_propagation: Option<mesos::v1::MountPropagation>,
}

impl<'a> MessageRead<'a> for HostPath<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.path = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.mount_propagation = Some(r.read_message::<mesos::v1::MountPropagation>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HostPath<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.path).len())
        + self.mount_propagation.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.path))?;
        if let Some(ref s) = self.mount_propagation { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SandboxPath<'a> {
    pub type_pb: Option<mesos::v1::mod_Volume::mod_Source::Type>,
    pub path: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SandboxPath<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(18) => msg.path = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SandboxPath<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + 1 + sizeof_len((&self.path).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        w.write_with_tag(18, |w| w.write_string(&**&self.path))?;
        Ok(())
    }
}

pub mod mod_SandboxPath {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    SELF = 1,
    PARENT = 2,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::SELF,
            2 => Type::PARENT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "SELF" => Type::SELF,
            "PARENT" => Type::PARENT,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    DOCKER_VOLUME = 1,
    HOST_PATH = 4,
    SANDBOX_PATH = 2,
    SECRET = 3,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::DOCKER_VOLUME,
            4 => Type::HOST_PATH,
            2 => Type::SANDBOX_PATH,
            3 => Type::SECRET,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "DOCKER_VOLUME" => Type::DOCKER_VOLUME,
            "HOST_PATH" => Type::HOST_PATH,
            "SANDBOX_PATH" => Type::SANDBOX_PATH,
            "SECRET" => Type::SECRET,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    RW = 1,
    RO = 2,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::RW
    }
}

impl From<i32> for Mode {
    fn from(i: i32) -> Self {
        match i {
            1 => Mode::RW,
            2 => Mode::RO,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Mode {
    fn from(s: &'a str) -> Self {
        match s {
            "RW" => Mode::RW,
            "RO" => Mode::RO,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NetworkInfo<'a> {
    pub ip_addresses: Vec<mesos::v1::mod_NetworkInfo::IPAddress<'a>>,
    pub name: Option<Cow<'a, str>>,
    pub groups: Vec<Cow<'a, str>>,
    pub labels: Option<mesos::v1::Labels<'a>>,
    pub port_mappings: Vec<mesos::v1::mod_NetworkInfo::PortMapping<'a>>,
}

impl<'a> MessageRead<'a> for NetworkInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(42) => msg.ip_addresses.push(r.read_message::<mesos::v1::mod_NetworkInfo::IPAddress>(bytes)?),
                Ok(50) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.groups.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.labels = Some(r.read_message::<mesos::v1::Labels>(bytes)?),
                Ok(58) => msg.port_mappings.push(r.read_message::<mesos::v1::mod_NetworkInfo::PortMapping>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NetworkInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ip_addresses.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.groups.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.labels.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.port_mappings.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ip_addresses { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        for s in &self.groups { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.labels { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.port_mappings { w.write_with_tag(58, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_NetworkInfo {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct IPAddress<'a> {
    pub protocol: mesos::v1::mod_NetworkInfo::Protocol,
    pub ip_address: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for IPAddress<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.protocol = r.read_enum(bytes)?,
                Ok(18) => msg.ip_address = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for IPAddress<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.protocol == mesos::v1::mod_NetworkInfo::Protocol::IPv4 { 0 } else { 1 + sizeof_varint(*(&self.protocol) as u64) }
        + self.ip_address.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.protocol != mesos::v1::mod_NetworkInfo::Protocol::IPv4 { w.write_with_tag(8, |w| w.write_enum(*&self.protocol as i32))?; }
        if let Some(ref s) = self.ip_address { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PortMapping<'a> {
    pub host_port: u32,
    pub container_port: u32,
    pub protocol: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for PortMapping<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.host_port = r.read_uint32(bytes)?,
                Ok(16) => msg.container_port = r.read_uint32(bytes)?,
                Ok(26) => msg.protocol = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PortMapping<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.host_port) as u64)
        + 1 + sizeof_varint(*(&self.container_port) as u64)
        + self.protocol.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.host_port))?;
        w.write_with_tag(16, |w| w.write_uint32(*&self.container_port))?;
        if let Some(ref s) = self.protocol { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Protocol {
    IPv4 = 1,
    IPv6 = 2,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::IPv4
    }
}

impl From<i32> for Protocol {
    fn from(i: i32) -> Self {
        match i {
            1 => Protocol::IPv4,
            2 => Protocol::IPv6,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Protocol {
    fn from(s: &'a str) -> Self {
        match s {
            "IPv4" => Protocol::IPv4,
            "IPv6" => Protocol::IPv6,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CapabilityInfo {
    pub capabilities: Vec<mesos::v1::mod_CapabilityInfo::Capability>,
}

impl<'a> MessageRead<'a> for CapabilityInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.capabilities.push(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CapabilityInfo {
    fn get_size(&self) -> usize {
        0
        + self.capabilities.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.capabilities { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

pub mod mod_CapabilityInfo {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Capability {
    UNKNOWN = 0,
    CHOWN = 1000,
    DAC_OVERRIDE = 1001,
    DAC_READ_SEARCH = 1002,
    FOWNER = 1003,
    FSETID = 1004,
    KILL = 1005,
    SETGID = 1006,
    SETUID = 1007,
    SETPCAP = 1008,
    LINUX_IMMUTABLE = 1009,
    NET_BIND_SERVICE = 1010,
    NET_BROADCAST = 1011,
    NET_ADMIN = 1012,
    NET_RAW = 1013,
    IPC_LOCK = 1014,
    IPC_OWNER = 1015,
    SYS_MODULE = 1016,
    SYS_RAWIO = 1017,
    SYS_CHROOT = 1018,
    SYS_PTRACE = 1019,
    SYS_PACCT = 1020,
    SYS_ADMIN = 1021,
    SYS_BOOT = 1022,
    SYS_NICE = 1023,
    SYS_RESOURCE = 1024,
    SYS_TIME = 1025,
    SYS_TTY_CONFIG = 1026,
    MKNOD = 1027,
    LEASE = 1028,
    AUDIT_WRITE = 1029,
    AUDIT_CONTROL = 1030,
    SETFCAP = 1031,
    MAC_OVERRIDE = 1032,
    MAC_ADMIN = 1033,
    SYSLOG = 1034,
    WAKE_ALARM = 1035,
    BLOCK_SUSPEND = 1036,
    AUDIT_READ = 1037,
}

impl Default for Capability {
    fn default() -> Self {
        Capability::UNKNOWN
    }
}

impl From<i32> for Capability {
    fn from(i: i32) -> Self {
        match i {
            0 => Capability::UNKNOWN,
            1000 => Capability::CHOWN,
            1001 => Capability::DAC_OVERRIDE,
            1002 => Capability::DAC_READ_SEARCH,
            1003 => Capability::FOWNER,
            1004 => Capability::FSETID,
            1005 => Capability::KILL,
            1006 => Capability::SETGID,
            1007 => Capability::SETUID,
            1008 => Capability::SETPCAP,
            1009 => Capability::LINUX_IMMUTABLE,
            1010 => Capability::NET_BIND_SERVICE,
            1011 => Capability::NET_BROADCAST,
            1012 => Capability::NET_ADMIN,
            1013 => Capability::NET_RAW,
            1014 => Capability::IPC_LOCK,
            1015 => Capability::IPC_OWNER,
            1016 => Capability::SYS_MODULE,
            1017 => Capability::SYS_RAWIO,
            1018 => Capability::SYS_CHROOT,
            1019 => Capability::SYS_PTRACE,
            1020 => Capability::SYS_PACCT,
            1021 => Capability::SYS_ADMIN,
            1022 => Capability::SYS_BOOT,
            1023 => Capability::SYS_NICE,
            1024 => Capability::SYS_RESOURCE,
            1025 => Capability::SYS_TIME,
            1026 => Capability::SYS_TTY_CONFIG,
            1027 => Capability::MKNOD,
            1028 => Capability::LEASE,
            1029 => Capability::AUDIT_WRITE,
            1030 => Capability::AUDIT_CONTROL,
            1031 => Capability::SETFCAP,
            1032 => Capability::MAC_OVERRIDE,
            1033 => Capability::MAC_ADMIN,
            1034 => Capability::SYSLOG,
            1035 => Capability::WAKE_ALARM,
            1036 => Capability::BLOCK_SUSPEND,
            1037 => Capability::AUDIT_READ,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Capability {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Capability::UNKNOWN,
            "CHOWN" => Capability::CHOWN,
            "DAC_OVERRIDE" => Capability::DAC_OVERRIDE,
            "DAC_READ_SEARCH" => Capability::DAC_READ_SEARCH,
            "FOWNER" => Capability::FOWNER,
            "FSETID" => Capability::FSETID,
            "KILL" => Capability::KILL,
            "SETGID" => Capability::SETGID,
            "SETUID" => Capability::SETUID,
            "SETPCAP" => Capability::SETPCAP,
            "LINUX_IMMUTABLE" => Capability::LINUX_IMMUTABLE,
            "NET_BIND_SERVICE" => Capability::NET_BIND_SERVICE,
            "NET_BROADCAST" => Capability::NET_BROADCAST,
            "NET_ADMIN" => Capability::NET_ADMIN,
            "NET_RAW" => Capability::NET_RAW,
            "IPC_LOCK" => Capability::IPC_LOCK,
            "IPC_OWNER" => Capability::IPC_OWNER,
            "SYS_MODULE" => Capability::SYS_MODULE,
            "SYS_RAWIO" => Capability::SYS_RAWIO,
            "SYS_CHROOT" => Capability::SYS_CHROOT,
            "SYS_PTRACE" => Capability::SYS_PTRACE,
            "SYS_PACCT" => Capability::SYS_PACCT,
            "SYS_ADMIN" => Capability::SYS_ADMIN,
            "SYS_BOOT" => Capability::SYS_BOOT,
            "SYS_NICE" => Capability::SYS_NICE,
            "SYS_RESOURCE" => Capability::SYS_RESOURCE,
            "SYS_TIME" => Capability::SYS_TIME,
            "SYS_TTY_CONFIG" => Capability::SYS_TTY_CONFIG,
            "MKNOD" => Capability::MKNOD,
            "LEASE" => Capability::LEASE,
            "AUDIT_WRITE" => Capability::AUDIT_WRITE,
            "AUDIT_CONTROL" => Capability::AUDIT_CONTROL,
            "SETFCAP" => Capability::SETFCAP,
            "MAC_OVERRIDE" => Capability::MAC_OVERRIDE,
            "MAC_ADMIN" => Capability::MAC_ADMIN,
            "SYSLOG" => Capability::SYSLOG,
            "WAKE_ALARM" => Capability::WAKE_ALARM,
            "BLOCK_SUSPEND" => Capability::BLOCK_SUSPEND,
            "AUDIT_READ" => Capability::AUDIT_READ,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LinuxInfo {
    pub bounding_capabilities: Option<mesos::v1::CapabilityInfo>,
    pub effective_capabilities: Option<mesos::v1::CapabilityInfo>,
    pub share_pid_namespace: Option<bool>,
}

impl<'a> MessageRead<'a> for LinuxInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.bounding_capabilities = Some(r.read_message::<mesos::v1::CapabilityInfo>(bytes)?),
                Ok(26) => msg.effective_capabilities = Some(r.read_message::<mesos::v1::CapabilityInfo>(bytes)?),
                Ok(32) => msg.share_pid_namespace = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LinuxInfo {
    fn get_size(&self) -> usize {
        0
        + self.bounding_capabilities.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.effective_capabilities.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.share_pid_namespace.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.bounding_capabilities { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.effective_capabilities { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.share_pid_namespace { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RLimitInfo {
    pub rlimits: Vec<mesos::v1::mod_RLimitInfo::RLimit>,
}

impl<'a> MessageRead<'a> for RLimitInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.rlimits.push(r.read_message::<mesos::v1::mod_RLimitInfo::RLimit>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RLimitInfo {
    fn get_size(&self) -> usize {
        0
        + self.rlimits.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.rlimits { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_RLimitInfo {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RLimit {
    pub type_pb: Option<mesos::v1::mod_RLimitInfo::mod_RLimit::Type>,
    pub hard: Option<u64>,
    pub soft: Option<u64>,
}

impl<'a> MessageRead<'a> for RLimit {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(16) => msg.hard = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.soft = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RLimit {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.hard.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.soft.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.hard { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.soft { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

pub mod mod_RLimit {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    UNKNOWN = 0,
    RLMT_AS = 1,
    RLMT_CORE = 2,
    RLMT_CPU = 3,
    RLMT_DATA = 4,
    RLMT_FSIZE = 5,
    RLMT_LOCKS = 6,
    RLMT_MEMLOCK = 7,
    RLMT_MSGQUEUE = 8,
    RLMT_NICE = 9,
    RLMT_NOFILE = 10,
    RLMT_NPROC = 11,
    RLMT_RSS = 12,
    RLMT_RTPRIO = 13,
    RLMT_RTTIME = 14,
    RLMT_SIGPENDING = 15,
    RLMT_STACK = 16,
}

impl Default for Type {
    fn default() -> Self {
        Type::UNKNOWN
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::UNKNOWN,
            1 => Type::RLMT_AS,
            2 => Type::RLMT_CORE,
            3 => Type::RLMT_CPU,
            4 => Type::RLMT_DATA,
            5 => Type::RLMT_FSIZE,
            6 => Type::RLMT_LOCKS,
            7 => Type::RLMT_MEMLOCK,
            8 => Type::RLMT_MSGQUEUE,
            9 => Type::RLMT_NICE,
            10 => Type::RLMT_NOFILE,
            11 => Type::RLMT_NPROC,
            12 => Type::RLMT_RSS,
            13 => Type::RLMT_RTPRIO,
            14 => Type::RLMT_RTTIME,
            15 => Type::RLMT_SIGPENDING,
            16 => Type::RLMT_STACK,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Type::UNKNOWN,
            "RLMT_AS" => Type::RLMT_AS,
            "RLMT_CORE" => Type::RLMT_CORE,
            "RLMT_CPU" => Type::RLMT_CPU,
            "RLMT_DATA" => Type::RLMT_DATA,
            "RLMT_FSIZE" => Type::RLMT_FSIZE,
            "RLMT_LOCKS" => Type::RLMT_LOCKS,
            "RLMT_MEMLOCK" => Type::RLMT_MEMLOCK,
            "RLMT_MSGQUEUE" => Type::RLMT_MSGQUEUE,
            "RLMT_NICE" => Type::RLMT_NICE,
            "RLMT_NOFILE" => Type::RLMT_NOFILE,
            "RLMT_NPROC" => Type::RLMT_NPROC,
            "RLMT_RSS" => Type::RLMT_RSS,
            "RLMT_RTPRIO" => Type::RLMT_RTPRIO,
            "RLMT_RTTIME" => Type::RLMT_RTTIME,
            "RLMT_SIGPENDING" => Type::RLMT_SIGPENDING,
            "RLMT_STACK" => Type::RLMT_STACK,
            _ => Self::default(),
        }
    }
}

}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TTYInfo {
    pub window_size: Option<mesos::v1::mod_TTYInfo::WindowSize>,
}

impl<'a> MessageRead<'a> for TTYInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.window_size = Some(r.read_message::<mesos::v1::mod_TTYInfo::WindowSize>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TTYInfo {
    fn get_size(&self) -> usize {
        0
        + self.window_size.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.window_size { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_TTYInfo {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WindowSize {
    pub rows: u32,
    pub columns: u32,
}

impl<'a> MessageRead<'a> for WindowSize {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.rows = r.read_uint32(bytes)?,
                Ok(16) => msg.columns = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for WindowSize {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.rows) as u64)
        + 1 + sizeof_varint(*(&self.columns) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.rows))?;
        w.write_with_tag(16, |w| w.write_uint32(*&self.columns))?;
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ContainerInfo<'a> {
    pub type_pb: mesos::v1::mod_ContainerInfo::Type,
    pub volumes: Vec<mesos::v1::Volume<'a>>,
    pub hostname: Option<Cow<'a, str>>,
    pub docker: Option<mesos::v1::mod_ContainerInfo::DockerInfo<'a>>,
    pub mesos: Option<mesos::v1::mod_ContainerInfo::MesosInfo<'a>>,
    pub network_infos: Vec<mesos::v1::NetworkInfo<'a>>,
    pub linux_info: Option<mesos::v1::LinuxInfo>,
    pub rlimit_info: Option<mesos::v1::RLimitInfo>,
    pub tty_info: Option<mesos::v1::TTYInfo>,
}

impl<'a> MessageRead<'a> for ContainerInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(18) => msg.volumes.push(r.read_message::<mesos::v1::Volume>(bytes)?),
                Ok(34) => msg.hostname = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.docker = Some(r.read_message::<mesos::v1::mod_ContainerInfo::DockerInfo>(bytes)?),
                Ok(42) => msg.mesos = Some(r.read_message::<mesos::v1::mod_ContainerInfo::MesosInfo>(bytes)?),
                Ok(58) => msg.network_infos.push(r.read_message::<mesos::v1::NetworkInfo>(bytes)?),
                Ok(66) => msg.linux_info = Some(r.read_message::<mesos::v1::LinuxInfo>(bytes)?),
                Ok(74) => msg.rlimit_info = Some(r.read_message::<mesos::v1::RLimitInfo>(bytes)?),
                Ok(82) => msg.tty_info = Some(r.read_message::<mesos::v1::TTYInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ContainerInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.type_pb) as u64)
        + self.volumes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.hostname.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.docker.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.mesos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.network_infos.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.linux_info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.rlimit_info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.tty_info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?;
        for s in &self.volumes { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.hostname { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.docker { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.mesos { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.network_infos { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.linux_info { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.rlimit_info { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.tty_info { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_ContainerInfo {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DockerInfo<'a> {
    pub image: Cow<'a, str>,
    pub network: mesos::v1::mod_ContainerInfo::mod_DockerInfo::Network,
    pub port_mappings: Vec<mesos::v1::mod_ContainerInfo::mod_DockerInfo::PortMapping<'a>>,
    pub privileged: bool,
    pub parameters: Vec<mesos::v1::Parameter<'a>>,
    pub force_pull_image: Option<bool>,
}

impl<'a> MessageRead<'a> for DockerInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.image = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.network = r.read_enum(bytes)?,
                Ok(26) => msg.port_mappings.push(r.read_message::<mesos::v1::mod_ContainerInfo::mod_DockerInfo::PortMapping>(bytes)?),
                Ok(32) => msg.privileged = r.read_bool(bytes)?,
                Ok(42) => msg.parameters.push(r.read_message::<mesos::v1::Parameter>(bytes)?),
                Ok(48) => msg.force_pull_image = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DockerInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.image).len())
        + if self.network == mesos::v1::mod_ContainerInfo::mod_DockerInfo::Network::HOST { 0 } else { 1 + sizeof_varint(*(&self.network) as u64) }
        + self.port_mappings.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.privileged == false { 0 } else { 1 + sizeof_varint(*(&self.privileged) as u64) }
        + self.parameters.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.force_pull_image.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.image))?;
        if self.network != mesos::v1::mod_ContainerInfo::mod_DockerInfo::Network::HOST { w.write_with_tag(16, |w| w.write_enum(*&self.network as i32))?; }
        for s in &self.port_mappings { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.privileged != false { w.write_with_tag(32, |w| w.write_bool(*&self.privileged))?; }
        for s in &self.parameters { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.force_pull_image { w.write_with_tag(48, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

pub mod mod_DockerInfo {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PortMapping<'a> {
    pub host_port: u32,
    pub container_port: u32,
    pub protocol: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for PortMapping<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.host_port = r.read_uint32(bytes)?,
                Ok(16) => msg.container_port = r.read_uint32(bytes)?,
                Ok(26) => msg.protocol = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PortMapping<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.host_port) as u64)
        + 1 + sizeof_varint(*(&self.container_port) as u64)
        + self.protocol.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.host_port))?;
        w.write_with_tag(16, |w| w.write_uint32(*&self.container_port))?;
        if let Some(ref s) = self.protocol { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Network {
    HOST = 1,
    BRIDGE = 2,
    NONE = 3,
    USER = 4,
}

impl Default for Network {
    fn default() -> Self {
        Network::HOST
    }
}

impl From<i32> for Network {
    fn from(i: i32) -> Self {
        match i {
            1 => Network::HOST,
            2 => Network::BRIDGE,
            3 => Network::NONE,
            4 => Network::USER,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Network {
    fn from(s: &'a str) -> Self {
        match s {
            "HOST" => Network::HOST,
            "BRIDGE" => Network::BRIDGE,
            "NONE" => Network::NONE,
            "USER" => Network::USER,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MesosInfo<'a> {
    pub image: Option<mesos::v1::Image<'a>>,
}

impl<'a> MessageRead<'a> for MesosInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.image = Some(r.read_message::<mesos::v1::Image>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MesosInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.image.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.image { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    DOCKER = 1,
    MESOS = 2,
}

impl Default for Type {
    fn default() -> Self {
        Type::DOCKER
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            1 => Type::DOCKER,
            2 => Type::MESOS,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "DOCKER" => Type::DOCKER,
            "MESOS" => Type::MESOS,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ContainerStatus<'a> {
    pub container_id: Option<mesos::v1::ContainerID<'a>>,
    pub network_infos: Vec<mesos::v1::NetworkInfo<'a>>,
    pub cgroup_info: Option<mesos::v1::CgroupInfo>,
    pub executor_pid: Option<u32>,
}

impl<'a> MessageRead<'a> for ContainerStatus<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(34) => msg.container_id = Some(r.read_message::<mesos::v1::ContainerID>(bytes)?),
                Ok(10) => msg.network_infos.push(r.read_message::<mesos::v1::NetworkInfo>(bytes)?),
                Ok(18) => msg.cgroup_info = Some(r.read_message::<mesos::v1::CgroupInfo>(bytes)?),
                Ok(24) => msg.executor_pid = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ContainerStatus<'a> {
    fn get_size(&self) -> usize {
        0
        + self.container_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.network_infos.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.cgroup_info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.executor_pid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.container_id { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.network_infos { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.cgroup_info { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.executor_pid { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CgroupInfo {
    pub net_cls: Option<mesos::v1::mod_CgroupInfo::NetCls>,
}

impl<'a> MessageRead<'a> for CgroupInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.net_cls = Some(r.read_message::<mesos::v1::mod_CgroupInfo::NetCls>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CgroupInfo {
    fn get_size(&self) -> usize {
        0
        + self.net_cls.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.net_cls { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_CgroupInfo {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Blkio { }

impl<'a> MessageRead<'a> for Blkio {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Blkio { }

pub mod mod_Blkio {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Value<'a> {
    pub op: Option<mesos::v1::Operation<'a>>,
    pub value: Option<u64>,
}

impl<'a> MessageRead<'a> for Value<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.op = Some(r.read_message::<mesos::v1::Operation>(bytes)?),
                Ok(16) => msg.value = Some(r.read_uint64(bytes)?),
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
        + self.op.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.op { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CFQ { }

impl<'a> MessageRead<'a> for CFQ {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for CFQ { }

pub mod mod_CFQ {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Statistics<'a> {
    pub device: Option<mesos::v1::mod_Device::Number>,
    pub sectors: Option<u64>,
    pub time: Option<u64>,
    pub io_serviced: Vec<mesos::v1::Value<'a>>,
    pub io_service_bytes: Vec<mesos::v1::Value<'a>>,
    pub io_service_time: Vec<mesos::v1::Value<'a>>,
    pub io_wait_time: Vec<mesos::v1::Value<'a>>,
    pub io_merged: Vec<mesos::v1::Value<'a>>,
    pub io_queued: Vec<mesos::v1::Value<'a>>,
}

impl<'a> MessageRead<'a> for Statistics<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.device = Some(r.read_message::<mesos::v1::mod_Device::Number>(bytes)?),
                Ok(16) => msg.sectors = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.time = Some(r.read_uint64(bytes)?),
                Ok(34) => msg.io_serviced.push(r.read_message::<mesos::v1::Value>(bytes)?),
                Ok(42) => msg.io_service_bytes.push(r.read_message::<mesos::v1::Value>(bytes)?),
                Ok(50) => msg.io_service_time.push(r.read_message::<mesos::v1::Value>(bytes)?),
                Ok(58) => msg.io_wait_time.push(r.read_message::<mesos::v1::Value>(bytes)?),
                Ok(66) => msg.io_merged.push(r.read_message::<mesos::v1::Value>(bytes)?),
                Ok(74) => msg.io_queued.push(r.read_message::<mesos::v1::Value>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Statistics<'a> {
    fn get_size(&self) -> usize {
        0
        + self.device.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.sectors.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.io_serviced.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.io_service_bytes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.io_service_time.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.io_wait_time.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.io_merged.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.io_queued.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.device { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.sectors { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.time { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        for s in &self.io_serviced { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.io_service_bytes { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.io_service_time { w.write_with_tag(50, |w| w.write_message(s))?; }
        for s in &self.io_wait_time { w.write_with_tag(58, |w| w.write_message(s))?; }
        for s in &self.io_merged { w.write_with_tag(66, |w| w.write_message(s))?; }
        for s in &self.io_queued { w.write_with_tag(74, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Throttling { }

impl<'a> MessageRead<'a> for Throttling {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Throttling { }

pub mod mod_Throttling {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Statistics<'a> {
    pub device: Option<mesos::v1::mod_Device::Number>,
    pub io_serviced: Vec<mesos::v1::Value<'a>>,
    pub io_service_bytes: Vec<mesos::v1::Value<'a>>,
}

impl<'a> MessageRead<'a> for Statistics<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.device = Some(r.read_message::<mesos::v1::mod_Device::Number>(bytes)?),
                Ok(18) => msg.io_serviced.push(r.read_message::<mesos::v1::Value>(bytes)?),
                Ok(26) => msg.io_service_bytes.push(r.read_message::<mesos::v1::Value>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Statistics<'a> {
    fn get_size(&self) -> usize {
        0
        + self.device.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.io_serviced.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.io_service_bytes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.device { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.io_serviced { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.io_service_bytes { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Statistics<'a> {
    pub cfq: Vec<mesos::v1::mod_CgroupInfo::mod_Blkio::mod_CFQ::Statistics<'a>>,
    pub cfq_recursive: Vec<mesos::v1::mod_CgroupInfo::mod_Blkio::mod_CFQ::Statistics<'a>>,
    pub throttling: Vec<mesos::v1::mod_CgroupInfo::mod_Blkio::mod_Throttling::Statistics<'a>>,
}

impl<'a> MessageRead<'a> for Statistics<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.cfq.push(r.read_message::<mesos::v1::mod_CgroupInfo::mod_Blkio::mod_CFQ::Statistics>(bytes)?),
                Ok(18) => msg.cfq_recursive.push(r.read_message::<mesos::v1::mod_CgroupInfo::mod_Blkio::mod_CFQ::Statistics>(bytes)?),
                Ok(26) => msg.throttling.push(r.read_message::<mesos::v1::mod_CgroupInfo::mod_Blkio::mod_Throttling::Statistics>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Statistics<'a> {
    fn get_size(&self) -> usize {
        0
        + self.cfq.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.cfq_recursive.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.throttling.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.cfq { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.cfq_recursive { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.throttling { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    UNKNOWN = 0,
    TOTAL = 1,
    READ = 2,
    WRITE = 3,
    SYNC = 4,
    ASYNC = 5,
}

impl Default for Operation {
    fn default() -> Self {
        Operation::UNKNOWN
    }
}

impl From<i32> for Operation {
    fn from(i: i32) -> Self {
        match i {
            0 => Operation::UNKNOWN,
            1 => Operation::TOTAL,
            2 => Operation::READ,
            3 => Operation::WRITE,
            4 => Operation::SYNC,
            5 => Operation::ASYNC,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Operation {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Operation::UNKNOWN,
            "TOTAL" => Operation::TOTAL,
            "READ" => Operation::READ,
            "WRITE" => Operation::WRITE,
            "SYNC" => Operation::SYNC,
            "ASYNC" => Operation::ASYNC,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NetCls {
    pub classid: Option<u32>,
}

impl<'a> MessageRead<'a> for NetCls {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.classid = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for NetCls {
    fn get_size(&self) -> usize {
        0
        + self.classid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.classid { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Labels<'a> {
    pub labels: Vec<mesos::v1::Label<'a>>,
}

impl<'a> MessageRead<'a> for Labels<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.labels.push(r.read_message::<mesos::v1::Label>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Labels<'a> {
    fn get_size(&self) -> usize {
        0
        + self.labels.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.labels { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Label<'a> {
    pub key: Cow<'a, str>,
    pub value: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Label<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Label<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.key).len())
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.key))?;
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Port<'a> {
    pub number: u32,
    pub name: Option<Cow<'a, str>>,
    pub protocol: Option<Cow<'a, str>>,
    pub visibility: Option<mesos::v1::mod_DiscoveryInfo::Visibility>,
    pub labels: Option<mesos::v1::Labels<'a>>,
}

impl<'a> MessageRead<'a> for Port<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.number = r.read_uint32(bytes)?,
                Ok(18) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.protocol = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.visibility = Some(r.read_enum(bytes)?),
                Ok(42) => msg.labels = Some(r.read_message::<mesos::v1::Labels>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Port<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.number) as u64)
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.protocol.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.visibility.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.labels.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.number))?;
        if let Some(ref s) = self.name { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.protocol { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.visibility { w.write_with_tag(32, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.labels { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ports<'a> {
    pub ports: Vec<mesos::v1::Port<'a>>,
}

impl<'a> MessageRead<'a> for Ports<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ports.push(r.read_message::<mesos::v1::Port>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Ports<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ports.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ports { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DiscoveryInfo<'a> {
    pub visibility: mesos::v1::mod_DiscoveryInfo::Visibility,
    pub name: Option<Cow<'a, str>>,
    pub environment: Option<Cow<'a, str>>,
    pub location: Option<Cow<'a, str>>,
    pub version: Option<Cow<'a, str>>,
    pub ports: Option<mesos::v1::Ports<'a>>,
    pub labels: Option<mesos::v1::Labels<'a>>,
}

impl<'a> MessageRead<'a> for DiscoveryInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.visibility = r.read_enum(bytes)?,
                Ok(18) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.environment = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.location = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.version = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.ports = Some(r.read_message::<mesos::v1::Ports>(bytes)?),
                Ok(58) => msg.labels = Some(r.read_message::<mesos::v1::Labels>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DiscoveryInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.visibility) as u64)
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.environment.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.location.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.version.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.ports.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.labels.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_enum(*&self.visibility as i32))?;
        if let Some(ref s) = self.name { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.environment { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.location { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.version { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.ports { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.labels { w.write_with_tag(58, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_DiscoveryInfo {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Visibility {
    FRAMEWORK = 0,
    CLUSTER = 1,
    EXTERNAL = 2,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::FRAMEWORK
    }
}

impl From<i32> for Visibility {
    fn from(i: i32) -> Self {
        match i {
            0 => Visibility::FRAMEWORK,
            1 => Visibility::CLUSTER,
            2 => Visibility::EXTERNAL,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Visibility {
    fn from(s: &'a str) -> Self {
        match s {
            "FRAMEWORK" => Visibility::FRAMEWORK,
            "CLUSTER" => Visibility::CLUSTER,
            "EXTERNAL" => Visibility::EXTERNAL,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WeightInfo<'a> {
    pub weight: f64,
    pub role: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for WeightInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.weight = r.read_double(bytes)?,
                Ok(18) => msg.role = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for WeightInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + 8
        + self.role.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(9, |w| w.write_double(*&self.weight))?;
        if let Some(ref s) = self.role { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct VersionInfo<'a> {
    pub version: Cow<'a, str>,
    pub build_date: Option<Cow<'a, str>>,
    pub build_time: Option<f64>,
    pub build_user: Option<Cow<'a, str>>,
    pub git_sha: Option<Cow<'a, str>>,
    pub git_branch: Option<Cow<'a, str>>,
    pub git_tag: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for VersionInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.version = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.build_date = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(25) => msg.build_time = Some(r.read_double(bytes)?),
                Ok(34) => msg.build_user = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.git_sha = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.git_branch = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.git_tag = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for VersionInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.version).len())
        + self.build_date.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.build_time.as_ref().map_or(0, |_| 1 + 8)
        + self.build_user.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.git_sha.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.git_branch.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.git_tag.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.version))?;
        if let Some(ref s) = self.build_date { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.build_time { w.write_with_tag(25, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.build_user { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.git_sha { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.git_branch { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.git_tag { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Flag<'a> {
    pub name: Cow<'a, str>,
    pub value: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Flag<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Flag<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Role<'a> {
    pub name: Cow<'a, str>,
    pub weight: f64,
    pub frameworks: Vec<mesos::v1::FrameworkID<'a>>,
    pub resources: Vec<mesos::v1::Resource<'a>>,
}

impl<'a> MessageRead<'a> for Role<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(17) => msg.weight = r.read_double(bytes)?,
                Ok(26) => msg.frameworks.push(r.read_message::<mesos::v1::FrameworkID>(bytes)?),
                Ok(34) => msg.resources.push(r.read_message::<mesos::v1::Resource>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Role<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + 1 + 8
        + self.frameworks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.resources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(17, |w| w.write_double(*&self.weight))?;
        for s in &self.frameworks { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.resources { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Metric<'a> {
    pub name: Cow<'a, str>,
    pub value: Option<f64>,
}

impl<'a> MessageRead<'a> for Metric<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(17) => msg.value = Some(r.read_double(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Metric<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + self.value.as_ref().map_or(0, |_| 1 + 8)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        if let Some(ref s) = self.value { w.write_with_tag(17, |w| w.write_double(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileInfo<'a> {
    pub path: Cow<'a, str>,
    pub nlink: Option<i32>,
    pub size: Option<u64>,
    pub mtime: Option<mesos::v1::TimeInfo>,
    pub mode: Option<u32>,
    pub uid: Option<Cow<'a, str>>,
    pub gid: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for FileInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.path = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.nlink = Some(r.read_int32(bytes)?),
                Ok(24) => msg.size = Some(r.read_uint64(bytes)?),
                Ok(34) => msg.mtime = Some(r.read_message::<mesos::v1::TimeInfo>(bytes)?),
                Ok(40) => msg.mode = Some(r.read_uint32(bytes)?),
                Ok(50) => msg.uid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.gid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FileInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.path).len())
        + self.nlink.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.size.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.mtime.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.mode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.gid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.path))?;
        if let Some(ref s) = self.nlink { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.size { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mtime { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.mode { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.uid { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.gid { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Device<'a> {
    pub path: Option<Cow<'a, str>>,
    pub number: Option<mesos::v1::mod_Device::Number>,
}

impl<'a> MessageRead<'a> for Device<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.number = Some(r.read_message::<mesos::v1::mod_Device::Number>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Device<'a> {
    fn get_size(&self) -> usize {
        0
        + self.path.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.number.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.path { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.number { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Device {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Number {
    pub major_number: u64,
    pub minor_number: u64,
}

impl<'a> MessageRead<'a> for Number {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.major_number = r.read_uint64(bytes)?,
                Ok(16) => msg.minor_number = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Number {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.major_number) as u64)
        + 1 + sizeof_varint(*(&self.minor_number) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint64(*&self.major_number))?;
        w.write_with_tag(16, |w| w.write_uint64(*&self.minor_number))?;
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeviceAccess<'a> {
    pub device: mesos::v1::Device<'a>,
    pub access: mesos::v1::mod_DeviceAccess::Access,
}

impl<'a> MessageRead<'a> for DeviceAccess<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.device = r.read_message::<mesos::v1::Device>(bytes)?,
                Ok(18) => msg.access = r.read_message::<mesos::v1::mod_DeviceAccess::Access>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeviceAccess<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.device).get_size())
        + 1 + sizeof_len((&self.access).get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.device))?;
        w.write_with_tag(18, |w| w.write_message(&self.access))?;
        Ok(())
    }
}

pub mod mod_DeviceAccess {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Access {
    pub read: Option<bool>,
    pub write: Option<bool>,
    pub mknod: Option<bool>,
}

impl<'a> MessageRead<'a> for Access {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.read = Some(r.read_bool(bytes)?),
                Ok(16) => msg.write = Some(r.read_bool(bytes)?),
                Ok(24) => msg.mknod = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Access {
    fn get_size(&self) -> usize {
        0
        + self.read.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.write.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.mknod.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.read { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.write { w.write_with_tag(16, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.mknod { w.write_with_tag(24, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeviceWhitelist<'a> {
    pub allowed_devices: Vec<mesos::v1::DeviceAccess<'a>>,
}

impl<'a> MessageRead<'a> for DeviceWhitelist<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.allowed_devices.push(r.read_message::<mesos::v1::DeviceAccess>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeviceWhitelist<'a> {
    fn get_size(&self) -> usize {
        0
        + self.allowed_devices.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.allowed_devices { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

