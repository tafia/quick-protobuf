//! A module to manage protobuf deserialization

use errors::{Result, ErrorKind};
use message::MessageRead;

use byteorder::LittleEndian as LE;
use byteorder::ByteOrder;

const WIRE_TYPE_VARINT: u8 = 0;
const WIRE_TYPE_FIXED64: u8 = 1;
const WIRE_TYPE_LENGTH_DELIMITED: u8 = 2;
const WIRE_TYPE_START_GROUP: u8 = 3;
const WIRE_TYPE_END_GROUP: u8 = 4;
const WIRE_TYPE_FIXED32: u8 = 5;

/// A struct to read protocol binary files
pub struct Reader<'a> {
    inner: &'a [u8],
    pos: usize,
    len: usize,
}

impl<'a> Reader<'a> {

    /// Creates a new reader from chunks of data
    pub fn from_bytes(bytes: &'a [u8]) -> Reader<'a> {
        let len = bytes.len();
        Reader {
            inner: bytes,
            len: len,
            pos: 0,
        }
    }

    /// Reads next tag, `None` if all bytes have been read
    pub fn next_tag(&mut self) -> Result<u32> {
        self.read_varint().map(|i| (i as u32))
    }

    /// Reads the next varint encoded u64
    pub fn read_varint(&mut self) -> Result<u64> {
        let mut r: u64 = 0;
        let mut i = 0;
        for _ in 0..9 {
            let b = self.inner.read_u8()?;
            self.len -= 1;
            r |= ((b & 0x7f) as u64) << i;
            if b < 0x80 {
                return Ok(r);
            }
            i += 7;
        }
        let res = match self.inner[self.pos] {
            0 => Ok(r),
            1 => {
                r |= 1 << 63;
                Ok(r)
            }
            _ => Err(ErrorKind::Varint.into()), // we have only one spare bit to fit into
        };
        self.pos += 1;
        res
    }

    /// Reads int32 (varint)
    pub fn read_int32(&mut self) -> Result<i32> {
        self.read_varint().map(|i| i as i32)
    }

    /// Reads int64 (varint)
    pub fn read_int64(&mut self) -> Result<i64> {
        self.read_varint().map(|i| i as i64)
    }

    /// Reads uint32 (varint)
    pub fn read_uint32(&mut self) -> Result<u32> {
        self.read_varint().map(|i| i as u32)
    }

    /// Reads uint64 (varint)
    pub fn read_uint64(&mut self) -> Result<u64> {
        self.read_varint()
    }

    /// Reads sint32 (varint)
    pub fn read_sint32(&mut self) -> Result<i32> {
        // zigzag
        let n = self.read_varint()? as u32;
        Ok(((n >> 1) as i32) ^ (-((n & 1) as i32)))
    }

    /// Reads sint64 (varint)
    pub fn read_sint64(&mut self) -> Result<i64> {
        // zigzag
        let n = self.read_varint()?;
        Ok(((n >> 1) as i64) ^ (-((n & 1) as i64)))
    }

    /// Reads fixed64 (little endian u64)
    pub fn read_fixed64(&mut self) -> Result<u64> {
        self.pos += 8;
        Ok(LE::read_u64(&self.inner[self.pos - 8 .. self.pos]))
    }

    /// Reads fixed32 (little endian u32)
    pub fn read_fixed32(&mut self) -> Result<u32> {
        self.pos += 4;
        Ok(LE::read_u32(&self.inner[self.pos - 4 .. self.pos]))
    }

    /// Reads sfixed64 (little endian i64)
    pub fn read_sfixed64(&mut self) -> Result<i64> {
        self.pos += 8;
        Ok(LE::read_i64(&self.inner[self.pos - 8 .. self.pos]))
    }

    /// Reads sfixed32 (little endian i32)
    pub fn read_sfixed32(&mut self) -> Result<i32> {
        self.pos += 4;
        Ok(LE::read_i32(&self.inner[self.pos - 4 .. self.pos]))
    }

    /// Reads float (little endian f32)
    pub fn read_float(&mut self) -> Result<f32> {
        self.pos += 4;
        Ok(LE::read_f32(&self.inner[self.pos - 4 .. self.pos]))
    }

    /// Reads double (little endian f64)
    pub fn read_double(&mut self) -> Result<f64> {
        self.pos += 8;
        Ok(LE::read_f64(&self.inner[self.pos - 8 .. self.pos]))
    }

    /// Reads bool (varint, check if == 0)
    pub fn read_bool(&mut self) -> Result<bool> {
        self.read_varint().map(|i| i != 0)
    }

    /// Reads enum, encoded as i32
    pub fn read_enum<E: From<i32>>(&mut self) -> Result<E> {
        self.read_int32().map(|e| e.into())
    }

    /// Reads bytes (Vec<u8>)
    pub fn read_bytes(&mut self) -> Result<&'a [u8]> {
        let len = self.read_varint()? as usize;
        self.pos += len;
        Ok(&self.inner[self.pos - len .. self.pos])
    }

    /// Reads string (String)
    pub fn read_str(&mut self) -> Result<&'a str> {
        let vec = self.read_bytes()?;
        ::std::str::from_utf8(vec).map_err(|e| e.into())
    }

    /// Reads packed repeated field (Vec<M>)
    ///
    /// Note: packed field are stored as a variable length chunk of data, while regular repeated
    /// fields behaves like an iterator, yielding their tag everytime
    pub fn read_packed_repeated_field<M, F: FnMut(&mut Self) -> Result<M>>(&mut self, mut read: F) -> Result<Vec<M>> {
        let len = self.read_varint()? as usize;
        let cur_len = self.len;
        self.len = self.pos + len;
        let mut v = Vec::new();
        while !self.is_eof() {
            v.push(read(self)?);
        }
        self.len = cur_len;
        Ok(v)
    }

    /// Reads a nested message
    pub fn read_message<M: MessageRead>(&mut self) -> Result<M> {
        let len = self.read_varint()? as usize;
        let cur_len = self.len;
        self.len = self.pos + len;
        let msg = M::from_reader(self)?;
        self.pos = self.len; // probably not necessary ...
        self.len = cur_len;
        Ok(msg)
    }

    /// Reads unknown data, based on its tag value (which itself gives us the wire_type value)
    pub fn read_unknown(&mut self, tag_value: u32) -> Result<()> {
        match (tag_value & 0x7) as u8 {
            WIRE_TYPE_VARINT => { self.read_varint()?; },
            WIRE_TYPE_FIXED64 => self.pos += 8,
            WIRE_TYPE_FIXED32 => self.pos += 4,
            WIRE_TYPE_LENGTH_DELIMITED => {
                let len = self.read_varint()? as usize;
                self.pos += len;
            },
            WIRE_TYPE_START_GROUP | 
                WIRE_TYPE_END_GROUP => { return Err(ErrorKind::Deprecated("group").into()); },
            t => { return Err(ErrorKind::UnknownWireType(t).into()); },
        }
        Ok(())
    }

    /// Gets the remaining length of bytes not read yet
    pub fn len(&self) -> usize {
        self.len
    }

//     /// Gets the inner reader
//     pub fn inner(&mut self) -> &mut R {
//         &mut self.inner
//     }

    /// Checks if `self.len == 0`
    pub fn is_eof(&self) -> bool {
        self.pos == self.len
    }
}

#[test]
fn test_varint() {
    let data = [0x96, 0x01];
    let mut r = Reader::from_bytes(&data[..]);
    assert_eq!(150, r.read_varint().unwrap());
    assert!(r.is_eof());
}
