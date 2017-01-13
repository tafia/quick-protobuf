//! A module to deserialize a `Message` written in a .proto file
//!
//! Creates the struct and implements a reader

use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::File;

use errors::{Result, ErrorKind};
use reader::{Reader};
use types::WireType;

/// A trait to handle deserialization based on parsed `Field`s
pub trait Message: Sized {
    fn from_reader<R: BufRead>(r: &mut Reader<R>) -> Result<Self>;

    /// Creates Message out of a file
    fn from_file<P: AsRef<Path>>(p: P) -> Result<Self> {
        let len = p.as_ref().metadata()?.len() as usize;
        let reader = BufReader::new(File::open(p.as_ref())?);
        let mut reader = Reader::from_reader(reader, len);
        Self::from_reader(&mut reader)
    }
}

#[derive(Debug, Default)]
pub struct ProtoDescriptor {
    pub filename: Vec<u8>,
    pub package: String,
//     import: String,
}

// #[derive(PartialEq,Clone,Default)]
// pub struct FileDescriptorProto {
//     // message fields
//     name: String,
//     package: String,
// //     dependency: ::protobuf::RepeatedField<::std::string::String>,
//     public_dependency: Vec<i32>,
//     weak_dependency: Vec<i32>,
// //     message_type: ::protobuf::RepeatedField<DescriptorProto>,
// //     enum_type: ::protobuf::RepeatedField<EnumDescriptorProto>,
// //     service: ::protobuf::RepeatedField<ServiceDescriptorProto>,
// //     extension: ::protobuf::RepeatedField<FieldDescriptorProto>,
// //     options: ::protobuf::SingularPtrField<FileOptions>,
// //     source_code_info: ::protobuf::SingularPtrField<SourceCodeInfo>,
//     syntax: String,
// }

impl Message for ProtoDescriptor {
    fn from_reader<R: BufRead>(mut r: &mut Reader<R>) -> Result<Self> {
        let mut desc = ProtoDescriptor::default();
        loop {
            match r.next_tag() {
                None => break,
                Some(Err(e)) => return Err(e),
                Some(Ok(tag)) => {
                    println!("tag: {:?}", tag);
                    match tag.unpack() {
                        (1, WireType::LengthDelimited) => desc.filename = r.read_bytes()?,
//                         (2, WireType::LengthDelimited) => desc.package = r.read_string()?,

                        (1, _) => return Err(ErrorKind::InvalidMessage(tag, "String").into()),
//                         (2, _) => return Err(ErrorKind::InvalidMessage(tag, "String").into()),
//                         (3, WireType::LengthDelimited) => desc.import = r.read_string()?,
//                         (3, _) => return Err(ErrorKind::InvalidMessage(tag, "String").into()),
                        _ => r.read_unknown(tag.wire_type())?,
                    }
                }
            }
        }
        Ok(desc)
    }
}

