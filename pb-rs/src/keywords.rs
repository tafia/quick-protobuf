/**
 * This list includes:
 *   1. Rust keywords (for, match, etc.)
 *   2. Rust types used in code generation (i32, Cow, etc.)
 *   3. Imported objects/traits (MessageInfo, TryFrom, etc.)
 *   4. Derived traits (Debug, Clone, etc.)
 *   5. Associated types of traits (Target, Error, etc.)
 *   6. Libraries/Crates (std, quick_protobuf, etc.)
 *   7. Common/Reserved Rust file/folder names (main, lib, etc.)
 *   8. Names of functions that generated structs can have (new, try_from, etc.)
 *   9. Names of parameters for those functions (r, bytes, etc.)
 *   10. Names of local variables within those functions (msg, f, etc.)
 *   11. Names of closure variables within those functions (e, t, etc.)
 */
static RUST_KEYWORDS: [&str; 121] = [
    "abstract",
    "alignof",
    "alloc",
    "as",
    "become",
    "bin",
    "bool",
    "box",
    "Box",
    "break",
    "buf",
    "bytes",
    "BytesReader",
    "Clone",
    "core",
    "const",
    "continue",
    "convert",
    "crate",
    "Cow",
    "Debug",
    "Default",
    "deref",
    "Deref",
    "DerefMut",
    "deref_mut",
    "do",
    "e",
    "else",
    "enum",
    "Err",
    "Error",
    "extern",
    "f",
    "f32",
    "f64",
    "false",
    "final",
    "fmt",
    "from",
    "from_reader",
    "fn",
    "for",
    "get_size",
    "HashMap",
    "i32",
    "i64",
    "if",
    "impl",
    "in",
    "inner",
    "let",
    "lib",
    "loop",
    "m",
    "main",
    "match",
    "mod",
    "move",
    "msg",
    "mut",
    "new",
    "None",
    "MessageInfo",
    "MessageRead",
    "MessageWrite",
    "offsetof",
    "Ok",
    "Option",
    "ops",
    "override",
    "PartialEq",
    "pinned",
    "priv",
    "proto",
    "pub",
    "pure",
    "quick_protobuf",
    "r",
    "reader",
    "ref",
    "Result",
    "return",
    "s",
    "self",
    "Self",
    "sizeof",
    "Some",
    "src",
    "static",
    "std",
    "str",
    "String",
    "struct",
    "super",
    "t",
    "Target",
    "trait",
    "true",
    "try_from",
    "try_into",
    "type",
    "typeof",
    "u8",
    "u32",
    "u64",
    "unsafe",
    "unsized",
    "use",
    "Vec",
    "virtual",
    "w",
    "where",
    "while",
    "Write",
    "writer",
    "Writer",
    "WriterBackend",
    "write_message",
    "yield",
    "_pin",
];

/// Check if the identifier is a Rust keyword and appends a `_pb` suffix if that's the case
pub fn sanitize_keyword(ident: &mut String) {
    if !ident.contains('.') && RUST_KEYWORDS.contains(&&**ident) {
        ident.push_str("_pb");
    } else {
        *ident = ident
            .split('.')
            .map(|s| {
                if RUST_KEYWORDS.contains(&s) {
                    format!("{}_pb", s)
                } else {
                    s.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(".");
    }
}
