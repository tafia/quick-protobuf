//! A module to handle all errors via error-chain crate

#![allow(missing_docs)]

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Utf8(::std::string::FromUtf8Error);
        StrUtf8(::std::str::Utf8Error);
    }
    errors {
        Deprecated(feat: &'static str) {
            description("deprecated feature")
            display("feature '{}' has been deprecated", feat)
        }
        UnknownWireType(t: u8) {
            description("unknown wire type")
            display("wire type must be less than 6, found {}", t)
        }
        Varint {
            description("cannot decode varint")
        }
        Eof {
            description("unexpected end of file")
        }
        ParseMessage(s: String) {
            description("error while parsing message")
            display("error while parsing message: {}", s)

        }
    }
}
