//! A module to deserialize a `Message` as defined in a .proto file
//!
//! Creates the struct and implements a reader

use std::io::{Read, BufReader};
use std::path::Path;
use std::fs::File;

use errors::Result;
use reader::Reader;

/// A trait to handle deserialization based on parsed `Field`s
pub trait Message: Sized {

    /// Creates `Self` from a `Reader`
    ///
    /// This method is generally automatically implemented when generating code
    /// out of .proto file
    fn from_reader<R: Read>(r: &mut Reader<R>) -> Result<Self>;

    /// Creates Message out of a file
    ///
    /// Convenient method for the top `Message` in the hierarchy of binary messages
    fn from_file<P: AsRef<Path>>(p: P) -> Result<Self> {
        let len = p.as_ref().metadata()?.len() as usize;
        let file = BufReader::new(File::open(p)?);
        let mut reader = Reader::from_reader(file, len);
        Self::from_reader(&mut reader)
    }

    /// Creates Message out of a file
    ///
    /// Convenient method for the top `Message` in the hierarchy of binary messages
    ///
    /// The file is _entirely_ read into memory before being parsed, 
    /// which boost performances but uses twice as much of memory
    fn from_file_for_speed<P: AsRef<Path>>(p: P) -> Result<Self> {
        let len = p.as_ref().metadata()?.len() as usize;
        let v = {
            let mut v = Vec::with_capacity(len);
            let mut r = File::open(p)?;
            r.read_to_end(&mut v)?;
            v
        };
        let mut v = &*v;
        let mut reader = Reader::from_reader(&mut v, len);
        Self::from_reader(&mut reader)
    }
}
