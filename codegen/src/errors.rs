//! A module to handle all errors via error-chain crate

#![allow(missing_docs)]

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Nom(::nom::simple_errors::Err);
    }
    errors {
        OutputFile(out: ::std::path::PathBuf) {
            description("output file name is invalid")
            display("cannot read output file name {}", out.display())
        }
        InvalidMessage(desc: String) {
            description("message if invalid")
            display("message checks errored: {}", desc)
            cause("proto definition might be invalid or something got wrong in the parsing")
        }
        InvalidImport(desc: String) {
            description("import is not supported")
            display("cannot convert protobuf import into module import: {}", desc)
            cause("import definition might be invalid, some characters may not be supported")
        }
        EmptyRead {
            description("No messages or enums were read; either there was no input or there were only unsupported structures")
        }
    }
}
