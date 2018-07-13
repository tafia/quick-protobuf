//! A module to manage protobuf deserialization
//!
//! There are actually two main *readers*
//! - a `BytesReader` which parses data from a `&[u8]`
//! - a `Reader` which is a wrapper on `BytesReader` which has its own buffer. It provides
//! convenient functions to the user suche as `from_file`
//!
//! It is advised, for convenience to directly work with a `Reader`.

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use errors::{Error, Result};
use message::MessageRead;

use byteorder::ByteOrder;
use byteorder::LittleEndian as LE;

const WIRE_TYPE_VARINT: u8 = 0;
const WIRE_TYPE_FIXED64: u8 = 1;
const WIRE_TYPE_LENGTH_DELIMITED: u8 = 2;
const WIRE_TYPE_START_GROUP: u8 = 3;
const WIRE_TYPE_END_GROUP: u8 = 4;
const WIRE_TYPE_FIXED32: u8 = 5;

/// A struct to read protocol binary files
///
/// # Examples
///
/// ```rust
/// # mod foo_bar {
/// #     use quick_protobuf::{MessageRead, BytesReader, Result};
/// #     pub struct Foo {}
/// #     pub struct Bar {}
/// #     pub struct FooBar { pub foos: Vec<Foo>, pub bars: Vec<Bar>, }
/// #     impl<'a> MessageRead<'a> for FooBar {
/// #         fn from_reader(_: &mut BytesReader, _: &[u8]) -> Result<Self> {
/// #              Ok(FooBar { foos: vec![], bars: vec![] })
/// #         }
/// #     }
/// # }
///
/// // FooBar is a message generated from a proto file
/// // in parcicular it contains a `from_reader` function
/// use foo_bar::FooBar;
/// use quick_protobuf::{MessageRead, BytesReader};
///
/// fn main() {
///     // bytes is a buffer on the data we want to deserialize
///     // typically bytes is read from a `Read`:
///     // r.read_to_end(&mut bytes).expect("cannot read bytes");
///     let mut bytes: Vec<u8>;
///     # bytes = vec![];
///
///     // we can build a bytes reader directly out of the bytes
///     let mut reader = BytesReader::from_bytes(&bytes);
///
///     // now using the generated module decoding is as easy as:
///     let foobar = FooBar::from_reader(&mut reader, &bytes).expect("Cannot read FooBar");
///
///     // if instead the buffer contains a length delimited stream of message we could use:
///     // while !r.is_eof() {
///     //     let foobar: FooBar = r.read_message(&bytes).expect(...);
///     //     ...
///     // }
///     println!("Found {} foos and {} bars", foobar.foos.len(), foobar.bars.len());
/// }
/// ```
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
            end: bytes.len(),
        }
    }

    /// Reads next tag, `None` if all bytes have been read
    #[inline(always)]
    pub fn next_tag(&mut self, bytes: &[u8]) -> Result<u32> {
        self.read_varint32(bytes)
    }

    /// Reads the next byte
    #[inline(always)]
    pub fn read_u8(&mut self, bytes: &[u8]) -> Result<u8> {
        let b = bytes.get(self.start).ok_or_else::<Error, _>(|| {
            Error::Io(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Cannot read next bytes",
            ))
        })?;
        self.start += 1;
        Ok(*b)
    }

    /// Reads the next varint encoded u64
    #[inline(always)]
    pub fn read_varint32(&mut self, bytes: &[u8]) -> Result<u32> {
        let mut b = self.read_u8(bytes)?;
        if b & 0x80 == 0 {
            return Ok(b as u32);
        }
        let mut r = (b & 0x7f) as u32;

        b = self.read_u8(bytes)?;
        r |= ((b & 0x7f) as u32) << 7;
        if b & 0x80 == 0 {
            return Ok(r);
        }

        b = self.read_u8(bytes)?;
        r |= ((b & 0x7f) as u32) << 14;
        if b & 0x80 == 0 {
            return Ok(r);
        }

        b = self.read_u8(bytes)?;
        r |= ((b & 0x7f) as u32) << 21;
        if b & 0x80 == 0 {
            return Ok(r);
        }

        b = self.read_u8(bytes)?;
        r |= ((b & 0xf) as u32) << 28;
        if b & 0x80 == 0 {
            return Ok(r);
        }

        // discards extra bytes
        for _ in 0..5 {
            if self.read_u8(bytes)? & 0x80 == 0 {
                return Ok(r);
            }
        }

        // cannot read more than 10 bytes
        Err(Error::Varint)
    }

    /// Reads the next varint encoded u64
    #[inline(always)]
    pub fn read_varint64(&mut self, bytes: &[u8]) -> Result<u64> {
        // part0
        let mut b = self.read_u8(bytes)?;
        if b & 0x80 == 0 {
            return Ok(b as u64);
        }
        let mut r0 = (b & 0x7f) as u32;

        b = self.read_u8(bytes)?;
        r0 |= ((b & 0x7f) as u32) << 7;
        if b & 0x80 == 0 {
            return Ok(r0 as u64);
        }

        b = self.read_u8(bytes)?;
        r0 |= ((b & 0x7f) as u32) << 14;
        if b & 0x80 == 0 {
            return Ok(r0 as u64);
        }

        b = self.read_u8(bytes)?;
        r0 |= ((b & 0x7f) as u32) << 21;
        if b & 0x80 == 0 {
            return Ok(r0 as u64);
        }

        // part1
        b = self.read_u8(bytes)?;
        let mut r1 = (b & 0x7f) as u32;
        if b & 0x80 == 0 {
            return Ok(r0 as u64 | (r1 as u64) << 28);
        }

        b = self.read_u8(bytes)?;
        r1 |= ((b & 0x7f) as u32) << 7;
        if b & 0x80 == 0 {
            return Ok(r0 as u64 | (r1 as u64) << 28);
        }

        b = self.read_u8(bytes)?;
        r1 |= ((b & 0x7f) as u32) << 14;
        if b & 0x80 == 0 {
            return Ok(r0 as u64 | (r1 as u64) << 28);
        }

        b = self.read_u8(bytes)?;
        r1 |= ((b & 0x7f) as u32) << 21;
        if b & 0x80 == 0 {
            return Ok(r0 as u64 | (r1 as u64) << 28);
        }

        // part2
        b = self.read_u8(bytes)?;
        let mut r2 = (b & 0x7f) as u32;
        if b & 0x80 == 0 {
            return Ok((r0 as u64 | (r1 as u64) << 28) | (r2 as u64) << 56);
        }

        b = self.read_u8(bytes)?;
        r2 |= (b as u32) << 7;
        if b & 0x80 == 0 {
            return Ok((r0 as u64 | (r1 as u64) << 28) | (r2 as u64) << 56);
        }

        // cannot read more than 10 bytes
        Err(Error::Varint)
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
        let v = read(&bytes[self.start..self.start + len]);
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
    where
        F: FnMut(&mut BytesReader, &'a [u8]) -> Result<M>,
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
    pub fn read_bytes<'a>(&mut self, bytes: &'a [u8]) -> Result<&'a [u8]> {
        self.read_len(bytes, |r, b| Ok(&b[r.start..r.end]))
    }

    /// Reads string (String)
    #[inline]
    pub fn read_string<'a>(&mut self, bytes: &'a [u8]) -> Result<&'a str> {
        self.read_len(bytes, |r, b| {
            ::std::str::from_utf8(&b[r.start..r.end]).map_err(|e| e.into())
        })
    }

    /// Reads packed repeated field (Vec<M>)
    ///
    /// Note: packed field are stored as a variable length chunk of data, while regular repeated
    /// fields behaves like an iterator, yielding their tag everytime
    #[inline]
    pub fn read_packed<'a, M, F>(&mut self, bytes: &'a [u8], mut read: F) -> Result<Vec<M>>
    where
        F: FnMut(&mut BytesReader, &'a [u8]) -> Result<M>,
    {
        self.read_len(bytes, |r, b| {
            let mut v = Vec::new();
            while !r.is_eof() {
                v.push(read(r, b)?);
            }
            Ok(v)
        })
    }

    /// Reads packed repeated field where M can directly be transmutted from raw bytes
    ///
    /// Note: packed field are stored as a variable length chunk of data, while regular repeated
    /// fields behaves like an iterator, yielding their tag everytime
    #[inline]
    pub fn read_packed_fixed<'a, M>(&mut self, bytes: &'a [u8]) -> Result<&'a [M]> {
        let len = self.read_varint32(bytes)? as usize;
        if self.len() < len {
            return Err(Error::Io(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Cannot read fixed packed field",
            )));
        }
        let n = len / ::std::mem::size_of::<M>();
        let slice = unsafe {
            ::std::slice::from_raw_parts(
                bytes.get_unchecked(self.start) as *const u8 as *const M,
                n,
            )
        };
        self.start += len;
        Ok(slice)
    }

    /// Reads a nested message
    #[inline]
    pub fn read_message<'a, M>(&mut self, bytes: &'a [u8]) -> Result<M>
    where
        M: MessageRead<'a>,
    {
        self.read_len(bytes, M::from_reader)
    }

    /// Reads a map item: (key, value)
    #[inline]
    pub fn read_map<'a, K, V, F, G>(
        &mut self,
        bytes: &'a [u8],
        mut read_key: F,
        mut read_val: G,
    ) -> Result<(K, V)>
    where
        F: FnMut(&mut BytesReader, &'a [u8]) -> Result<K>,
        G: FnMut(&mut BytesReader, &'a [u8]) -> Result<V>,
        K: ::std::fmt::Debug + Default,
        V: ::std::fmt::Debug + Default,
    {
        self.read_len(bytes, |r, bytes| {
            let mut k = K::default();
            let mut v = V::default();
            while !r.is_eof() {
                let t = r.read_u8(bytes)?;
                match t >> 3 {
                    1 => k = read_key(r, bytes)?,
                    2 => v = read_val(r, bytes)?,
                    t => return Err(Error::Map(t)),
                }
            }
            Ok((k, v))
        })
    }

    /// Reads unknown data, based on its tag value (which itself gives us the wire_type value)
    #[inline]
    pub fn read_unknown(&mut self, bytes: &[u8], tag_value: u32) -> Result<()> {
        match (tag_value & 0x7) as u8 {
            WIRE_TYPE_VARINT => {
                self.read_varint64(bytes)?;
            }
            WIRE_TYPE_FIXED64 => self.start += 8,
            WIRE_TYPE_FIXED32 => self.start += 4,
            WIRE_TYPE_LENGTH_DELIMITED => {
                let len = self.read_varint64(bytes)? as usize;
                self.start += len;
            }
            WIRE_TYPE_START_GROUP | WIRE_TYPE_END_GROUP => {
                return Err(Error::Deprecated("group"));
            }
            t => {
                return Err(Error::UnknownWireType(t));
            }
        }
        Ok(())
    }

    /// Gets the remaining length of bytes not read yet
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Checks if `self.len == 0`
    #[inline(always)]
    pub fn is_eof(&self) -> bool {
        self.start == self.end
    }

    /// Advance inner cursor to the end
    pub fn read_to_end(&mut self) {
        self.start = self.end;
    }
}

/// A struct to read protobuf data
///
/// Contrary to `BytesReader`, this struct will own a buffer
///
/// # Examples
///
/// ```rust,should_panic
/// # mod foo_bar {
/// #     use quick_protobuf::{MessageRead, BytesReader, Result};
/// #     pub struct Foo {}
/// #     pub struct Bar {}
/// #     pub struct FooBar { pub foos: Vec<Foo>, pub bars: Vec<Bar>, }
/// #     impl<'a> MessageRead<'a> for FooBar {
/// #         fn from_reader(_: &mut BytesReader, _: &[u8]) -> Result<Self> {
/// #              Ok(FooBar { foos: vec![], bars: vec![] })
/// #         }
/// #     }
/// # }
///
/// // FooBar is a message generated from a proto file
/// // In particular it implements the `MessageRead` trait, containing a `from_reader` function.
/// use foo_bar::FooBar;
/// use quick_protobuf::Reader;
///
/// fn main() {
///     // create a reader, which will parse the protobuf binary file and pop events
///     // this reader will read the entire file into an internal buffer
///     let mut reader = Reader::from_file("/path/to/binary/protobuf.bin")
///         .expect("Cannot read input file");
///
///     // Use the generated module fns with the reader to convert your data into rust structs.
///     //
///     // Depending on your input file, the message can or not be prefixed with the encoded length
///     // for instance, a *stream* which contains several messages generally split them using this
///     // technique (see https://developers.google.com/protocol-buffers/docs/techniques#streaming)
///     //
///     // To read a message without a length prefix you can directly call `FooBar::from_reader`:
///     // let foobar = reader.read(FooBar::from_reader).expect("Cannot read FooBar message");
///     //
///     // Else to read a length then a message, you can use:
///     let foobar: FooBar = reader.read(|r, b| r.read_message(b))
///         .expect("Cannot read FooBar message");
///     // Reader::read_message uses `FooBar::from_reader` internally through the `MessageRead`
///     // trait.
///
///     println!("Found {} foos and {} bars!", foobar.foos.len(), foobar.bars.len());
/// }
/// ```
pub struct Reader {
    buffer: Vec<u8>,
    inner: BytesReader,
}

impl Reader {
    /// Creates a new `Reader`
    pub fn from_reader<R: Read>(mut r: R, capacity: usize) -> Result<Reader> {
        let mut buf = Vec::with_capacity(capacity);
        unsafe {
            buf.set_len(capacity);
        }
        buf.shrink_to_fit();
        r.read_exact(&mut buf)?;
        Ok(Reader::from_bytes(buf))
    }

    /// Creates a new `Reader` out of a file path
    pub fn from_file<P: AsRef<Path>>(src: P) -> Result<Reader> {
        let len = src.as_ref().metadata().unwrap().len() as usize;
        let f = File::open(src)?;
        Reader::from_reader(f, len)
    }

    /// Creates a new reader consuming the bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Reader {
        let reader = BytesReader {
            start: 0,
            end: bytes.len(),
        };
        Reader {
            buffer: bytes,
            inner: reader,
        }
    }

    /// Run a `BytesReader` dependent function
    #[inline]
    pub fn read<'a, M, F>(&'a mut self, mut read: F) -> Result<M>
    where
        F: FnMut(&mut BytesReader, &'a [u8]) -> Result<M>,
    {
        read(&mut self.inner, &self.buffer)
    }

    /// Gets the inner `BytesReader`
    pub fn inner(&mut self) -> &mut BytesReader {
        &mut self.inner
    }

    /// Gets the buffer used internally
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}

#[test]
fn test_varint() {
    let data = [0x96, 0x01];
    let mut r = BytesReader::from_bytes(&data[..]);
    assert_eq!(150, r.read_varint32(&data[..]).unwrap());
    assert!(r.is_eof());
}
