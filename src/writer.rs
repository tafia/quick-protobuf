use std::io::Write;

use errors::Result;
use message::MessageWrite;

use byteorder::WriteBytesExt;
use byteorder::LittleEndian as LE;

/// A struct to write protobuf messages
pub struct Writer<W> {
    inner: W,
}

impl<W: Write> Writer<W> {

    /// Creates a new `ProtobufWriter`
    pub fn new(w: W) -> Writer<W> {
        Writer { inner: w }
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

    pub fn write_int32(&mut self, v: i32) -> Result<()> {
        self.write_varint(v as u64)
    }

    pub fn write_int64(&mut self, v: i64) -> Result<()> {
        self.write_varint(v as u64)
    }

    pub fn write_uint32(&mut self, v: u32) -> Result<()> {
        self.write_varint(v as u64)
    }

    pub fn write_uint64(&mut self, v: u64) -> Result<()> {
        self.write_varint(v)
    }

    pub fn write_sint32(&mut self, v: i32) -> Result<()> {
        self.write_varint(((v << 1) ^ (v >> 31)) as u64)
    }

    pub fn write_sint64(&mut self, v: i64) -> Result<()> {
        self.write_varint(((v << 1) ^ (v >> 63)) as u64)
    }

    pub fn write_fixed64(&mut self, v: u64) -> Result<()> {
        self.inner.write_u64::<LE>(v).map_err(|e| e.into())
    }

    pub fn write_fixed32(&mut self, v: u32) -> Result<()> {
        self.inner.write_u32::<LE>(v).map_err(|e| e.into())
    }

    pub fn write_sfixed64(&mut self, v: i64) -> Result<()> {
        self.inner.write_i64::<LE>(v).map_err(|e| e.into())
    }

    pub fn write_sfixed32(&mut self, v: i32) -> Result<()> {
        self.inner.write_i32::<LE>(v).map_err(|e| e.into())
    }

    pub fn write_float(&mut self, v: f32) -> Result<()> {
        self.inner.write_f32::<LE>(v).map_err(|e| e.into())
    }

    pub fn write_double(&mut self, v: f64) -> Result<()> {
        self.inner.write_f64::<LE>(v).map_err(|e| e.into())
    }

    pub fn write_bool(&mut self, v: bool) -> Result<()> {
        self.write_varint(if v { 1 } else { 0 })
    }

    pub fn write_enum(&mut self, v: i32) -> Result<()> {
        self.write_int32(v)
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.write_varint(bytes.len() as u64)?;
        self.inner.write_all(bytes).map_err(|e| e.into())
    }

    pub fn write_string(&mut self, s: &str) -> Result<()> {
        self.write_bytes(s.as_bytes())
    }

    pub fn write_packed_repeated_field<M, F, S>(&mut self, v: &[M], mut write: F, size: &S) -> Result<()>
        where F: FnMut(&mut Self, &M) -> Result<()>,
              S: Fn(&M) -> usize,
    {
        let len: usize = v.iter().map(|m| size(m)).sum();
        self.write_varint(len as u64)?;
        for m in v {
            write(self, m)?;
        }
        Ok(())
    }

    pub fn write_packed_fixed_size<M>(&mut self, v: &[M], item_size: usize) -> Result<()> {
        let len = v.len() * item_size;
        let bytes = unsafe { ::std::slice::from_raw_parts(v as *const [M] as *const M as *const u8, len) };
        self.write_bytes(bytes)
    }

    pub fn write_int32_with_tag(&mut self, tag: u32, v: i32) -> Result<()> {
        self.write_tag(tag)?;
        self.write_varint(v as u64)
    }

    pub fn write_int64_with_tag(&mut self, tag: u32, v: i64) -> Result<()> {
        self.write_tag(tag)?;
        self.write_varint(v as u64)
    }

    pub fn write_uint32_with_tag(&mut self, tag: u32, v: u32) -> Result<()> {
        self.write_tag(tag)?;
        self.write_varint(v as u64)
    }

    pub fn write_uint64_with_tag(&mut self, tag: u32, v: u64) -> Result<()> {
        self.write_tag(tag)?;
        self.write_varint(v)
    }

    pub fn write_sint32_with_tag(&mut self, tag: u32, v: i32) -> Result<()> {
        self.write_tag(tag)?;
        self.write_sint32(v)
    }

    pub fn write_sint64_with_tag(&mut self, tag: u32, v: i64) -> Result<()> {
        self.write_tag(tag)?;
        self.write_sint64(v)
    }

    pub fn write_fixed64_with_tag(&mut self, tag: u32, v: u64) -> Result<()> {
        self.write_tag(tag)?;
        self.inner.write_u64::<LE>(v).map_err(|e| e.into())
    }

    pub fn write_fixed32_with_tag(&mut self, tag: u32, v: u32) -> Result<()> {
        self.write_tag(tag)?;
        self.inner.write_u32::<LE>(v).map_err(|e| e.into())
    }

    pub fn write_sfixed64_with_tag(&mut self, tag: u32, v: i64) -> Result<()> {
        self.write_tag(tag)?;
        self.inner.write_i64::<LE>(v).map_err(|e| e.into())
    }

    pub fn write_sfixed32_with_tag(&mut self, tag: u32, v: i32) -> Result<()> {
        self.write_tag(tag)?;
        self.inner.write_i32::<LE>(v).map_err(|e| e.into())
    }

    pub fn write_float_with_tag(&mut self, tag: u32, v: f32) -> Result<()> {
        self.write_tag(tag)?;
        self.inner.write_f32::<LE>(v).map_err(|e| e.into())
    }

    pub fn write_double_with_tag(&mut self, tag: u32, v: f64) -> Result<()> {
        self.write_tag(tag)?;
        self.inner.write_f64::<LE>(v).map_err(|e| e.into())
    }

    pub fn write_bool_with_tag(&mut self, tag: u32, v: bool) -> Result<()> {
        self.write_tag(tag)?;
        self.write_varint(if v { 1 } else { 0 })
    }

    pub fn write_bytes_with_tag(&mut self, tag: u32, bytes: &[u8]) -> Result<()> {
        self.write_tag(tag)?;
        self.write_varint(bytes.len() as u64)?;
        self.inner.write_all(bytes).map_err(|e| e.into())
    }

    pub fn write_string_with_tag(&mut self, tag: u32, s: &str) -> Result<()> {
        self.write_tag(tag)?;
        self.write_bytes(s.as_bytes())
    }

    pub fn write_packed_repeated_field_with_tag<M, F, S>(&mut self, 
                                                         tag: u32, 
                                                         v: &[M], 
                                                         mut write: F, 
                                                         size: &S) -> Result<()>
        where F: FnMut(&mut Self, &M) -> Result<()>,
              S: Fn(&M) -> usize,
    {
        self.write_tag(tag)?;
        let len: usize = v.iter().map(|m| size(m)).sum();
        self.write_varint(len as u64)?;
        for m in v {
            write(self, m)?;
        }
        Ok(())
    }

    pub fn write_packed_fixed_size_with_tag<M>(&mut self, 
                                               tag: u32, 
                                               v: &[M], 
                                               item_size: usize) -> Result<()> {
        self.write_tag(tag)?;
        let len = v.len() * item_size;
        let bytes = unsafe { ::std::slice::from_raw_parts(v as *const [M] as *const M as *const u8, len) };
        self.write_bytes(bytes)
    }

    pub fn write_message_with_tag<M: MessageWrite>(&mut self, tag: u32, m: &M) -> Result<()> {
        self.write_tag(tag)?;
        let len = m.get_size();
        self.write_varint(len as u64)?;
        m.write_message(self)
    }

    pub fn write_enum_with_tag(&mut self, tag: u32, v: i32) -> Result<()> {
        self.write_tag(tag)?;
        self.write_int32(v)
    }
}
