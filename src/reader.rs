use std::io::{BufRead};

use errors::{Result, ErrorKind};
use types::{Tag, WireType};
use message::Message;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian as LE;

/// A struct to read protocol binary files
pub struct Reader<R> {
    inner: R,
    len: usize,
}

impl<R: BufRead> Reader<R> {

    /// Creates a new protocol buffer reader with the maximum len of bytes to read
    pub fn from_reader(r: R, len: usize) -> Reader<R> {
        Reader { inner: r, len: len }
    }

    /// Reads next tag, `None` if all bytes have been read
    pub fn next_tag(&mut self) -> Option<Result<Tag>> {
        if self.len == 0 {
            None
        } else {
            Some(self.read_varint().map(|i| (i as u32).into()))
        }
    }

    fn read_varint(&mut self) -> Result<u64> {
        let mut r: u64 = 0;
        let mut i = 0;
        for _ in 0..10 {
            self.len -= 1;
            let b = self.inner.read_u8()?;
            // TODO: may overflow if i == 9
            r |= ((b & 0x7f) as u64) << i;
            if b < 0x80 {
                return Ok(r);
            }
            i += 7;
        }
        if i == 70 {
            Err(ErrorKind::Varint.into())
        } else {
            Err(ErrorKind::Eof.into())
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
        unimplemented!()
    }

    pub fn read_sint64(&mut self) -> Result<i64> {
        unimplemented!()
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

    pub fn read_enum<E>(&mut self) -> Result<E> {
        unimplemented!()
    }

    pub fn read_unknown(&mut self, wire_type: WireType) -> Result<()> {
        // TODO: explore using Seek inner reader
        let mut len = match wire_type {
            WireType::Varint => return self.read_varint().map(|_| ()),
            WireType::Fixed64 => 8,
            WireType::LengthDelimited => self.read_varint()? as usize,
            WireType::StartGroup | 
                WireType::EndGroup => return Err(ErrorKind::Deprecated("group").into()),
            WireType::Fixed32 => 4,
            WireType::Unknown => return Err(ErrorKind::UnknownWireType.into()),
        };
        
        if len == 0 {
            return Ok(());
        }

        // consume len bytes
        self.len -= len;
        loop {
            let read = match self.inner.fill_buf() {
                Ok(v) if v.is_empty() => return Err(ErrorKind::Eof.into()),
                Ok(v) => v.len(),
                Err(e) => return Err(e.into()),
            };
            if read >= len {
                self.inner.consume(len);
                return Ok(());
            } else {
                len -= read;
                self.inner.consume(read);
            }
        }
    }

    pub fn read_bytes(&mut self) -> Result<Vec<u8>> {
        let len = self.read_varint()? as usize;
        self.len -= len;
        let mut vec = vec![0u8; len];
        self.inner.read_exact(&mut vec[..])?;
        Ok(vec)
    }

    pub fn read_string(&mut self) -> Result<String> {
        let mut vec = self.read_bytes()?;
        let _: Vec<_> = vec.drain(..2).collect();
        String::from_utf8(vec).map_err(|e| e.into())
    }

    pub fn read_packed_repeated_field(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub fn read_message<M: Message>(&mut self) -> Result<M> {
        let len = self.read_varint()? as usize;
        let cur_len = self.len;
        self.len = len;
        let msg = M::from_reader(self)?;
        self.len = cur_len - len;
        Ok(msg)
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
