//! A module to handle all errors via error-chain crate

#![allow(missing_docs)]

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Nom(::nom::simple_errors::Err);
    }
    errors {
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
        InvalidWireType(wire_type: u32) {
            description("invalid wire type value")
            display("expecting a value between 0 and 5, got {}", wire_type)
        }
    }
}
