//! A module to manage protobuf deserialization
//!
//! There are actually two main *readers*
//! - a `BytesReader` which parses data from a `&[u8]`
//! - a `Reader` which is a wrapper on `BytesReader` which has its own buffer. It provides
//! convenient functions to the user suche as `from_file`
//!
//! It is advised, for convenience to directly work with a `Reader`.

use core::iter::FusedIterator;
#[cfg(feature = "std")]
use std::borrow::ToOwned;
#[cfg(feature = "std")]
use std::fs::File;
#[cfg(feature = "std")]
use std::io::Read;
#[cfg(feature = "std")]
use std::path::Path;

use core::convert::TryFrom;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::borrow::Cow;
#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::errors::{Error, Result};
use crate::message::MessageRead;

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
/// use quick_protobuf::{BytesReader, MessageRead};
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
///     println!(
///         "Found {} foos and {} bars",
///         foobar.foos.len(),
///         foobar.bars.len()
///     );
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
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
    #[cfg_attr(std, inline(always))]
    pub fn next_tag(&mut self, bytes: &[u8]) -> Result<u32> {
        self.read_varint32(bytes)
    }

    /// Reads the next byte
    #[cfg_attr(std, inline(always))]
    pub fn read_u8(&mut self, bytes: &[u8]) -> Result<u8> {
        let b = bytes.get(self.start).ok_or(Error::UnexpectedEndOfBuffer)?;
        self.start += 1;
        Ok(*b)
    }

    /// Reads the next varint encoded u64
    #[cfg_attr(std, inline(always))]
    pub fn read_varint32(&mut self, bytes: &[u8]) -> Result<u32> {
        let mut b = self.read_u8(bytes)?; // byte0
        if b & 0x80 == 0 {
            return Ok(b as u32);
        }
        let mut r = (b & 0x7f) as u32;

        b = self.read_u8(bytes)?; // byte1
        r |= ((b & 0x7f) as u32) << 7;
        if b & 0x80 == 0 {
            return Ok(r);
        }

        b = self.read_u8(bytes)?; // byte2
        r |= ((b & 0x7f) as u32) << 14;
        if b & 0x80 == 0 {
            return Ok(r);
        }

        b = self.read_u8(bytes)?; // byte3
        r |= ((b & 0x7f) as u32) << 21;
        if b & 0x80 == 0 {
            return Ok(r);
        }

        b = self.read_u8(bytes)?; // byte4
        r |= ((b & 0xf) as u32) << 28; // silently prevent overflow; only mask 0xF
        if b & 0x80 == 0 {
            // WARNING ABOUT TRUNCATION
            //
            // In this case, byte4 takes the form 0ZZZ_YYYY where:
            //     Y: part of the resulting 32-bit number
            //     Z: beyond 32 bits (excess bits,not used)
            //
            // If the Z bits were set, it might indicate that the number being
            // decoded was intended to be bigger than 32 bits, suggesting an
            // error somewhere else.
            //
            // However, for the sake of consistency with Google's own protobuf
            // implementation, and also to allow for any efficient use of those
            // extra bits by users if they wish (this crate is meant for speed
            // optimization anyway) we shall not check for this here.
            //
            // Therefore, THIS FUNCTION SIMPLY IGNORES THE EXTRA BITS, WHICH IS
            // ESSENTIALLY A SILENT TRUNCATION!
            return Ok(r);
        }

        // ANOTHER WARNING ABOUT TRUNCATION
        //
        // Again, we do not check whether the byte representation fits within 32
        // bits, and simply ignore extra bytes, CONSTITUTING A SILENT
        // TRUNCATION!
        //
        // Therefore, if the user wants this function to avoid ignoring any
        // bits/bytes, they need to ensure that the input is a varint
        // representing a value within EITHER u32 OR i32 range. Since at this
        // point we are beyond 5 bits, the only possible case is a negative i32
        // (since negative numbers are always 10 bytes in protobuf). We must
        // have exactly 5 bytes more to go.
        //
        // Since we know it must be a negative number, and this function is
        // meant to read 32-bit ints (there is a different function for reading
        // 64-bit ints), the user might want to take care to ensure that this
        // negative number is within valid i32 range, i.e. at least
        // -2,147,483,648. Otherwise, this function simply ignores the extra
        // bits, essentially constituting a silent truncation!
        //
        // What this means in the end is that the user should ensure that the
        // resulting number, once decoded from the varint format, takes such a
        // form:
        //
        // 11111111_11111111_11111111_11111111_1XXXXXXX_XXXXXXXX_XXXXXXXX_XXXXXXXX
        // ^(MSB bit 63)                       ^(bit 31 is set)                  ^(LSB bit 0)

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
    #[cfg_attr(std, inline(always))]
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

        // WARNING ABOUT TRUNCATION:
        //
        // For the number to be within valid 64 bit range, some conditions about
        // this last byte must be met:
        // 1. This must be the last byte (MSB not set)
        // 2. No 64-bit overflow (middle 6 bits are beyond 64 bits for the
        //    entire varint, so they cannot be set either)
        //
        // However, for the sake of consistency with Google's own protobuf
        // implementation, and also to allow for any efficient use of those
        // extra bits by users if they wish (this crate is meant for speed
        // optimization anyway) we shall not check for this here.
        //
        // Therefore, THIS FUNCTION SIMPLY IGNORES THE EXTRA BITS, WHICH IS
        // ESSENTIALLY A SILENT TRUNCATION!
        b = self.read_u8(bytes)?;
        r2 |= (b as u32) << 7;
        if b & 0x80 == 0 {
            return Ok((r0 as u64 | (r1 as u64) << 28) | (r2 as u64) << 56);
        }

        // cannot read more than 10 bytes
        Err(Error::Varint)
    }

    /// Reads int32 (varint)
    #[cfg_attr(std, inline)]
    pub fn read_int32(&mut self, bytes: &[u8]) -> Result<i32> {
        self.read_varint32(bytes).map(|i| i as i32)
    }

    /// Reads int64 (varint)
    #[cfg_attr(std, inline)]
    pub fn read_int64(&mut self, bytes: &[u8]) -> Result<i64> {
        self.read_varint64(bytes).map(|i| i as i64)
    }

    /// Reads uint32 (varint)
    #[cfg_attr(std, inline)]
    pub fn read_uint32(&mut self, bytes: &[u8]) -> Result<u32> {
        self.read_varint32(bytes)
    }

    /// Reads uint64 (varint)
    #[cfg_attr(std, inline)]
    pub fn read_uint64(&mut self, bytes: &[u8]) -> Result<u64> {
        self.read_varint64(bytes)
    }

    /// Reads sint32 (varint)
    #[cfg_attr(std, inline)]
    pub fn read_sint32(&mut self, bytes: &[u8]) -> Result<i32> {
        // zigzag
        let n = self.read_varint32(bytes)?;
        Ok(((n >> 1) as i32) ^ (-((n & 1) as i32)))
    }

    /// Reads sint64 (varint)
    #[cfg_attr(std, inline)]
    pub fn read_sint64(&mut self, bytes: &[u8]) -> Result<i64> {
        // zigzag
        let n = self.read_varint64(bytes)?;
        Ok(((n >> 1) as i64) ^ (-((n & 1) as i64)))
    }

    /// Reads fixed64 (little endian u64)
    #[cfg_attr(std, inline)]
    fn read_fixed<M, F: Fn(&[u8]) -> M>(&mut self, bytes: &[u8], len: usize, read: F) -> Result<M> {
        let v = read(
            bytes
                .get(self.start..self.start + len)
                .ok_or(Error::UnexpectedEndOfBuffer)?,
        );
        self.start += len;
        Ok(v)
    }

    /// Reads fixed64 (little endian u64)
    #[cfg_attr(std, inline)]
    pub fn read_fixed64(&mut self, bytes: &[u8]) -> Result<u64> {
        self.read_fixed(bytes, 8, LE::read_u64)
    }

    /// Reads fixed32 (little endian u32)
    #[cfg_attr(std, inline)]
    pub fn read_fixed32(&mut self, bytes: &[u8]) -> Result<u32> {
        self.read_fixed(bytes, 4, LE::read_u32)
    }

    /// Reads sfixed64 (little endian i64)
    #[cfg_attr(std, inline)]
    pub fn read_sfixed64(&mut self, bytes: &[u8]) -> Result<i64> {
        self.read_fixed(bytes, 8, LE::read_i64)
    }

    /// Reads sfixed32 (little endian i32)
    #[cfg_attr(std, inline)]
    pub fn read_sfixed32(&mut self, bytes: &[u8]) -> Result<i32> {
        self.read_fixed(bytes, 4, LE::read_i32)
    }

    /// Reads float (little endian f32)
    #[cfg_attr(std, inline)]
    pub fn read_float(&mut self, bytes: &[u8]) -> Result<f32> {
        self.read_fixed(bytes, 4, LE::read_f32)
    }

    /// Reads double (little endian f64)
    #[cfg_attr(std, inline)]
    pub fn read_double(&mut self, bytes: &[u8]) -> Result<f64> {
        self.read_fixed(bytes, 8, LE::read_f64)
    }

    /// Reads bool (varint, check if == 0)
    #[cfg_attr(std, inline)]
    pub fn read_bool(&mut self, bytes: &[u8]) -> Result<bool> {
        self.read_varint32(bytes).map(|i| i != 0)
    }

    /// Reads enum, encoded as i32
    #[cfg_attr(std, inline)]
    pub fn read_enum<E: From<i32>>(&mut self, bytes: &[u8]) -> Result<E> {
        self.read_int32(bytes).map(|e| e.into())
    }

    /// First reads a varint and use it as size to read a generic object
    #[cfg_attr(std, inline(always))]
    fn read_len_varint<'a, M, F>(&mut self, bytes: &'a [u8], read: F) -> Result<M>
    where
        F: FnMut(&mut BytesReader, &'a [u8]) -> Result<M>,
    {
        let len = self.read_varint32(bytes)? as usize;
        self.read_len(bytes, read, len)
    }

    /// Reads a certain number of bytes specified by len
    #[cfg_attr(std, inline(always))]
    fn read_len<'a, M, F>(&mut self, bytes: &'a [u8], mut read: F, len: usize) -> Result<M>
    where
        F: FnMut(&mut BytesReader, &'a [u8]) -> Result<M>,
    {
        let cur_end = self.end;
        self.end = self.start + len;
        let v = read(self, bytes)?;
        self.start = self.end;
        self.end = cur_end;
        Ok(v)
    }

    /// Reads bytes (Vec<u8>)
    #[cfg_attr(std, inline)]
    pub fn read_bytes<'a>(&mut self, bytes: &'a [u8]) -> Result<&'a [u8]> {
        self.read_len_varint(bytes, |r, b| {
            b.get(r.start..r.end).ok_or(Error::UnexpectedEndOfBuffer)
        })
    }

    /// Reads string (String)
    #[cfg_attr(std, inline)]
    pub fn read_string<'a>(&mut self, bytes: &'a [u8]) -> Result<&'a str> {
        self.read_len_varint(bytes, |r, b| {
            b.get(r.start..r.end)
                .ok_or(Error::UnexpectedEndOfBuffer)
                .and_then(|x| ::core::str::from_utf8(x).map_err(|e| e.into()))
        })
    }

    /// Reads packed repeated field (Vec<M>)
    ///
    /// Note: packed field are stored as a variable length chunk of data, while regular repeated
    /// fields behaves like an iterator, yielding their tag everytime
    #[cfg_attr(std, inline)]
    pub fn read_packed<'a, M, F>(&mut self, bytes: &'a [u8], mut read: F) -> Result<Vec<M>>
    where
        F: FnMut(&mut BytesReader, &'a [u8]) -> Result<M>,
    {
        self.read_len_varint(bytes, |r, b| {
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
    #[cfg_attr(std, inline)]
    pub fn read_packed_fixed<'a, M: Copy + PartialEq>(
        &mut self,
        bytes: &'a [u8],
    ) -> Result<PackedFixed<'a, M>>
    where
        [M]: ToOwned,
    {
        let len = self.read_varint32(bytes)? as usize;
        if self.len() < len {
            return Err(Error::UnexpectedEndOfBuffer);
        }

        // Note the floor divide; we rely on this to guarantee
        // correctness in the rest of this function
        let n = len / ::core::mem::size_of::<M>();
        let target = &bytes[self.start..self.start + (n * ::core::mem::size_of::<M>())];

        self.start += len;
        Ok(PackedFixed::from(target))
    }

    /// Reads a nested message
    ///
    /// First reads a varint and interprets it as the length of the message
    #[cfg_attr(std, inline)]
    pub fn read_message<'a, M>(&mut self, bytes: &'a [u8]) -> Result<M>
    where
        M: MessageRead<'a>,
    {
        self.read_len_varint(bytes, M::from_reader)
    }

    /// Reads a nested message
    ///
    /// Reads just the message and does not try to read it's size first.
    ///  * 'len' - The length of the message to be read.
    #[cfg_attr(std, inline)]
    pub fn read_message_by_len<'a, M>(&mut self, bytes: &'a [u8], len: usize) -> Result<M>
    where
        M: MessageRead<'a>,
    {
        self.read_len(bytes, M::from_reader, len)
    }

    /// Reads a map item: (key, value)
    #[cfg_attr(std, inline)]
    pub fn read_map<'a, K, V, F, G>(
        &mut self,
        bytes: &'a [u8],
        mut read_key: F,
        mut read_val: G,
    ) -> Result<(K, V)>
    where
        F: FnMut(&mut BytesReader, &'a [u8]) -> Result<K>,
        G: FnMut(&mut BytesReader, &'a [u8]) -> Result<V>,
        K: ::core::fmt::Debug + Default,
        V: ::core::fmt::Debug + Default,
    {
        self.read_len_varint(bytes, |r, bytes| {
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
    #[cfg_attr(std, inline)]
    pub fn read_unknown(&mut self, bytes: &[u8], tag_value: u32) -> Result<()> {
        // Since `read.varint64()` calls `read_u8()`, which increments
        // `self.start`, we don't need to manually increment `self.start` in
        // control flows that either call `read_varint64()` or error out.
        let offset = match (tag_value & 0x7) as u8 {
            WIRE_TYPE_VARINT => {
                self.read_varint64(bytes)?;
                return Ok(());
            }
            WIRE_TYPE_FIXED64 => 8,
            WIRE_TYPE_FIXED32 => 4,
            WIRE_TYPE_LENGTH_DELIMITED => {
                usize::try_from(self.read_varint64(bytes)?).map_err(|_| Error::Varint)?
            }
            WIRE_TYPE_START_GROUP | WIRE_TYPE_END_GROUP => {
                return Err(Error::Deprecated("group"));
            }
            t => {
                return Err(Error::UnknownWireType(t));
            }
        };

        // Meant to prevent overflowing. Comparison used is *strictly* lesser
        // since `self.end` is given by `len()`; i.e. `self.end` is 1 more than
        // highest index
        if self.end - self.start < offset {
            Err(Error::Varint)
        } else {
            self.start += offset;
            Ok(())
        }
    }

    /// Gets the remaining length of bytes not read yet
    #[cfg_attr(std, inline(always))]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Checks if `self.len == 0`
    #[cfg_attr(std, inline(always))]
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
///     let mut reader =
///         Reader::from_file("/path/to/binary/protobuf.bin").expect("Cannot read input file");
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
///     let foobar: FooBar = reader
///         .read(|r, b| r.read_message(b))
///         .expect("Cannot read FooBar message");
///     // Reader::read_message uses `FooBar::from_reader` internally through the `MessageRead`
///     // trait.
///
///     println!(
///         "Found {} foos and {} bars!",
///         foobar.foos.len(),
///         foobar.bars.len()
///     );
/// }
/// ```
pub struct Reader {
    buffer: Vec<u8>,
    inner: BytesReader,
}

impl Reader {
    /// Creates a new `Reader`
    #[cfg(feature = "std")]
    #[allow(clippy::uninit_vec)]
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
    #[cfg(feature = "std")]
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
    #[cfg_attr(std, inline)]
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

/// Deserialize a `MessageRead from a `&[u8]`
pub fn deserialize_from_slice<'a, M: MessageRead<'a>>(bytes: &'a [u8]) -> Result<M> {
    let mut reader = BytesReader::from_bytes(bytes);
    reader.read_message::<M>(bytes)
}

/// Wrapper enum over packed fixed data, similar to `Cow`.
///
/// When we read packed fixed data, the raw bytes are often misaligned to the
/// data type they represent. We don't want to have to align all the data before
/// reading (especially if we're only accessing a few elements), as this
/// involves lengthy allocations. This enum provides the `Borrowed` variant as a
/// wrapper for such data, holding a reference to the (possibly) misaligned raw
/// bytes and providing methods to read and iterate that are alignment-safe.
///
/// However, it is also convenient for the user to be able to use a
/// `PackedFixed` variant that owns its own data (perhaps when setting the data
/// themselves). It is mainly for this reason that the `Owned` variant is
/// provided, which owns a `Vec<T>`.
///
/// One implementation detail is that the `Owned` variant is always aligned, so
/// no use of `read_unaligned` is necessary. Methods are provided to convert
/// from `Borrowed` to `Owned`, if it is found that it helps compiler
/// optimization (not fully benchmarked at time of writing, seems
/// temperamental).
#[derive(Debug, Clone, Default)]
pub enum PackedFixed<'a, T: Copy + PartialEq> {
    /// Default when no data has been received yet; e.g. when just initialized.
    #[default]
    NoDataYet,
    /// Variant that carries a reference to raw bytes that may or may not be
    /// aligned, representing a packed set of fixed numbers.
    ///
    /// `PackedFixed` methods called on `Borrowed` will use `read_unaligned()`
    /// to interact with the data without copying all bytes to an aligned buffer
    /// in order to avoid delay from that memory allocation. So far, I can't
    /// think of any way to take advantage of the situations when it is
    /// coincidentally aligned.
    Borrowed(&'a [u8]),
    /// Variant that contains an owned vector of numbers.
    Owned(Vec<T>),
}

impl<'a, T: Copy + PartialEq> PackedFixed<'a, T> {
    /// Return the length of the DATA (not the bytes).
    pub fn len(&self) -> usize {
        match self {
            PackedFixed::Borrowed(bytes) => bytes.len() / ::core::mem::size_of::<T>(),
            PackedFixed::Owned(v) => v.len(),
            PackedFixed::NoDataYet => 0,
        }
    }

    /// Mutate in place to `Owned` variant. In the case of `Borrowed`, this
    /// performs a bitwise copy of the entire slice.
    pub fn own(&mut self) {
        match self {
            PackedFixed::NoDataYet => *self = PackedFixed::Owned(Vec::new()),
            PackedFixed::Borrowed(_) => {
                *self = self.make_owned_variant_from_unaligned_buf();
            }
            PackedFixed::Owned(_) => {} // no-op for PackedFixed::Owned, just like Cow
        }
    }

    /// Get a `Vec<T>` of the internal data, moving `self` in the process. The
    /// reason we move `self` is so that calling this on an `Owned` variant
    /// will not require copying data. `Borrowed` variants will trigger a
    /// bitwise copy.
    ///
    /// It would be really nice if this could instead return `&[T]` without
    /// moving `self`, but we can't do this for the `Borrowed` variant, so
    /// we have no such method on `PackedFixed` as a whole. And anyway, this is
    /// what `at()` on `Borrowed` is for.
    pub fn into_vec(self) -> Vec<T> {
        match self {
            PackedFixed::NoDataYet => Vec::new(),
            PackedFixed::Borrowed(_) => self.make_vec_from_unaligned_buf(),
            PackedFixed::Owned(v) => v,
        }
    }

    /// Get the element at index `index`.
    ///
    /// Note that `index` refers to the index of the type `T`, and NOT the byte
    /// index. In the case of `Borrowed`, this index is calculated during
    /// runtime, as if the underlying data was already in form `Vec<T>`.
    pub fn at(&self, index: usize) -> T {
        match self {
            PackedFixed::Borrowed(bytes) => {
                let byte_offset = index * core::mem::size_of::<T>();
                if byte_offset >= bytes.len() {
                    panic!("PackedFixed::at(): Index out of range!");
                }

                let mut ptr = bytes.as_ptr();
                unsafe {
                    ptr = ptr.add(byte_offset);
                    (ptr as *const T).read_unaligned()
                }
            }
            PackedFixed::Owned(v) => v[index],
            PackedFixed::NoDataYet => panic!("Cannot call at() on PackedFixed::NoDataYet!"),
        }
    }

    /// Mutate `self` to `Owned` variant before returning immutable slice
    pub fn to_slice(&mut self) -> &[T] {
        self.own();
        if let PackedFixed::Owned(ref contents) = *self {
            contents
        } else {
            unreachable!();
        }
    }

    /// Mutate `self` to `Owned` variant before returning mutable slice
    pub fn to_mut_slice(&mut self) -> &mut [T] {
        self.own();
        if let PackedFixed::Owned(ref mut contents) = *self {
            contents
        } else {
            unreachable!();
        }
    }

    /// Returns `true` if no data is contained in the enum.
    pub fn is_empty(&self) -> bool {
        match self {
            PackedFixed::NoDataYet => true,
            PackedFixed::Borrowed(bytes) => bytes.is_empty(),
            PackedFixed::Owned(contents) => contents.is_empty(),
        }
    }

    // This method is private and mainly to avoid repetition in code.
    fn make_vec_from_unaligned_buf(&self) -> Vec<T> {
        match &self {
            PackedFixed::Borrowed(bytes) => unsafe {
                let src = bytes.as_ptr();
                let mut buf = Vec::<T>::with_capacity(self.len());
                let dst = buf.as_mut_ptr() as *mut u8;
                ::core::ptr::copy(src, dst, bytes.len()); // careful to use length in bytes here
                buf.set_len(self.len());
                buf
            },
            _ => unreachable!(),
        }
    }

    // This method is private and mainly to avoid repetition in code.
    fn make_owned_variant_from_unaligned_buf(&self) -> Self {
        match &self {
            PackedFixed::Borrowed(_) => PackedFixed::Owned(self.make_vec_from_unaligned_buf()),
            _ => unreachable!(),
        }
    }
}

/// Iterator over `PackedFixed`.
pub struct PackedFixedIntoIter<'a, T: Copy + PartialEq> {
    packed_fixed: PackedFixed<'a, T>,
    index: usize,
}

impl<'a, T: Copy + PartialEq> FusedIterator for PackedFixedIntoIter<'a, T> {}

impl<'a, T: Copy + PartialEq> Iterator for PackedFixedIntoIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.packed_fixed.len() {
            None
        } else {
            let res = Some(self.packed_fixed.at(self.index));
            self.index += 1;
            res
        }
    }
}

impl<'a, T: Copy + PartialEq> IntoIterator for PackedFixed<'a, T> {
    type Item = T;

    type IntoIter = PackedFixedIntoIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            packed_fixed: self,
            index: 0,
        }
    }
}

