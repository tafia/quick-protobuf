use std::io;

/// An error enum
#[derive(Debug)]
pub enum Error {
    /// IO error
    Io(io::Error),
    /// Nom Error
    Nom(::nom::simple_errors::Err),
    /// No .proto file provided
    NoProto,
    /// Cannot read input file
    InputFile(String),
    /// Cannot read output file
    OutputFile(String),
    /// Cannot read output directory
    OutputDirectory(String),
    /// Multiple input files with `--output` argument
    OutputMultipleInputs,
    /// Invalid message
    InvalidMessage(String),
    /// Varint decoding error
    InvalidImport(String),
    /// Empty read
    EmptyRead,
    /// Enum or message not found
    MessageOrEnumNotFound(String),
    /// Invalid default enum
    InvalidDefaultEnum(String),
    /// Missing `read_fn` implementation for maps
    ReadFnMap,
    /// Cycle detected
    Cycle(Vec<String>),
    /// `--output` and `--output_directory` both used
    OutputAndOutputDir,
}

/// A wrapper for `Result<T, Error>`
pub type Result<T> = ::std::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Nom(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::Io(e) => write!(f, "{}", e),
            Error::Nom(e) => write!(f, "{}", e),
            Error::NoProto => write!(f, "No .proto file provided"),
            Error::InputFile(file) => write!(f, "Cannot read input file '{}'", file),
            Error::OutputFile(file) => write!(f, "Cannot read output file '{}'", file),
            Error::OutputDirectory(dir) => write!(f, "Cannot read output directory '{}'", dir),
            Error::OutputMultipleInputs => write!(f, "--output only allowed for single input file"),
            Error::InvalidMessage(msg) => write!(
                f,
                "Message checks errored: {}\r\n\
                Proto definition might be invalid or something got wrong in the parsing",
                msg
            ),
            Error::InvalidImport(imp) => write!(
                f,
                "Cannot convert protobuf import into module import:: {}\r\n\
                Import definition might be invalid, some characters may not be supported",
                imp
            ),
            Error::EmptyRead => write!(
                f,
                "No message or enum were read;\
                either definition might be invalid or there were only unsupported structures"
            ),
            Error::MessageOrEnumNotFound(me) => write!(f, "Could not find message or enum {}", me),
            Error::InvalidDefaultEnum(en) => write!(
                f,
                "Enum field cannot be set to '{}', this variant does not exist",
                en
            ),
            Error::ReadFnMap => write!(f, "There should be a special case for maps"),
            Error::Cycle(msgs) => write!(
                f,
                "Messages {:?} are cyclic (missing an optional field)",
                msgs
            ),
            Error::OutputAndOutputDir => {
                write!(f, "only one of --output or --output_directory allowed")
            }
        }
    }
}
