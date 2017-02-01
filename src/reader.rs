//! A module to manage protobuf deserialization

use std::io::Read;
use std::path::Path;
use std::fs::File;

use errors::{Result, ErrorKind};

use byteorder::LittleEndian as LE;
use byteorder::ByteOrder;

const WIRE_TYPE_VARINT: u8 = 0;
const WIRE_TYPE_FIXED64: u8 = 1;
const WIRE_TYPE_LENGTH_DELIMITED: u8 = 2;
const WIRE_TYPE_START_GROUP: u8 = 3;
const WIRE_TYPE_END_GROUP: u8 = 4;
const WIRE_TYPE_FIXED32: u8 = 5;

/// A struct to read protocol binary files
#[derive(Debug, Clone, PartialEq)]
pub struct BytesReader {
    start: usize,
    end: usize,
}

impl BytesReader {

    /// Creates a new reader from chunks of data
    pub fn from_bytes(bytes: &[u8]) -> BytesReader {
        BytesReader {
            start: 0,
            end: bytes.len()
        }
    }

    /// Reads next tag, `None` if all bytes have been read
    #[inline(always)]
    pub fn next_tag(&mut self, bytes: &[u8]) -> Result<u32> {
        self.read_varint32(bytes)
    }

    #[inline(always)]
    fn read_u8(&mut self, bytes: &[u8]) -> u8 {
        let b = bytes[self.start];
        self.start += 1;
        b
    }

    /// Reads the next varint encoded u64
    #[inline(always)]
    pub fn read_varint32(&mut self, bytes: &[u8]) -> Result<u32> {
        let mut b = self.read_u8(bytes);
        if b & 0x80 == 0 { return Ok(b as u32); }
        let mut r = (b & 0x7f) as u32;

        b = self.read_u8(bytes);
        r |= ((b & 0x7f) as u32) << 7;
        if b & 0x80 == 0 { return Ok(r); }

        b = self.read_u8(bytes);
        r |= ((b & 0x7f) as u32) << 14;
        if b & 0x80 == 0 { return Ok(r); }

        b = self.read_u8(bytes);
        r |= ((b & 0x7f) as u32) << 21;
        if b & 0x80 == 0 { return Ok(r); }

        b = self.read_u8(bytes);
        r |= ((b & 0xf) as u32) << 28;
        if b & 0x80 == 0 { return Ok(r); }

        // discards extra bytes
        for _ in 0..5 {
            if self.read_u8(bytes) & 0x80 == 0 { return Ok(r); }
        }

        // cannot read more than 10 bytes
        Err(ErrorKind::Varint.into())
    }

    /// Reads the next varint encoded u64
    #[inline(always)]
    pub fn read_varint64(&mut self, bytes: &[u8]) -> Result<u64> {

        // part0
        let mut b = self.read_u8(bytes);
        if b & 0x80 == 0 { return Ok(b as u64); }
        let mut r0 = (b & 0x7f) as u32;

        b = self.read_u8(bytes);
        r0 |= ((b & 0x7f) as u32) << 7;
        if b & 0x80 == 0 { return Ok(r0 as u64); }

        b = self.read_u8(bytes);
        r0 |= ((b & 0x7f) as u32) << 14;
        if b & 0x80 == 0 { return Ok(r0 as u64); }

        b = self.read_u8(bytes);
        r0 |= ((b & 0x7f) as u32) << 21;
        if b & 0x80 == 0 { return Ok(r0 as u64); }

        // part1
        b = self.read_u8(bytes);
        let mut r1 = (b & 0x7f) as u32;
        if b & 0x80 == 0 { return Ok((r0 as u64 | (r1 as u64) << 28)); }

        b = self.read_u8(bytes);
        r1 |= ((b & 0x7f) as u32) << 7;
        if b & 0x80 == 0 { return Ok((r0 as u64 | (r1 as u64) << 28)); }

        b = self.read_u8(bytes);
        r1 |= ((b & 0x7f) as u32) << 14;
        if b & 0x80 == 0 { return Ok((r0 as u64 | (r1 as u64) << 28)); }

        b = self.read_u8(bytes);
        r1 |= ((b & 0x7f) as u32) << 21;
        if b & 0x80 == 0 { return Ok((r0 as u64 | (r1 as u64) << 28)); }

        // part2
        b = self.read_u8(bytes);
        let mut r2 = (b & 0x7f) as u32;
        if b & 0x80 == 0 { return Ok(((r0 as u64 | (r1 as u64) << 28) | (r2 as u64) << 56)); }

        b = self.read_u8(bytes);
        r2 |= (b as u32) << 7;
        if b & 0x80 == 0 { return Ok(((r0 as u64 | (r1 as u64) << 28) | (r2 as u64) << 56)); }

        // cannot read more than 10 bytes
        Err(ErrorKind::Varint.into())
        
    }