/// Iterator over `&'a PackedFixed`. Note: This does NOT return references to
/// the iterated elements (which is why we aren't following the convention of
/// calling it `PackedFixedIter`), because:
/// - Due to limitations of `read_unaligned()` (must always copy), we cannot get
///   references to elements of the `Borrowed` variant
/// - The only data types expected to be handled by `PackedFixed` are primitive
///   numeral types anyway.
///
/// This is purely for convenience, so we can iterate over `&PackedFixed`
/// without moving it.
pub struct PackedFixedRefIter<'a, T: Copy + PartialEq> {
    packed_fixed: &'a PackedFixed<'a, T>,
    index: usize,
}

impl<'a, T: Copy + PartialEq> FusedIterator for PackedFixedRefIter<'a, T> {}

impl<'a, T: Copy + PartialEq> Iterator for PackedFixedRefIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.packed_fixed.len() {
            None
        } else {
            let res = Some(self.packed_fixed.at(self.index));
            self.index += 1;
            res
        }
    }
}

impl<'a, T: Copy + PartialEq> IntoIterator for &'a PackedFixed<'a, T> {
    type Item = T;

    type IntoIter = PackedFixedRefIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            packed_fixed: self,
            index: 0,
        }
    }
}

impl<'a, T: Copy + PartialEq, const N: usize> From<&'a [u8; N]> for PackedFixed<'a, T> {
    fn from(value: &'a [u8; N]) -> Self {
        Self::Borrowed(value)
    }
}

