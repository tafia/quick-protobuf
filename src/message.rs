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
        let reader = BufReader::new(File::open(p.as_ref())?);
        let mut reader = Reader::from_reader(reader, len);
        Self::from_reader(&mut reader)
    }
}