    /// Reads int32 (varint)
    #[inline]
    pub fn read_int32(&mut self, bytes: &[u8]) -> Result<i32> {
        self.read_varint32(bytes).map(|i| i as i32)
    }

    /// Reads int64 (varint)
    #[inline]
    pub fn read_int64(&mut self, bytes: &[u8]) -> Result<i64> {
        self.read_varint64(bytes).map(|i| i as i64)
    }

    /// Reads uint32 (varint)
    #[inline]
    pub fn read_uint32(&mut self, bytes: &[u8]) -> Result<u32> {
        self.read_varint32(bytes)
    }

    /// Reads uint64 (varint)
    #[inline]
    pub fn read_uint64(&mut self, bytes: &[u8]) -> Result<u64> {
        self.read_varint64(bytes)
    }

    /// Reads sint32 (varint)
    #[inline]
    pub fn read_sint32(&mut self, bytes: &[u8]) -> Result<i32> {
        // zigzag
        let n = self.read_varint32(bytes)?;
        Ok(((n >> 1) as i32) ^ (-((n & 1) as i32)))
    }

    /// Reads sint64 (varint)
    #[inline]
    pub fn read_sint64(&mut self, bytes: &[u8]) -> Result<i64> {
        // zigzag
        let n = self.read_varint64(bytes)?;
        Ok(((n >> 1) as i64) ^ (-((n & 1) as i64)))
    }

    /// Reads fixed64 (little endian u64)
    #[inline]
    fn read_fixed<M, F: Fn(&[u8]) -> M>(&mut self, bytes: &[u8], len: usize, read: F) -> Result<M> {
        let v = read(&bytes[self.start .. self.start + len]);
        self.start += len;
        Ok(v)
    }

    /// Reads fixed64 (little endian u64)
    #[inline]
    pub fn read_fixed64(&mut self, bytes: &[u8]) -> Result<u64> {
        self.read_fixed(bytes, 8, LE::read_u64)
    }

    /// Reads fixed32 (little endian u32)
    #[inline]
    pub fn read_fixed32(&mut self, bytes: &[u8]) -> Result<u32> {
        self.read_fixed(bytes, 4, LE::read_u32)
    }

    /// Reads sfixed64 (little endian i64)
    #[inline]
    pub fn read_sfixed64(&mut self, bytes: &[u8]) -> Result<i64> {
        self.read_fixed(bytes, 8, LE::read_i64)
    }

    /// Reads sfixed32 (little endian i32)
    #[inline]
    pub fn read_sfixed32(&mut self, bytes: &[u8]) -> Result<i32> {
        self.read_fixed(bytes, 4, LE::read_i32)
    }

    /// Reads float (little endian f32)
    #[inline]
    pub fn read_float(&mut self, bytes: &[u8]) -> Result<f32> {
        self.read_fixed(bytes, 4, LE::read_f32)
    }

    /// Reads double (little endian f64)
    #[inline]
    pub fn read_double(&mut self, bytes: &[u8]) -> Result<f64> {
        self.read_fixed(bytes, 8, LE::read_f64)
    }

    /// Reads bool (varint, check if == 0)
    #[inline]
    pub fn read_bool(&mut self, bytes: &[u8]) -> Result<bool> {
        self.read_varint32(bytes).map(|i| i != 0)
    }

    /// Reads enum, encoded as i32
    #[inline]
    pub fn read_enum<E: From<i32>>(&mut self, bytes: &[u8]) -> Result<E> {
        self.read_int32(bytes).map(|e| e.into())
    }

