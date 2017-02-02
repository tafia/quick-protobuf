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
    }
}
