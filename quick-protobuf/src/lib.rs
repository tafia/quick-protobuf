//! A library to read binary protobuf files
//!
//! This reader is developed similarly to a pull reader

#![deny(missing_docs)]
#![allow(dead_code)]

extern crate byteorder;
extern crate failure;
extern crate failure_derive;

pub mod errors;
pub mod message;
pub mod reader;
pub mod sizeofs;
pub mod writer;

pub use crate::errors::{Error, Result};
pub use crate::message::{MessageRead, MessageWrite};
pub use crate::reader::{deserialize_from_slice, BytesReader};
pub use crate::writer::{serialize_into_vec, serialize_into_slice, BytesWriter, Writer, WriterBackend};
