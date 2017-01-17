//! A library to read binary protobuf files
//!
//! This reader is developed similarly to a pull reader

#![recursion_limit = "1024"]
#![allow(dead_code)]

#[macro_use]
extern crate error_chain;
extern crate byteorder;

pub mod errors;
pub mod types;
pub mod reader;
pub mod message;

#[cfg(test)]
mod tests {
    
}
