//! A module to deserialize a `Message` as defined in a .proto file
//!
//! Creates the struct and implements a reader

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use errors::Result;
use reader::BytesReader;
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

/// A trait to handle deserialization from protocol buffers.
pub trait MessageRead<'a>: Sized {
    /// Constructs an instance of `Self` by reading from the given bytes
    /// via the given reader.
    ///
    /// It does NOT read message length first. If you want to read a variable
    /// length message, use `BytesReader::read_message` directly
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self>;
}
