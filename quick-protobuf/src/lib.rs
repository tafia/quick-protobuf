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

pub use errors::{Error, Result};
pub use message::{MessageRead, MessageWrite};
pub use reader::{deserialize_from_slice, BytesReader, Reader};
pub use writer::{serialize_into_vec, Writer};
