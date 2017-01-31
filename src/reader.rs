//! A module to manage protobuf deserialization

use errors::{Result, ErrorKind};
// use message::MessageRead;

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
pub struct Reader {
    start: usize,
    end: usize,
}

impl Reader {

    /// Creates a new reader from chunks of data
    pub fn from_bytes(bytes: &[u8]) -> Reader {
        Reader {
            start: 0,
            end: bytes.len()
        }
    }

    /// Reads next tag, `None` if all bytes have been read
    pub fn next_tag(&mut self, bytes: &[u8]) -> Result<u32> {
        self.read_varint(bytes).map(|i| (i as u32))
    }

    /// Reads the next varint encoded u64
    pub fn read_varint(&mut self, bytes: &[u8]) -> Result<u64> {
        let mut r: u64 = 0;
        let mut i = 0;
        for (j, b) in bytes.iter().skip(self.start).take(9).cloned().enumerate() {
            r |= ((b & 0x7f) as u64) << i;
            if b < 0x80 {
                self.start += j + 1;
                return Ok(r);
            }
            i += 7;
        }
        self.start += 9;
        let res = match bytes[self.start] {
            0 => Ok(r),
            1 => {
                r |= 1 << 63;
                Ok(r)
            }
            _ => Err(ErrorKind::Varint.into()), // we have only one spare bit to fit into
        };
        self.start += 1;
        res
    }

    /// Reads int32 (varint)
    pub fn read_int32(&mut self, bytes: &[u8]) -> Result<i32> {
        self.read_varint(bytes).map(|i| i as i32)
    }

    /// Reads int64 (varint)
    pub fn read_int64(&mut self, bytes: &[u8]) -> Result<i64> {
        self.read_varint(bytes).map(|i| i as i64)
    }

    /// Reads uint32 (varint)
    pub fn read_uint32(&mut self, bytes: &[u8]) -> Result<u32> {
        self.read_varint(bytes).map(|i| i as u32)
    }

    /// Reads uint64 (varint)
    pub fn read_uint64(&mut self, bytes: &[u8]) -> Result<u64> {
        self.read_varint(bytes)
    }

    /// Reads sint32 (varint)
    pub fn read_sint32(&mut self, bytes: &[u8]) -> Result<i32> {
        // zigzag
        let n = self.read_varint(bytes)? as u32;
        Ok(((n >> 1) as i32) ^ (-((n & 1) as i32)))
    }

    /// Reads sint64 (varint)
    pub fn read_sint64(&mut self, bytes: &[u8]) -> Result<i64> {
        // zigzag
        let n = self.read_varint(bytes)?;
        Ok(((n >> 1) as i64) ^ (-((n & 1) as i64)))
    }

    /// Reads fixed64 (little endian u64)
    fn read_fixed<M, F: Fn(&[u8]) -> M>(&mut self, bytes: &[u8], len: usize, read: F) -> Result<M> {
        let v = read(&bytes[self.start .. self.start + len]);
        self.start += len;
        Ok(v)
    }

    /// Reads fixed64 (little endian u64)
    pub fn read_fixed64(&mut self, bytes: &[u8]) -> Result<u64> {
        self.read_fixed(bytes, 8, LE::read_u64)
    }

    /// Reads fixed32 (little endian u32)
    pub fn read_fixed32(&mut self, bytes: &[u8]) -> Result<u32> {
        self.read_fixed(bytes, 4, LE::read_u32)
    }

    /// Reads sfixed64 (little endian i64)
    pub fn read_sfixed64(&mut self, bytes: &[u8]) -> Result<i64> {
        self.read_fixed(bytes, 8, LE::read_i64)
    }

    /// Reads sfixed32 (little endian i32)
    pub fn read_sfixed32(&mut self, bytes: &[u8]) -> Result<i32> {
        self.read_fixed(bytes, 4, LE::read_i32)
    }

    /// Reads float (little endian f32)
    pub fn read_float(&mut self, bytes: &[u8]) -> Result<f32> {
        self.read_fixed(bytes, 4, LE::read_f32)
    }

    /// Reads double (little endian f64)
    pub fn read_double(&mut self, bytes: &[u8]) -> Result<f64> {
        self.read_fixed(bytes, 8, LE::read_f64)
    }

    /// Reads bool (varint, check if == 0)
    pub fn read_bool(&mut self, bytes: &[u8]) -> Result<bool> {
        self.read_varint(bytes).map(|i| i != 0)
    }

    /// Reads enum, encoded as i32
    pub fn read_enum<E: From<i32>>(&mut self, bytes: &[u8]) -> Result<E> {
        self.read_int32(bytes).map(|e| e.into())
    }

    fn read_len<'a, M, F>(&mut self, bytes: &'a [u8], mut read: F) -> Result<M>
        where F: FnMut(&mut Reader, &'a[u8]) -> Result<M>,
    {
        let len = self.read_varint(bytes)? as usize;
        let cur_end = self.end;
        self.end = self.start + len;
        let v = read(self, bytes)?;
        self.start = self.end;
        self.end = cur_end;
        Ok(v)
    }

    /// Reads bytes (Vec<u8>)
    pub fn read_bytes<'a>(&mut self, bytes: &'a[u8]) -> Result<&'a[u8]> {
        self.read_len(bytes, |r, b| Ok(&b[r.start..r.end]))
    }

    /// Reads string (String)
    pub fn read_string<'a>(&mut self, bytes: &'a[u8]) -> Result<&'a str> {
        self.read_len(bytes, |r, b| ::std::str::from_utf8(&b[r.start..r.end]).map_err(|e| e.into()))
    }

    /// Reads packed repeated field (Vec<M>)
    ///
    /// Note: packed field are stored as a variable length chunk of data, while regular repeated
    /// fields behaves like an iterator, yielding their tag everytime
    pub fn read_packed_repeated_field<'a, M, F>(&mut self, bytes: &'a[u8], mut read: F) -> Result<Vec<M>>
        where F: FnMut(&mut Reader, &'a[u8]) -> Result<M>,
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
    pub fn read_message<'a, M, F>(&mut self, bytes: &'a[u8], read: F) -> Result<M>
        where F: FnMut(&mut Reader, &'a[u8]) -> Result<M> 
    {
        self.read_len(bytes, read)
    }

    /// Reads unknown data, based on its tag value (which itself gives us the wire_type value)
    pub fn read_unknown(&mut self, bytes: &[u8], tag_value: u32) -> Result<()> {
        match (tag_value & 0x7) as u8 {
            WIRE_TYPE_VARINT => { self.read_varint(bytes)?; },
            WIRE_TYPE_FIXED64 => self.start += 8,
            WIRE_TYPE_FIXED32 => self.start += 4,
            WIRE_TYPE_LENGTH_DELIMITED => {
                let len = self.read_varint(bytes)? as usize;
                self.start += len;
            },
            WIRE_TYPE_START_GROUP | 
                WIRE_TYPE_END_GROUP => { return Err(ErrorKind::Deprecated("group").into()); },
            t => { return Err(ErrorKind::UnknownWireType(t).into()); },
        }
        Ok(())
    }

    /// Gets the remaining length of bytes not read yet
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Checks if `self.len == 0`
    pub fn is_eof(&self) -> bool {
        self.start == self.end
    }
}

#[test]
fn test_varint() {
    let data = [0x96, 0x01];
    let mut r = Reader::from_bytes(&data[..]);
    assert_eq!(150, r.read_varint(&data[..]).unwrap());
    assert!(r.is_eof());
}