impl<'a, T: Copy + PartialEq> From<&'a [u8]> for PackedFixed<'a, T> {
    fn from(value: &'a [u8]) -> Self {
        Self::Borrowed(value)
    }
}

impl<'a, T: Copy + PartialEq> From<&'a Vec<u8>> for PackedFixed<'a, T> {
    fn from(value: &'a Vec<u8>) -> Self {
        Self::Borrowed(value)
    }
}

impl<'a, T: Copy + PartialEq> From<Vec<T>> for PackedFixed<'a, T> {
    fn from(value: Vec<T>) -> Self {
        Self::Owned(value)
    }
}

impl<'a, T: Copy + PartialEq> PartialEq for PackedFixed<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.into_iter().eq(other)
    }
}

#[test]
fn test_varint() {
    let data = [0x96, 0x01];
    let mut r = BytesReader::from_bytes(&data[..]);
    assert_eq!(150, r.read_varint32(&data[..]).unwrap());
    assert!(r.is_eof());
}

#[test]
fn read_size_overflowing_unknown() {
    let bytes = &[
        200, 250, 35, // varint tag with WIRE_TYPE_VARINT -- 589128
        //
        //
        47, // varint itself
        //
        //
        250, 36, // varint tag with WIRE_TYPE_LENGTH_DELIMITED -- 4730
        //
        //
        255, 255, 255, 255, 255, 255, 255, 255, 255, 3, // huge 10-byte length
        //
        //
        255, 255, 227, // unused extra bytes
    ];

    let mut r = BytesReader::from_bytes(bytes);

    assert!(!r.is_eof());
    assert_eq!(r.next_tag(bytes).unwrap(), 589128);
    r.read_unknown(bytes, 589128).unwrap();

    assert!(!r.is_eof());
    assert_eq!(r.next_tag(bytes).unwrap(), 4730);
    let e = r.read_unknown(bytes, 4730).unwrap_err();

    assert!(matches!(e, Error::Varint), "{:?}", e);
}