    #[inline(always)]
    fn read_len<'a, M, F>(&mut self, bytes: &'a [u8], mut read: F) -> Result<M>
        where F: FnMut(&mut BytesReader, &'a[u8]) -> Result<M>,
    {
        let len = self.read_varint32(bytes)? as usize;
        let cur_end = self.end;
        self.end = self.start + len;
        let v = read(self, bytes)?;
        self.start = self.end;
        self.end = cur_end;
        Ok(v)
    }

    /// Reads bytes (Vec<u8>)
    #[inline]
    pub fn read_bytes<'a>(&mut self, bytes: &'a[u8]) -> Result<&'a[u8]> {
        self.read_len(bytes, |r, b| Ok(&b[r.start..r.end]))
    }

    /// Reads string (String)
    #[inline]
    pub fn read_string<'a>(&mut self, bytes: &'a[u8]) -> Result<&'a str> {
        self.read_len(bytes, |r, b| ::std::str::from_utf8(&b[r.start..r.end]).map_err(|e| e.into()))
    }

    /// Reads packed repeated field (Vec<M>)
    ///
    /// Note: packed field are stored as a variable length chunk of data, while regular repeated
    /// fields behaves like an iterator, yielding their tag everytime
    #[inline]
    pub fn read_packed<'a, M, F>(&mut self, bytes: &'a[u8], mut read: F) -> Result<Vec<M>>
        where F: FnMut(&mut BytesReader, &'a[u8]) -> Result<M>,
    {
        self.read_len(bytes, |r, b| {
            let mut v = Vec::new();
            while !r.is_eof() {
                v.push(read(r, b)?);
            }
            Ok(v)
        })
    }

    /// Reads a nested message
    #[inline]
    pub fn read_message<'a, M, F>(&mut self, bytes: &'a[u8], read: F) -> Result<M>
        where F: FnMut(&mut BytesReader, &'a[u8]) -> Result<M> 
    {
        self.read_len(bytes, read)
    }

    /// Reads unknown data, based on its tag value (which itself gives us the wire_type value)
    #[inline]
    pub fn read_unknown(&mut self, bytes: &[u8], tag_value: u32) -> Result<()> {
        match (tag_value & 0x7) as u8 {
            WIRE_TYPE_VARINT => { self.read_varint64(bytes)?; },
            WIRE_TYPE_FIXED64 => self.start += 8,
            WIRE_TYPE_FIXED32 => self.start += 4,
            WIRE_TYPE_LENGTH_DELIMITED => {
                let len = self.read_varint64(bytes)? as usize;
                self.start += len;
            },
            WIRE_TYPE_START_GROUP | 
                WIRE_TYPE_END_GROUP => { return Err(ErrorKind::Deprecated("group").into()); },
            t => { return Err(ErrorKind::UnknownWireType(t).into()); },
        }
        Ok(())
    }

    /// Gets the remaining length of bytes not read yet
    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Checks if `self.len == 0`
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.start == self.end
    }
}

/// A struct to read protobuf data
///
/// Contrary to `BytesReader`, this struct will own the buffer
pub struct Reader {
    buf: Vec<u8>,
    reader: BytesReader,
}

impl Reader {

    /// Creates a new `Reader`
    pub fn from_reader<R: Read>(mut r: R, capacity: usize) -> Result<Reader> {
        let mut buf = Vec::with_capacity(capacity);
        unsafe { buf.set_len(capacity); }
        buf.shrink_to_fit();
        r.read_exact(&mut buf)?;
        let reader = BytesReader {
            start: 0,
            end: capacity,
        };
        Ok(Reader {
            buf: buf,
            reader: reader,
        })
    }

    /// Creates a new `Reader` out of a file path
    pub fn from_file<P: AsRef<Path>>(src: P) -> Result<Reader> {
        let len = src.as_ref().metadata().unwrap().len() as usize;
        let f = File::open(src)?;
        Reader::from_reader(f, len)
    }

    /// Run a `BytesReader` dependent function
    #[inline]
    pub fn read<'a, M, F>(&'a mut self, mut read: F) -> Result<M>
        where F: FnMut(&mut BytesReader, &'a[u8]) -> Result<M> 
    {
        read(&mut self.reader, &self.buf)
    }

}

#[test]
fn test_varint() {
    let data = [0x96, 0x01];
    let mut r = BytesReader::from_bytes(&data[..]);
    assert_eq!(150, r.read_varint32(&data[..]).unwrap());
    assert!(r.is_eof());
}
