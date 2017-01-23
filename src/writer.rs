use std::io::Write;
use errors::Result;
// use message::Message;

use byteorder::WriteBytesExt;
// use byteorder::LittleEndian as LE;


/// A struct to write protobuf messages
pub struct ProtobufWriter<W> {
    inner: W,
}

impl<W: Write> ProtobufWriter<W> {

    /// Creates a new `ProtobufWriter`
    pub fn new(w: W) -> ProtobufWriter<W> {
        ProtobufWriter { inner: w }
    }

    pub fn write_varint(&mut self, mut v: u64) -> Result<()> {
        while v > 0x7F {
            self.inner.write_u8(((v as u8) & 0x7F) | 0x80)?;
            v >>= 7;
        }
        self.inner.write_u8(v as u8)?;
        Ok(())
    }

    pub fn write_tag(&mut self, tag: u32) -> Result<()> {
        self.write_varint(tag as u64)?;
        Ok(())
    }

    pub fn write_bytes(&mut self, tag: u32, bytes: &[u8]) -> Result<()> {
        self.write_tag(tag)?;
        self.write_varint(bytes.len() as u64)?;
        self.inner.write_all(bytes)?;
        Ok(())
    }


}