#[test]
fn test_packed_fixed_iter() {
    let pf: PackedFixed<i32> = vec![1, 2, 3, 4, 5].into();

    let mut total = 0;

    for _ in 0..10 {
        for i in &pf {
            total += i;
        }
    }

    for i in pf {
        total += i;
    }

    assert_eq!(total, (1 + 2 + 3 + 4 + 5) * (10 + 1));
}

#[test]
fn test_packed_fixed_eq() {
    let v = vec![
        0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x02u8, 0x00u8, 0x00u8, 0x00u8, 0x03u8, 0x00u8, 0x00u8,
        0x00u8,
    ];
    let borrowed: PackedFixed<i32> = PackedFixed::Borrowed(&v);
    let mut owned: PackedFixed<i32> = borrowed.clone();
    owned.own();

    let owned_reversed: PackedFixed<i32> = vec![3, 2, 1].into();

    let v_reversed = vec![
        0x03u8, 0x00u8, 0x00u8, 0x00u8, 0x02u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8,
        0x00u8,
    ];
    let borrowed_reversed: PackedFixed<i32> = PackedFixed::Borrowed(&v_reversed);

    let ndy: PackedFixed<i32> = PackedFixed::NoDataYet;
    let ndy2: PackedFixed<i32> = PackedFixed::NoDataYet;
    let def: PackedFixed<i32> = PackedFixed::default();

    assert_eq!(borrowed, owned);
    assert_eq!(borrowed_reversed, owned_reversed);
    assert_eq!(ndy, ndy2);
    assert_eq!(ndy, def);

    assert_ne!(borrowed, owned_reversed);
    assert_ne!(owned, owned_reversed);
    assert_ne!(owned, borrowed_reversed);
    assert_ne!(borrowed, borrowed_reversed);
}
