//! A module to handle all errors via failure crate

#[cfg(feature = "std")]
use std::io;

/// An error enum which derives `Fail`
#[derive(Debug, failure_derive::Fail)]
pub enum Error {
    /// Io error
    #[cfg(feature = "std")]
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    /// Utf8 Error
    #[fail(display = "{}", _0)]
    Utf8(::core::str::Utf8Error),

    /// Deprecated feature (in protocol buffer specification)
    #[fail(display = "Feature '{}' has been deprecated", _0)]
    Deprecated(&'static str),

    /// Unknown wire type
    #[fail(display = "Unknown wire type '{}', must be less than 6", _0)]
    UnknownWireType(u8),

    /// Varint decoding error
    #[fail(display = "Cannot decode varint")]
    Varint,

    /// Error while parsing protocol buffer message
    #[cfg(feature = "std")]
    #[fail(display = "Error while parsing message: {}", _0)]
    Message(String),

    /// Unexpected map tag
    #[fail(display = "Unexpected map tag: '{}', expecting 1 or 2", _0)]
    Map(u8),

    /// Out of data when reading from or writing to a byte buffer
    #[fail(display = "Unexpected end of buffer")]
    UnexpectedEndOfBuffer,

    /// The supplied output buffer is not large enough to serialize the message
    #[fail(display = "Output buffer too small")]
    OutputBufferTooSmall,
}

/// A wrapper for `Result<T, Error>`
pub type Result<T> = ::core::result::Result<T, Error>;

#[cfg(feature = "std")]
impl Into<io::Error> for Error {
    fn into(self) -> ::std::io::Error {
        use failure::Fail;
        match self {
            Error::Io(x) => x,
            Error::Utf8(x) => io::Error::new(io::ErrorKind::InvalidData, x),
            x => io::Error::new(io::ErrorKind::Other, x.compat()),
        }
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<::core::str::Utf8Error> for Error {
    fn from(e: ::core::str::Utf8Error) -> Error {
        Error::Utf8(e)
    }
}
