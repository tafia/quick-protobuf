use std::io::Read;

use errors::{Result, ErrorKind};
use message::Message;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian as LE;

const WIRE_TYPE_VARINT: u8 = 0;
const WIRE_TYPE_FIXED64: u8 = 1;
const WIRE_TYPE_LENGTH_DELIMITED: u8 = 2;
const WIRE_TYPE_START_GROUP: u8 = 3;
const WIRE_TYPE_END_GROUP: u8 = 4;
const WIRE_TYPE_FIXED32: u8 = 5;

/// A struct to read protocol binary files
pub struct Reader<R> {
    inner: R,
    len: usize,
}

impl<R: Read> Reader<R> {

    /// Creates a new protocol buffer reader with the maximum len of bytes to read
    pub fn from_reader(r: R, len: usize) -> Reader<R> {
        Reader { inner: r, len: len }
    }

    /// Reads next tag, `None` if all bytes have been read
    pub fn next_tag_value(&mut self) -> Option<Result<u32>> {
        if self.len == 0 {
            None
        } else {
            Some(self.read_varint().map(|i| (i as u32)))
        }
    }

    fn read_varint(&mut self) -> Result<u64> {
        let mut r: u64 = 0;
        let mut i = 0;
        for _ in 0..9 {
            self.len -= 1;
            let b = self.inner.read_u8()?;
            r |= ((b & 0x7f) as u64) << i;
            if b < 0x80 {
                return Ok(r);
            }
            i += 7;
        }
        self.len -= 1;
        match self.inner.read_u8()? {
            0 => Ok(r),
            1 => {
                r |= 1 << 63;
                Ok(r)
            }
            _ => Err(ErrorKind::Varint.into()), // we have only one spare bit to fit into
        }
    }

    pub fn read_int32(&mut self) -> Result<i32> {
        self.read_varint().map(|i| i as i32)
    }

    pub fn read_int64(&mut self) -> Result<i64> {
        self.read_varint().map(|i| i as i64)
    }

    pub fn read_uint32(&mut self) -> Result<u32> {
        self.read_varint().map(|i| i as u32)
    }

    pub fn read_uint64(&mut self) -> Result<u64> {
        self.read_varint()
    }

    pub fn read_sint32(&mut self) -> Result<i32> {
        // zigzag
        let n = self.read_varint()? as u32;
        Ok(((n >> 1) as i32) ^ (-((n & 1) as i32)))
    }

    pub fn read_sint64(&mut self) -> Result<i64> {
        // zigzag
        let n = self.read_varint()?;
        Ok(((n >> 1) as i64) ^ (-((n & 1) as i64)))
    }

    pub fn read_fixed64(&mut self) -> Result<u64> {
        self.len -= 8;
        self.inner.read_u64::<LE>().map_err(|e| e.into())
    }

    pub fn read_fixed32(&mut self) -> Result<u32> {
        self.len -= 4;
        self.inner.read_u32::<LE>().map_err(|e| e.into())
    }

    pub fn read_sfixed64(&mut self) -> Result<i64> {
        self.len -= 8;
        self.inner.read_i64::<LE>().map_err(|e| e.into())
    }

    pub fn read_sfixed32(&mut self) -> Result<i32> {
        self.len -= 4;
        self.inner.read_i32::<LE>().map_err(|e| e.into())
    }

    pub fn read_float(&mut self) -> Result<f32> {
        self.len -= 4;
        self.inner.read_f32::<LE>().map_err(|e| e.into())
    }

    pub fn read_double(&mut self) -> Result<f64> {
        self.len -= 8;
        self.inner.read_f64::<LE>().map_err(|e| e.into())
    }

    pub fn read_bool(&mut self) -> Result<bool> {
        self.read_varint().map(|i| i != 0)
    }

    pub fn read_enum<E: From<u64>>(&mut self) -> Result<E> {
        self.read_varint().map(|i| i.into())
    }

    pub fn read_bytes(&mut self) -> Result<Vec<u8>> {
        let len = self.read_varint()? as usize;
        self.len -= len;
        let mut vec = Vec::with_capacity(len);
        unsafe { vec.set_len(len); }
        self.inner.read_exact(&mut vec[..])?;
        Ok(vec)
    }

    pub fn read_string(&mut self) -> Result<String> {
        let vec = self.read_bytes()?;
        String::from_utf8(vec).map_err(|e| e.into())
    }

    pub fn read_packed_repeated_field<M, F: FnMut(&mut Self) -> Result<M>>(&mut self, mut read: F) -> Result<Vec<M>> {
        let len = self.read_varint()? as usize;
        let cur_len = self.len;
        self.len = len;
        let mut v = Vec::new();
        while self.len > 0 {
            v.push(read(self)?);
        }
        self.len = cur_len - len;
        Ok(v)
    }

    pub fn read_message<M: Message>(&mut self) -> Result<M> {
        let len = self.read_varint()? as usize;
        let cur_len = self.len;
        self.len = len;
        let msg = M::from_reader(self)?;
        self.len = cur_len - len;
        Ok(msg)
    }

    pub fn read_unknown(&mut self, tag_value: u32) -> Result<()> {
        match (tag_value & 0x7) as u8 {
            WIRE_TYPE_VARINT => { self.read_varint()?; },
            WIRE_TYPE_FIXED64 => {
                self.len -= 8;
                self.inner.read_exact(&mut [0; 8])?;
            }
            WIRE_TYPE_FIXED32 => {
                self.len -= 4;
                self.inner.read_exact(&mut [0; 4])?;
            }
            WIRE_TYPE_LENGTH_DELIMITED => {
                let len = self.read_varint()? as usize;
                if len == 0 { return Ok(()); }
                self.len -= len;
                let mut buf = Vec::with_capacity(len);
                unsafe { buf.set_len(len); }
                self.inner.read_exact(&mut buf)?;
            },
            WIRE_TYPE_START_GROUP | 
                WIRE_TYPE_END_GROUP => { return Err(ErrorKind::Deprecated("group").into()); },
            t => { return Err(ErrorKind::UnknownWireType(t).into()); },
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[test]
fn test_varint() {
    let data: &[u8] = &[0x96, 0x01];
    let mut r = Reader::from_reader(data, data.len());
    assert_eq!(150, r.read_varint().unwrap());
    assert!(r.next_tag().is_none());
}

#[test]
fn test_next_field() {
    let data: &[u8] = &[0x08, 0x96, 0x01];
    let mut r = Reader::from_reader(data, data.len());
    let tag = r.next_tag().unwrap().unwrap();
    assert_eq!((1, WireType::Varint), tag.unpack());
    assert_eq!(150, r.read_varint().unwrap());
    assert!(r.next_tag().is_none());
}
