//! A module to handle all errors via error-chain crate

use types::Tag;

error_chain! {
//     links {
//         I2R(::i2r::errors::Error, ::i2r::errors::ErrorKind);
//     }
    foreign_links {
        Io(::std::io::Error);
        Utf8(::std::string::FromUtf8Error);
//         ParseInt(::std::num::ParseIntError);
    }
    errors {
        Deprecated(feat: &'static str) {
            description("deprecated feature")
            display("feature '{}' has been deprecated", feat)
        }
        UnknownWireType {
            description("unknown wire type")
        }
        Varint {
            description("cannot decode varint")
        }
        Eof {
            description("unexpected end of file")
        }
        InvalidMessage(tag: Tag, field_type: &'static str) {
            description("invalid message field")
            display("expecting '{}', got tag {:?}", field_type, tag)
        }
//         Unexpected(t: String) {
//             description("unexpected error")
//             display("unexpected error: '{}'", t)
//         }
    }
}
