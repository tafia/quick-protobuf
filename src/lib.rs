//! A library to read binary protobuf files
//!
//! This reader is developed similarly to a pull reader

#![deny(missing_docs)]
#![recursion_limit = "1024"]
#![allow(dead_code)]

extern crate byteorder;
#[macro_use]
extern crate error_chain;

pub mod errors;
pub mod message;
pub mod reader;
pub mod writer;
pub mod sizeofs;

pub use errors::Result;
pub use message::{MessageRead, MessageWrite};
pub use reader::{BytesReader, Reader};
pub use writer::Writer;
