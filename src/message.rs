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
pub struct DescriptorProto {
    pub name: Vec<u8>,
    pub fields: Vec<FieldDescriptorProto>,
//     import: String,
}

#[derive(Debug, Default)]
pub struct FieldDescriptorProto {
    pub name: Vec<u8>,
}

impl Message for DescriptorProto {
    fn from_reader<R: BufRead>(mut r: &mut Reader<R>) -> Result<Self> {
        let mut desc = DescriptorProto::default();
        loop {
            match r.next_tag() {
                None => break,
                Some(Err(e)) => return Err(e),
                Some(Ok(tag)) => {
                    println!("tag: {:?}", tag);
                    match tag.unpack() {
                        (1, WireType::LengthDelimited) => desc.name = r.read_bytes()?,
                        (2, WireType::LengthDelimited) => {
                            desc.fields.push(r.read_embedded_message()?);
                        }
                        (1, _) => return Err(ErrorKind::InvalidMessage(tag, "String").into()),
                        (2, _) => return Err(ErrorKind::InvalidMessage(tag, "Repeated FieldDescriptor").into()),
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

impl Message for FieldDescriptorProto {
    fn from_reader<R: BufRead>(mut r: &mut Reader<R>) -> Result<Self> {
        let mut field = FieldDescriptorProto::default();
        loop {
            match r.next_tag() {
                None => break,
                Some(Err(e)) => return Err(e),
                Some(Ok(tag)) => {
                    println!("tag: {:?}", tag);
                    match tag.unpack() {
                        (1, WireType::LengthDelimited) => field.name = r.read_bytes()?,
                        (1, _) => return Err(ErrorKind::InvalidMessage(tag, "String").into()),
//                         (3, WireType::LengthDelimited) => desc.import = r.read_string()?,
//                         (3, _) => return Err(ErrorKind::InvalidMessage(tag, "String").into()),
                        _ => r.read_unknown(tag.wire_type())?,
                    }
                }
            }
        }
        Ok(field)
    }
}

