//! A module to deserialize a `Message` as defined in a .proto file
//!
//! Creates the struct and implements a reader

use std::io::{Write, BufWriter};
use std::path::Path;
use std::fs::File;

use errors::Result;
// use reader::Reader;
use writer::Writer;

// /// A trait to handle deserialization based on parsed `Field`s
// pub trait MessageRead<'a>: Sized {
// 
//     /// Creates `Self` from a `Reader`
//     ///
//     /// This method is generally automatically implemented when generating code
//     /// out of .proto file
//     fn from_reader(r: &mut Reader, bytes: &'a[u8]) -> Result<Self>;
// }

/// A trait to handle deserialization based on parsed `Field`s
pub trait MessageWrite: Sized {

    /// Writes `Self` into W writer
    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()>;

    /// Computes necessary binary size of self once serialized in protobuf
    fn get_size(&self) -> usize;

    /// Writes self into a file
    fn write_file<P: AsRef<Path>>(&self, p: P) -> Result<()> {
        let file = BufWriter::new(File::create(p)?);
        let mut writer = Writer::new(file);
        self.write_message(&mut writer)
    }
}
