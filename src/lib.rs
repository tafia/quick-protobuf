//! A library to read binary protobuf files
//!
//! This reader is developed similarly to a pull reader

#![recursion_limit = "1024"]
#![allow(dead_code)]

#[macro_use] extern crate error_chain;
extern crate byteorder;

pub mod errors;
pub mod types;
pub mod reader;
pub mod message;

#[cfg(test)]
mod tests {
    
    use message::{DescriptorProto, Message};

    #[test]
    fn test_proto_descriptor() {
        let f = DescriptorProto::from_file("/home/jtuffe/\
            download/protos/Samples/AR/AERO_ASI_AR_20161121_1392440_44126797.bin").unwrap();
        println!("protobuf: {:?}", f.fields.len());

        let f = DescriptorProto::from_file("/home/jtuffe/\
            download/protos/Samples/AR/AERO_ASI_AR_20161121_1392440_44121917.bin").unwrap();
        println!("protobuf: {:?}", f.fields.len());

        let f = DescriptorProto::from_file("/home/jtuffe/\
            download/protos/Samples/AR/AERO_ASI_AR_20161121_1392440_44121891.bin").unwrap();
        println!("protobuf: {:?}", f.fields.len());
    }
}
