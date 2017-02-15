//! A module to deserialize a `Message` as defined in a .proto file
//!
//! Creates the struct and implements a reader

use std::io::{Write, BufWriter};
use std::path::Path;
use std::fs::File;

use errors::Result;
use writer::Writer;

/// A trait to handle deserialization based on parsed `Field`s
pub trait MessageWrite: Sized {

    /// Writes `Self` into W writer
    fn write_message<W: Write>(&self, _: &mut Writer<W>) -> Result<()> {
        Ok(())
    }

    /// Computes necessary binary size of self once serialized in protobuf
    fn get_size(&self) -> usize {
        0
    }

    /// Writes self into a file
    fn write_file<P: AsRef<Path>>(&self, p: P) -> Result<()> {
        let file = BufWriter::new(File::create(p)?);
        let mut writer = Writer::new(file);
        self.write_message(&mut writer)
    }
}
