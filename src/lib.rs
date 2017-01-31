//! A library to read binary protobuf files
//!
//! This reader is developed similarly to a pull reader

#![deny(missing_docs)]

#![recursion_limit = "1024"]
#![allow(dead_code)]

#[macro_use]
extern crate error_chain;
extern crate byteorder;

pub mod errors;
pub mod message;
pub mod reader;
pub mod writer;
pub mod sizeofs;
// pub mod packed;

pub use errors::Result;
pub use message::{MessageWrite};
// pub use message::{MessageRead, MessageWrite};
pub use reader::Reader;
pub use writer::Writer;
