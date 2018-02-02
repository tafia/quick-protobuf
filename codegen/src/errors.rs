//! A module to handle all errors via error-chain crate

use std::io;

/// An error enum which derives `Fail`
#[derive(Debug, Fail)]
pub enum Error {
    /// Io error
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    /// Nom Error
    #[fail(display = "{}", _0)]
    Nom(#[cause] ::nom::simple_errors::Err),

    // No .proto file provided
    #[fail(display = "No .proto file provided")]
    NoProto,
    /// Input file
    #[fail(display = "Cannot read input file '{}'", _0)]
    InputFile(String),
    /// Output file
    #[fail(display = "Cannot read output file '{}'", _0)]
    OutputFile(String),
    /// Output file
    #[fail(display = "Cannot read output directory '{}'", _0)]
    OutputDirectory(String),
    /// Multiple input files with --output argument
    #[fail(display = "--output only allowed for single input file")]
    OutputMultipleInputs,
    /// Invalid message
    #[fail(display = "Message checks errored: {}\r\n\
    Proto definition might be invalid or something got wrong in the parsing", _0)]
    InvalidMessage(String),
    /// Varint decoding error
    #[fail(display = "Cannot convert protobuf import into module import:: {}\r\n\
    Import definition might be invalid, some characters may not be supported", _0)]
    InvalidImport(String),
    /// Empty read
    #[fail(display = "No message or enum were read;\
    either definition might be invalid or there were only unsupported structures")]
    EmptyRead,
    /// enum not found

    /// Enum not found
    #[fail(display = "Could not find enum {}", _0)]
    EnumNotFound(String),
    /// Enum not found
    #[fail(display = "Could not find message {}", _0)]
    MessageNotFound(String),
    /// read_fn implementation for Maps
    #[fail(display = "There should be a special case for maps")]
    ReadFnMap,
    
}

/// A wrapper for `Result<T, Error>`
pub type Result<T> = ::std::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
