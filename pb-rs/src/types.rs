use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use log::{debug, warn};

use crate::errors::{Error, Result};
use crate::keywords::sanitize_keyword;
use crate::parser::file_descriptor;

fn sizeof_varint(v: u32) -> usize {
    match v {
        0x0..=0x7F => 1,
        0x80..=0x3FFF => 2,
        0x4000..=0x1F_FFFF => 3,
        0x20_0000..=0xFFF_FFFF => 4,
        _ => 5,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syntax {
    Proto2,
    Proto3,
}

impl Default for Syntax {
    fn default() -> Syntax {
        Syntax::Proto2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frequency {
    Proto2Frequency(Proto2Frequency),
    Proto3Frequency(Proto3Frequency),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Proto2Frequency {
    Optional,
    Repeated,
    Required,
    Map,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Proto3Frequency {
    Optional,
    Repeated,
    Default,
    Map,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedType {
    SingularType,
    ArrayLikeType,
    Map,
}

impl Frequency {
    pub fn is_map(&self) -> bool {
        matches!(
            self,
            Frequency::Proto2Frequency(Proto2Frequency::Map)
                | Frequency::Proto3Frequency(Proto3Frequency::Map)
        )
    }

    pub fn is_optional(&self) -> bool {
        matches!(
            self,
            Frequency::Proto2Frequency(Proto2Frequency::Optional)
                | Frequency::Proto3Frequency(Proto3Frequency::Optional)
        )
    }

    pub fn is_repeated(&self) -> bool {
        matches!(
            self,
            Frequency::Proto2Frequency(Proto2Frequency::Repeated)
                | Frequency::Proto3Frequency(Proto3Frequency::Repeated)
        )
    }
}

impl From<Frequency> for GeneratedType {
    fn from(value: Frequency) -> Self {
        if value.is_map() {
            GeneratedType::Map
        } else if value.is_repeated() {
            GeneratedType::ArrayLikeType
        } else {
            GeneratedType::SingularType
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct MessageIndex {
    indexes: Vec<usize>,
}

impl fmt::Debug for MessageIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        f.debug_set().entries(self.indexes.iter()).finish()
    }
}

impl MessageIndex {
    pub fn get_message<'a>(&self, desc: &'a FileDescriptor) -> &'a Message {
        let first_message = self.indexes.first().and_then(|i| desc.messages.get(*i));
        self.indexes
            .iter()
            .skip(1)
            .fold(first_message, |cur, next| {
                cur.and_then(|msg| msg.messages.get(*next))
            })
            .expect("Message index not found")
    }

    fn get_message_mut<'a>(&self, desc: &'a mut FileDescriptor) -> &'a mut Message {
        let first_message = self
            .indexes
            .first()
            .and_then(move |i| desc.messages.get_mut(*i));
        self.indexes
            .iter()
            .skip(1)
            .fold(first_message, |cur, next| {
                cur.and_then(|msg| msg.messages.get_mut(*next))
            })
            .expect("Message index not found")
    }

    fn push(&mut self, i: usize) {
        self.indexes.push(i);
    }

    fn pop(&mut self) {
        self.indexes.pop();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct EnumIndex {
    msg_index: MessageIndex,
    index: usize,
}

impl EnumIndex {
    pub fn get_enum<'a>(&self, desc: &'a FileDescriptor) -> &'a Enumerator {
        let enums = if self.msg_index.indexes.is_empty() {
            &desc.enums
        } else {
            &self.msg_index.get_message(desc).enums
        };
        enums.get(self.index).expect("Enum index not found")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldType {
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Bool,
    Enum(EnumIndex),
    Fixed64,
    Sfixed64,
    Double,
    StringCow,
    BytesCow,
    String_,
    Bytes_,
    Message(MessageIndex),
    MessageOrEnum(String),
    Fixed32,
    Sfixed32,
    Float,
    Map(Box<FieldType>, Box<FieldType>),
}

impl FieldType {
    fn is_cow(&self) -> bool {
        matches!(self, FieldType::StringCow | FieldType::BytesCow,)
    }

    fn is_non_cow_string_or_byte(&self) -> bool {
        matches!(self, FieldType::String_ | FieldType::Bytes_,)
    }

    // Some writer functions (such as `write_string()`, etc.) take references to
    // their arguments, and some take the argument type itself (when it's a
    // primitive type, like `i32`). This means that our codegen sometimes
    // sometimes needs to dereference an argument to avoid passing a reference
    // when we should not. This function allows us to check for this case.
    fn need_to_dereference(&self) -> bool {
        matches!(
            self,
            FieldType::Enum(_)
                | FieldType::Int32
                | FieldType::Sint32
                | FieldType::Int64
                | FieldType::Sint64
                | FieldType::Uint32
                | FieldType::Uint64
                | FieldType::Bool
                | FieldType::Fixed64
                | FieldType::Sfixed64
                | FieldType::Double
                | FieldType::Fixed32
                | FieldType::Sfixed32
                | FieldType::Float,
        )
    }

    pub fn is_primitive(&self) -> bool {
        !matches!(
            *self,
            FieldType::Message(_)
                | FieldType::Map(_, _)
                | FieldType::StringCow
                | FieldType::BytesCow
                | FieldType::String_
                | FieldType::Bytes_
        )
    }

    fn has_cow(&self) -> bool {
        match *self {
            FieldType::BytesCow | FieldType::StringCow => true,
            FieldType::Map(ref k, ref v) => k.has_cow() || v.has_cow(),
            _ => false,
        }
    }

    fn has_bytes_and_string(&self) -> bool {
        matches!(*self, FieldType::Bytes_ | FieldType::String_)
    }

    fn is_map(&self) -> bool {
        matches!(*self, FieldType::Map(_, _))
    }

    fn wire_type_num(&self, packed: bool) -> u32 {
        if packed {
            2
        } else {
            self.wire_type_num_non_packed()
        }
    }

    fn wire_type_num_non_packed(&self) -> u32 {
        /*
        0	Varint	int32, int64, uint32, uint64, sint32, sint64, bool, enum
        1	64-bit	fixed64, sfixed64, double
        2	Length-delimited	string, bytes, embedded messages, packed repeated fields
        3	Start group	groups (deprecated)
        4	End group	groups (deprecated)
        5	32-bit	fixed32, sfixed32, float
        */
        match *self {
            FieldType::Int32
            | FieldType::Sint32
            | FieldType::Int64
            | FieldType::Sint64
            | FieldType::Uint32
            | FieldType::Uint64
            | FieldType::Bool
            | FieldType::Enum(_) => 0,
            FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double => 1,
            FieldType::StringCow
            | FieldType::BytesCow
            | FieldType::String_
            | FieldType::Bytes_
            | FieldType::Message(_)
            | FieldType::Map(_, _) => 2,
            FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => 5,
            FieldType::MessageOrEnum(_) => unreachable!("Message / Enum not resolved"),
        }
    }

    fn proto_type(&self) -> &str {
        match *self {
            FieldType::Int32 => "int32",
            FieldType::Sint32 => "sint32",
            FieldType::Int64 => "int64",
            FieldType::Sint64 => "sint64",
            FieldType::Uint32 => "uint32",
            FieldType::Uint64 => "uint64",
            FieldType::Bool => "bool",
            FieldType::Enum(_) => "enum",
            FieldType::Fixed32 => "fixed32",
            FieldType::Sfixed32 => "sfixed32",
            FieldType::Float => "float",
            FieldType::Fixed64 => "fixed64",
            FieldType::Sfixed64 => "sfixed64",
            FieldType::Double => "double",
            FieldType::String_ => "string",
            FieldType::Bytes_ => "bytes",
            FieldType::StringCow => "string",
            FieldType::BytesCow => "bytes",
            FieldType::Message(_) => "message",
            FieldType::Map(_, _) => "map",
            FieldType::MessageOrEnum(_) => unreachable!("Message / Enum not resolved"),
        }
    }

    fn is_fixed_size(&self) -> bool {
        matches!(self.wire_type_num_non_packed(), 1 | 5)
    }

    fn singular_field_defaults(&self, desc: &FileDescriptor) -> String {
        match *self {
            FieldType::Int32 => "0i32".to_owned(),
            FieldType::Sint32 => "0i32".to_owned(),
            FieldType::Int64 => "0i64".to_owned(),
            FieldType::Sint64 => "0i64".to_owned(),
            FieldType::Uint32 => "0u32".to_owned(),
            FieldType::Uint64 => "0u64".to_owned(),
            FieldType::Bool => "false".to_owned(),
            FieldType::Fixed32 => "0u32".to_owned(),
            FieldType::Sfixed32 => "0i32".to_owned(),
            FieldType::Float => "0f32".to_owned(),
            FieldType::Fixed64 => "0u64".to_owned(),
            FieldType::Sfixed64 => "0i64".to_owned(),
            FieldType::Double => "0f64".to_owned(),
            FieldType::StringCow => "Cow::Borrowed(\"\")".to_owned(),
            FieldType::String_ => "\"\".to_string()".to_owned(),
            FieldType::BytesCow => "Cow::Borrowed(b\"\")".to_owned(),
            FieldType::Bytes_ => "Vec::<u8>::new()".to_owned(),
            FieldType::Enum(ref e) => {
                let e = e.get_enum(desc);
                e.fully_qualified_fields[0].0.to_owned()
            }
            FieldType::Message(ref m) => {
                let m = m.get_message(desc);
                format!("{}{}::default()", m.get_modules(desc), m.name)
            }
            _ => unreachable!(),
        }
    }

    pub fn message(&self) -> Option<&MessageIndex> {
        if let FieldType::Message(ref m) = self {
            Some(m)
        } else {
            None
        }
    }

    fn has_lifetime(
        &self,
        desc: &FileDescriptor,
        config: &Config,
        packed: bool,
        ignore: &mut Vec<MessageIndex>,
    ) -> bool {
        match *self {
            FieldType::StringCow | FieldType::BytesCow => true, // Cow<[u8]>
            FieldType::Message(ref m) => m.get_message(desc).has_lifetime(desc, config, ignore),
            FieldType::Fixed64
            | FieldType::Sfixed64
            | FieldType::Double
            | FieldType::Fixed32
            | FieldType::Sfixed32
            | FieldType::String_
            | FieldType::Bytes_
            | FieldType::Float => packed, // Cow<[M]>
            FieldType::Map(ref key, ref value) => {
                key.has_lifetime(desc, config, false, ignore)
                    || value.has_lifetime(desc, config, false, ignore)
            }
            _ => false,
        }
    }

    fn rust_type(&self, desc: &FileDescriptor, config: &Config) -> Result<String> {
        Ok(match *self {
            FieldType::Int32 | FieldType::Sint32 | FieldType::Sfixed32 => "i32".to_string(),
            FieldType::Int64 | FieldType::Sint64 | FieldType::Sfixed64 => "i64".to_string(),
            FieldType::Uint32 | FieldType::Fixed32 => "u32".to_string(),
            FieldType::Uint64 | FieldType::Fixed64 => "u64".to_string(),
            FieldType::Double => "f64".to_string(),
            FieldType::Float => "f32".to_string(),
            FieldType::StringCow => "Cow<'a, str>".to_string(),
            FieldType::BytesCow => "Cow<'a, [u8]>".to_string(),
            FieldType::String_ => "String".to_string(),
            FieldType::Bytes_ => "Vec<u8>".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Enum(ref e) => {
                let e = e.get_enum(desc);
                format!("{}{}", e.get_modules(desc), e.name)
            }
            FieldType::Message(ref msg) => {
                let m = msg.get_message(desc);
                let lifetime = if m.has_lifetime(desc, config, &mut Vec::new()) {
                    "<'a>"
                } else {
                    ""
                };
                format!("{}{}{}", m.get_modules(desc), m.name, lifetime)
            }
            FieldType::Map(ref key, ref value) => format!(
                "KVMap<{}, {}>",
                key.rust_type(desc, config)?,
                value.rust_type(desc, config)?
            ),
            FieldType::MessageOrEnum(_) => unreachable!("Message / Enum not resolved"),
        })
    }

    /// Returns the relevant function to read the data, both for regular and Cow wrapped
    fn read_fn(&self, desc: &FileDescriptor) -> Result<(String, String)> {
        Ok(match *self {
            FieldType::Message(ref msg) => {
                let m = msg.get_message(desc);
                let m = format!(
                    "r.read_message::<{}{}>(bytes)?",
                    m.get_modules(desc),
                    m.name
                );
                (m.clone(), m)
            }
            FieldType::Map(_, _) => return Err(Error::ReadFnMap),
            FieldType::StringCow | FieldType::BytesCow => {
                let m = format!("r.read_{}(bytes)", self.proto_type());
                let cow = format!("{}.map(Cow::Borrowed)?", m);
                (m, cow)
            }
            FieldType::String_ => {
                let m = format!("r.read_{}(bytes)", self.proto_type());
                let vec = format!("{}?.to_owned()", m);
                (m, vec)
            }
            FieldType::Bytes_ => {
                let m = format!("r.read_{}(bytes)", self.proto_type());
                let vec = format!("{}?.to_owned()", m);
                (m, vec)
            }
            FieldType::MessageOrEnum(_) => unreachable!("Message / Enum not resolved"),
            _ => {
                let m = format!("r.read_{}(bytes)?", self.proto_type());
                (m.clone(), m)
            }
        })
    }

    fn get_size(&self, s: &str) -> String {
        match *self {
            FieldType::Int32
            | FieldType::Int64
            | FieldType::Uint32
            | FieldType::Uint64
            | FieldType::Bool
            | FieldType::Enum(_) => format!("sizeof_varint(*(&{}) as u64)", s),
            FieldType::Sint32 => format!("sizeof_sint32(*(&{}))", s),
            FieldType::Sint64 => format!("sizeof_sint64(*(&{}))", s),

            FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double => "8".to_string(),
            FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => "4".to_string(),

            FieldType::StringCow | FieldType::BytesCow => format!("sizeof_len((&{}).len())", s),

            FieldType::String_ | FieldType::Bytes_ => format!("sizeof_len({}.len())", s),

            FieldType::Message(_) => format!("sizeof_len(({}).get_size())", s),

            FieldType::Map(ref k, ref v) => {
                format!("2 + {} + {}", k.get_size("k"), v.get_size("v"))
            }
            FieldType::MessageOrEnum(_) => unreachable!("Message / Enum not resolved"),
        }
    }

    fn get_write(&self, s: &str, boxed: bool) -> String {
        match *self {
            FieldType::Enum(_) => format!("write_enum(*&{} as i32)", s),

            FieldType::Int32
            | FieldType::Sint32
            | FieldType::Int64
            | FieldType::Sint64
            | FieldType::Uint32
            | FieldType::Uint64
            | FieldType::Bool
            | FieldType::Fixed64
            | FieldType::Sfixed64
            | FieldType::Double
            | FieldType::Fixed32
            | FieldType::Sfixed32
            | FieldType::Float => format!("write_{}(*&{})", self.proto_type(), s),

            FieldType::StringCow => format!("write_string({})", s),
            FieldType::BytesCow => format!("write_bytes({})", s),
            FieldType::String_ => format!("write_string({})", s),
            FieldType::Bytes_ => format!("write_bytes({})", s),

            FieldType::Message(_) if boxed => format!("write_message(&*({}))", s),
            FieldType::Message(_) => format!("write_message({})", s),

            FieldType::Map(ref k, ref v) => format!(
                "write_map({}, {}, |w| w.{}, {}, |w| w.{})",
                self.get_size(""),
                tag(1, k, false),
                k.get_write("k", false),
                tag(2, v, false),
                v.get_write("v", false)
            ),
            FieldType::MessageOrEnum(_) => unreachable!("Message / Enum not resolved"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub frequency: Frequency,
    pub typ: FieldType,
    pub number: i32,
    pub default: Option<String>,
    pub packed: Option<bool>,
    pub boxed: bool,
    pub deprecated: bool,
}

impl Field {
    fn has_valid_visible_custom_default(&self, desc: &FileDescriptor, config: &Config) -> bool {
        let is_visible = config.add_deprecated_fields || !self.deprecated;

        let is_proto2 = desc.syntax == Syntax::Proto2;

        let optional_or_required = matches!(
            self.frequency,
            Frequency::Proto2Frequency(Proto2Frequency::Optional)
                | Frequency::Proto2Frequency(Proto2Frequency::Required)
        );

        // Default values for message fields are language-specific, and most
        // languages use the null equivalent. I would use `None`; and since our
        // field type is already `Option<MyMessage>`, reading from the field
        // directly would suffice -- a getter would be redundant.
        let is_not_message = !matches!(self.typ, FieldType::Message(_));

        // As of now, we only generate getters for fields with explicit defaults
        let has_custom_default_tag = self.default.is_some();

        is_visible && is_proto2 && optional_or_required && is_not_message && has_custom_default_tag
    }

    fn must_generate_impl_default(&self, desc: &FileDescriptor, config: &Config) -> bool {
        // `impl Default` must be generated only for Proto2 `required` fields,
        // because:
        //
        // 1. They can have custom defaults (since they're Proto2)
        // 2. Custom default influences the field's initialization values
        //    instead of creating a default `const`.
        self.has_valid_visible_custom_default(desc, config)
            && self.frequency == Frequency::Proto2Frequency(Proto2Frequency::Required)
    }

    fn must_generate_custom_default_const(&self, desc: &FileDescriptor, config: &Config) -> bool {
        // Custom default `const`s are generated only for Proto2 `optional`
        // fields, because
        //
        // 1. They can have custom defaults (since they're Proto2)
        // 2. `optional` fields are always initialized to `None`, so we must
        //    generate a default `const` instead to store the default custom
        //    value for reference (either manually or through an autogenerated
        //    getter if the command line argument `--generate-getters` has been
        //    set).
        self.has_valid_visible_custom_default(desc, config) && self.frequency.is_optional()
    }

    fn must_generate_getter(&self, desc: &FileDescriptor, config: &Config) -> bool {
        // The same conditions apply to generating getters as
        // `must_generate_custom_default_const()`, except that we must also
        // check that the command line argument `--generate-getters` has been
        // set.
        //
        // NOTE: This means we only generate getters for Proto2 `optional`
        // fields *WITH* a custom default tag! Our assumption is that those
        // without custom default tags are less likely to be used in a manner
        // that involves referencing the default value (and the user can always
        // do it the manual way)
        self.must_generate_custom_default_const(desc, config) && config.generate_getters
    }

    fn write_getter<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if !(self.must_generate_getter(desc, config)) {
            return Ok(());
        }

        let name = self.name.to_owned();

        let mut return_type = self.rust_type_incl_non_cow(desc, config)?;
        if self.typ.is_cow() {
            return_type = format!("&{return_type}");
        }

        let default_const_name = self.get_default_const_name()?;

        writeln!(w, "    pub fn get_{name}(&self) -> {return_type} {{")?;

        if self.typ.is_non_cow_string_or_byte() {
            match self.typ {
                FieldType::String_ => {
                    writeln!(
                        w,
                        "        &self.{name}.as_ref().map(|s| s.as_str()).unwrap_or(Self::{default_const_name})",
                    )?;
                }
                FieldType::Bytes_ => {
                    writeln!(
                        w,
                        "        &self.{name}.as_ref().map(|s| s.as_slice()).unwrap_or(Self::{default_const_name})",
                    )?;
                }
                _ => unreachable!(),
            }
        } else {
            let dont_deref_if_cow = if !self.typ.is_cow() { "*" } else { "" };

            writeln!(
                w,
                "        {dont_deref_if_cow}self.{name}.as_ref().unwrap_or(&Self::{default_const_name})",
            )?;
        }
        writeln!(w, "    }}")?;
        Ok(())
    }

    fn can_write_default_const(&self, desc: &FileDescriptor, config: &Config) -> bool {
        // Proto3 doesn't allow custom defaults, and its standard defaults line
        // up with Rust's, so we don't need to add custom defaults.
        let is_proto3 = desc.syntax == Syntax::Proto3;

        let is_message_or_map = !self.typ.need_to_dereference()
            && !self.typ.is_cow()
            && !self.typ.is_non_cow_string_or_byte();

        let no_custom_default = self.default.is_none();

        let absent_due_to_deprecation = self.deprecated && !config.add_deprecated_fields;

        !(is_proto3
            || is_message_or_map
            || no_custom_default
            || absent_due_to_deprecation
            || !self.frequency.is_optional())
    }

    fn get_default_const_name(&self) -> Result<String> {
        Ok(format!("DEFAULT_{}", self.name))
    }

    fn rust_type_incl_non_cow(&self, desc: &FileDescriptor, config: &Config) -> Result<String> {
        if self.typ.is_non_cow_string_or_byte() {
            Ok(match self.typ {
                FieldType::String_ => "&str".to_owned(),
                FieldType::Bytes_ => "&[u8]".to_owned(),
                _ => unreachable!(),
            })
        } else {
            self.typ.rust_type(desc, config)
        }
    }

    fn write_custom_default<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if !self.can_write_default_const(desc, config) {
            return Ok(());
        }

        let default_const_name = self.get_default_const_name()?;

        let got_type = self.rust_type_incl_non_cow(desc, config)?;

        let custom_default = self.default.as_ref().unwrap();

        writeln!(
            w,
            "    pub const {default_const_name}: {got_type} = {custom_default};"
        )?;
        Ok(())
    }

    fn write_default<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if self.deprecated && !config.add_deprecated_fields {
            return Ok(());
        }

        writeln!(
            w,
            "            {}: {},",
            self.name,
            self.get_field_default(desc, config)
        )?;
        Ok(())
    }

    fn get_field_default(&self, desc: &FileDescriptor, config: &Config) -> String {
        match self.frequency.into() {
            GeneratedType::SingularType => {
                if self.boxed {
                    return "None".to_owned();
                }
                if self.frequency.is_optional() {
                    // No matter whether there is a custom or standard default,
                    // the field will still be initialized to `None`, since we
                    // rely on `is_some()` as a hazzer. User must retrieve
                    // default themselves.
                    "None".to_owned()
                } else {
                    match &self.default {
                        Some(custom_default) => {
                            // If the command line option `--dont_use_cow` is
                            // set, `String` and `Vec<u8>` will be used instead
                            // of `Cow<'a, str>` and `Cow<'a, [u8]>`
                            // respectively. However, custom defaults are
                            // represented using `const` variables, and neither
                            // `String` nor `Vec<u8>` are able to be `const`.
                            // Thus, we need to be able to generate:
                            //
                            //      1) for string fields, both `&str` as well as
                            //          `String` versions
                            //      2) for byte fields, both `[u8; N]` as well
                            //          as `Vec<u8>` versions
                            //
                            // For convenience, we start with the `const`
                            // versions, and add on methods to convert to
                            // non-`const` versions below.
                            if self.typ.is_non_cow_string_or_byte() {
                                return match self.typ {
                                    FieldType::String_ => format!("{custom_default}.to_string()"),
                                    FieldType::Bytes_ => format!("{custom_default}.to_vec()"),
                                    _ => unreachable!(),
                                };
                            }
                            custom_default.clone()
                        }
                        None => self.typ.singular_field_defaults(desc),
                    }
                }
            }
            GeneratedType::ArrayLikeType => {
                if self.packed() && self.typ.is_fixed_size() && !config.dont_use_cow {
                    "PackedFixed::from(Vec::new())".to_owned()
                } else {
                    "Vec::new()".to_owned()
                }
            }
            GeneratedType::Map => "HashMap::new()".to_owned(),
        }
    }

    fn packed(&self) -> bool {
        self.packed.unwrap_or(false)
    }

    fn sanitize_default(&mut self, desc: &FileDescriptor, config: &Config) -> Result<()> {
        if let Some(ref mut d) = self.default {
            *d = match &*self.typ.rust_type(desc, config)? {
                "u32" => format!("{}u32", *d),
                "u64" => format!("{}u64", *d),
                "i32" => format!("{}i32", *d),
                "i64" => format!("{}i64", *d),
                "f32" => match &*d.to_lowercase() {
                    "inf" => "::core::f32::INFINITY".to_string(),
                    "-inf" => "::core::f32::NEG_INFINITY".to_string(),
                    "nan" => "::core::f32::NAN".to_string(),
                    _ => format!("{}f32", *d),
                },
                "f64" => match &*d.to_lowercase() {
                    "inf" => "::core::f64::INFINITY".to_string(),
                    "-inf" => "::core::f64::NEG_INFINITY".to_string(),
                    "nan" => "::core::f64::NAN".to_string(),
                    _ => format!("{}f64", *d),
                },
                "Cow<'a, str>" => format!("Cow::Borrowed(\"{}\")", d),
                "Cow<'a, [u8]>" => format!("Cow::Borrowed(b\"{}\")", d),
                "String" => format!("\"{}\"", d),
                "Bytes" => format!("b\"{}\"", d),
                "Vec<u8>" => format!("b\"{}\"", d),
                "bool" => format!("{}", d.parse::<bool>().unwrap()),
                e => format!("{}::{}", e, d), // enum, as message and map do not have defaults
            }
        }
        Ok(())
    }

    fn tag(&self) -> u32 {
        tag(self.number as u32, &self.typ, self.packed())
    }

    pub fn get_type(&self, desc: &FileDescriptor, config: &Config) -> String {
        let rust_type = self.typ.rust_type(desc, config).unwrap();
        if self.boxed {
            return format!("Option<Box<{}>>", rust_type);
        }

        match self.frequency.into() {
            GeneratedType::SingularType => {
                if self.frequency.is_optional() {
                    format!("Option<{}>", rust_type)
                } else {
                    rust_type
                }
            }
            GeneratedType::ArrayLikeType => {
                if self.packed() && self.typ.is_fixed_size() && !config.dont_use_cow {
                    format!("PackedFixed<'a, {}>", rust_type)
                } else {
                    format!("Vec<{}>", rust_type)
                }
            }
            GeneratedType::Map => {
                rust_type // rust_type is already KVMap<{}, {}>
            }
        }
    }

    fn write_definition<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if self.deprecated {
            if config.add_deprecated_fields {
                writeln!(w, "    #[deprecated]")?;
            } else {
                return Ok(());
            }
        }
        writeln!(w, "    pub {}: {},", self.name, self.get_type(desc, config))?;

        Ok(())
    }

    fn write_match_tag<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if self.deprecated && !config.add_deprecated_fields {
            return Ok(());
        }

        let (val, val_cow) = if self.frequency.is_map() {
            ("".to_owned(), "".to_owned()) // ignore if is map
        } else {
            self.typ.read_fn(desc)?
        };

        let name = &self.name;
        write!(w, "                Ok({}) => ", self.tag())?;

        if self.boxed {
            writeln!(w, "msg.{} = Some(Box::new({})),", name, val)?;
            return Ok(());
        }

        match self.frequency.into() {
            GeneratedType::SingularType => match &self.frequency {
                Frequency::Proto2Frequency(freq) => match freq {
                    Proto2Frequency::Optional => writeln!(w, "msg.{} = Some({}),", name, val_cow)?,
                    Proto2Frequency::Required => writeln!(w, "msg.{} = {},", name, val_cow)?,
                    _ => unreachable!(),
                },
                Frequency::Proto3Frequency(freq) => match freq {
                    Proto3Frequency::Optional => writeln!(w, "msg.{} = Some({}),", name, val_cow)?,
                    Proto3Frequency::Default => writeln!(w, "msg.{} = {},", name, val_cow)?,
                    _ => unreachable!(),
                },
            },
            GeneratedType::ArrayLikeType => {
                if self.packed() {
                    if self.typ.is_fixed_size() {
                        writeln!(w, "msg.{} = r.read_packed_fixed(bytes)?,", name)?;
                    } else {
                        writeln!(
                            w,
                            "msg.{} = r.read_packed(bytes, |r, bytes| Ok({}))?,",
                            name, val_cow
                        )?;
                    }
                } else {
                    writeln!(w, "msg.{}.push({}),", name, val_cow)?;
                }
            }
            GeneratedType::Map => {
                // TODO: Is there any way of doing this without `if let`? `let`
                // by itself requires "irrefutable types".
                if let FieldType::Map(ref key, ref value) = self.typ {
                    writeln!(w, "{{")?;
                    writeln!(
                        w,
                        "                    let (key, value) = \
                        r.read_map(bytes, |r, bytes| Ok({}), |r, bytes| Ok({}))?;",
                        key.read_fn(desc)?.1,
                        value.read_fn(desc)?.1
                    )?;
                    writeln!(
                        w,
                        "                    msg.{}.insert(key, value);",
                        self.name
                    )?;
                    writeln!(w, "                }}")?;
                    return Ok(());
                } else {
                    unreachable!();
                }
            }
        }

        Ok(())
    }

    fn write_get_size<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if self.deprecated && !config.add_deprecated_fields {
            return Ok(());
        }

        write!(w, "        + ")?;
        let tag_size = sizeof_varint(self.tag());

        match self.frequency.into() {
            GeneratedType::SingularType => {
                /*
                NOTE: This section might look similar to the one in
                `write_write()`, but it's not exactly the same! Because code
                generation is rather case-specific, I've decided not to try and
                refactor this with the one in `write_write()` in case
                alterations need to be made later on for some specific
                situation

                There's some convoluted conditionals and string stitching
                here in order to tackle all the different possible
                combinations of `Option`, `Box`, has presence, no presence,
                etc. It's mostly to satisfy the fiddly code syntax, and does
                not have much impact on the big idea.

                Logic-wise, it just generates code that checks for any
                necessary conditions for serialization, before adding its
                size to the tally.
                */
                fn get_size_addition(field: &Field, tag_size: usize, s: &str) -> String {
                    format!(
                        "{tag_size} + {actual_size}",
                        actual_size =
                            field
                                .typ
                                .get_size(if field.typ.is_fixed_size() { "" } else { s })
                    )
                }

                let conditions_checked = {
                    let name = self.name.clone();
                    let def = self.get_field_default(desc, config);
                    let m_size_addition = get_size_addition(self, tag_size, "m");
                    let self_name_size_addition =
                        get_size_addition(self, tag_size, format!("self.{name}").as_str());
                    let maybe_deref_m = if self.boxed || !self.typ.is_primitive() {
                        "m"
                    } else {
                        "&m"
                    };

                    match (!self.has_presence(), self.frequency.is_optional()) {
                        (true, true) => format!("self.{name}.as_ref().map_or(0, |{maybe_deref_m}| if m != {def} {{ {m_size_addition} }} else {{ 0 }}"),
                        (true, false) => format!("if self.{name} == {def} {{ 0 }} else {{ {self_name_size_addition} }}"),
                        (false, true) => format!("self.{name}.as_ref().map_or(0, |{maybe_deref_m}| {m_size_addition})"),
                        (false, false) => get_size_addition(self, tag_size, format!("self.{}", self.name).as_str()),
                    }
                };

                writeln!(w, "{}", conditions_checked.as_str(),)?;
            }
            GeneratedType::ArrayLikeType => {
                if self.packed() {
                    write!(
                        w,
                        "if self.{}.is_empty() {{ 0 }} else {{ {} + ",
                        self.name, tag_size
                    )?;
                    match self.typ.wire_type_num_non_packed() {
                        1 => writeln!(w, "sizeof_len(self.{}.len() * 8) }}", self.name)?,
                        5 => writeln!(w, "sizeof_len(self.{}.len() * 4) }}", self.name)?,
                        _ => writeln!(
                            w,
                            "sizeof_len(self.{}.iter().map(|&s| {}).sum::<usize>()) }}",
                            self.name,
                            self.typ.get_size("s")
                        )?,
                    }
                } else {
                    match self.typ.wire_type_num_non_packed() {
                        1 => writeln!(w, "({} + 8) * self.{}.len()", tag_size, self.name)?,
                        5 => writeln!(w, "({} + 4) * self.{}.len()", tag_size, self.name)?,
                        _ => writeln!(
                            w,
                            "self.{name}.iter().map(|{maybe_ampersand}s| {tag_size} + {got_size}).sum::<usize>()",
                            maybe_ampersand = if self.typ.need_to_dereference() { "&" } else { "" },
                            name = self.name,
                            got_size = self.typ.get_size("s")
                        )?,
                    }
                }
            }
            GeneratedType::Map => {
                if let FieldType::Map(k, v) = &self.typ {
                    writeln!(
                        w,
                        "self.{name}.iter().map(|({maybe_ampersand_k}k, {maybe_ampersand_v}v)| {tag_size} + sizeof_len({got_size})).sum::<usize>()",
                        maybe_ampersand_k = if k.need_to_dereference() { "&" } else { "" },
                        maybe_ampersand_v = if v.need_to_dereference() { "&" } else { "" },
                        name = self.name,
                        got_size = self.typ.get_size(""),
                    )?;
                } else {
                    unreachable!();
                }
            }
        }
        Ok(())
    }

    fn has_presence(&self) -> bool {
        match &self.frequency {
            Frequency::Proto2Frequency(ref f) => {
                if let Proto2Frequency::Repeated = f {
                    return false;
                }
                if let FieldType::Map(..) = self.typ {
                    return false;
                }
                true
            }
            Frequency::Proto3Frequency(ref f) => {
                if let Proto3Frequency::Repeated = f {
                    return false;
                }
                if let FieldType::Map(..) = self.typ {
                    return false;
                }
                if let FieldType::Message(_) = self.typ {
                    return true;
                }
                if let Proto3Frequency::Optional = f {
                    return true;
                }
                false
            }
        }
    }

    fn write_write<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if self.deprecated && !config.add_deprecated_fields {
            return Ok(());
        }

        write!(w, "        ")?;

        match self.frequency.into() {
            GeneratedType::SingularType => {
                /*
                NOTE: This section might look similar to the one in
                `get_write()`, but it's not exactly the same! Because code
                generation is rather case-specific, I've decided not to try
                and refactor this with the one in `get_write()`, in case
                alterations need to be made later on for some specific
                situation.

                There's some convoluted conditionals and string stitching
                here in order to tackle all the different possible
                combinations of `Option`, `Box`, has presence, no presence,
                etc. It's mostly to satisfy the fiddly code syntax, and does
                not have much impact on the big idea.

                Logic-wise, it just generates code that checks for any
                necessary conditions for serialization, before calling
                `write_with_tag()`.
                */

                // Fiddly piece of code to get the actual type ready to pass to
                // their respective writer functions; i.e. if it's a `Box`,
                // dereference it etc. Some types need extra references, or
                // dereferences.
                fn apply_unwrapping_code(f: &Field, equating_cows: bool, s: &str) -> String {
                    let mut core = s.to_owned();

                    if f.boxed {
                        core = format!("*({core})");
                    }

                    // Required message, map and byte_ fields
                    if !f.typ.need_to_dereference()
                        && !f.typ.is_cow()
                        && !f.frequency.is_optional()
                        && !f.typ.is_non_cow_string_or_byte()
                    {
                        core = format!("&{core}");
                    }

                    /*
                    There's a rather annoying edge case, an example of which is
                    given below:

                    if &self.f_bytes != Cow::Borrowed(b"") {...}
                       ^

                    For most field types, the above ampersand is ok, but for `Cow`s,
                    it will trigger an error (cannot compare `&Cow` to `Cow`), so we
                    introduce a check for this specifically.
                    */
                    if !equating_cows {
                        core = if f.typ.is_cow() || f.typ.is_non_cow_string_or_byte() {
                            format!("&{core}")
                        } else {
                            core.clone()
                        };
                    }

                    core
                }

                fn get_write_method(f: &Field, s: &str) -> String {
                    format!(
                        "w.write_with_tag({}, |w| w.{})",
                        f.tag(),
                        f.typ.get_write(s, f.boxed)
                    )
                }

                // Check for conditions if necessary
                let conditions_checked = {
                    let name = self.name.clone();
                    let def = self.get_field_default(desc, config);
                    let m_core = apply_unwrapping_code(self, false, "m");
                    let self_name_core =
                        apply_unwrapping_code(self, false, format!("self.{name}").as_str());
                    let m_write_method = get_write_method(self, m_core.as_str());
                    let self_name_write_method = get_write_method(self, &self_name_core);
                    let maybe_deref_m = if self.boxed || !self.typ.is_primitive() {
                        "m"
                    } else {
                        "&m"
                    };
                    let self_name_core_equating_cows =
                        apply_unwrapping_code(self, true, format!("self.{name}").as_str());

                    match (!self.has_presence(), self.frequency.is_optional()) {
                        (true, true) => format!("self.{name}.as_ref().map_or(Ok(()), |{maybe_deref_m}| if {m_core} != {def} {{ {m_write_method} }} else {{ Ok(()) }})?;"),
                        (true, false) => format!("if {self_name_core_equating_cows} != {def} {{ {self_name_write_method}?; }}"),
                        (false, true) => format!("self.{name}.as_ref().map_or(Ok(()), |{maybe_deref_m}| {m_write_method})?;"),
                        (false, false) => format!("{}?;", self_name_write_method),
                    }
                };

                writeln!(w, "{}", conditions_checked.as_str(),)?;
            }
            GeneratedType::ArrayLikeType => {
                if self.packed() {
                    if self.typ.is_fixed_size() {
                        writeln!(
                            w,
                            "w.write_packed_fixed_with_tag({}, &self.{})?;",
                            self.tag(),
                            self.name
                        )?;
                    } else {
                        writeln!(
                            w,
                            "w.write_packed_with_tag({}, &self.{}, |w, &m| w.{}, &|&m| {})?;",
                            self.tag(),
                            self.name,
                            self.typ.get_write("m", self.boxed),
                            self.typ.get_size("m")
                        )?
                    }
                } else {
                    let maybe_deref_s = if self.typ.need_to_dereference() {
                        "*s"
                    } else {
                        "s"
                    };
                    writeln!(
                        w,
                        "for s in &self.{} {{ w.write_with_tag({}, |w| w.{})?; }}",
                        self.name,
                        self.tag(),
                        self.typ.get_write(maybe_deref_s, self.boxed)
                    )?;
                }
            }
            GeneratedType::Map => {
                if let FieldType::Map(k, v) = &self.typ {
                    writeln!(
                        w,
                        "for ({maybe_ampersand_k}k, {maybe_ampersand_v}v) in self.{name}.iter() {{ w.write_with_tag({tag}, |w| w.{got_write})?; }}",
                        maybe_ampersand_k = if k.need_to_dereference() { "&" } else { "" },
                        maybe_ampersand_v = if v.need_to_dereference() { "&" } else { "" },
                        name = self.name,
                        tag = self.tag(),
                        got_write = self.typ.get_write("", false),
                    )?;
                } else {
                    unreachable!();
                }
            }
        }
        Ok(())
    }
}

fn get_modules(module: &str, imported: bool, desc: &FileDescriptor) -> String {
    let skip = usize::from(desc.package.is_empty() && !imported);
    module
        .split('.')
        .filter(|p| !p.is_empty())
        .skip(skip)
        .map(|p| format!("{}::", p))
        .collect()
}

#[derive(Debug, Clone, Default)]
pub struct Extend {
    /// The message being extended.
    pub name: String,
    /// All fields that are being added to the extended message.
    pub fields: Vec<Field>,
}

impl Extend {}

#[derive(Debug, Clone, Default)]
pub struct Message {
    pub name: String,
    pub fields: Vec<Field>,
    pub oneofs: Vec<OneOf>,
    pub reserved_nums: Option<Vec<i32>>,
    pub reserved_names: Option<Vec<String>>,
    pub imported: bool,
    pub package: String,        // package from imports + nested items
    pub messages: Vec<Message>, // nested messages
    pub enums: Vec<Enumerator>, // nested enums
    pub module: String,         // 'package' corresponding to actual generated Rust module
    pub path: PathBuf,
    pub import: PathBuf,
    pub index: MessageIndex,
    /// Allowed extensions for this message, None if no extensions.
    pub extensions: Option<Extensions>,
}

impl Message {
    fn write_default_consts<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        // Proto3 doesn't allow custom defaults, so we only need the list of
        // custom default consts for Proto2
        if desc.syntax != Syntax::Proto2 {
            return Ok(());
        }

        // If there are no showable fields with custom defaults, nothing to
        // write; this is a more thorough version of `is_unit()`, which this
        // replaces. Oneofs don't have custom defaults, so we don't need to
        // check for that.
        if !self
            .fields
            .iter()
            .any(|f| f.can_write_default_const(desc, config))
        {
            return Ok(());
        }

        let mut ignore = Vec::new();
        if config.dont_use_cow {
            ignore.push(self.index.clone());
        }
        if self.has_lifetime(desc, config, &mut ignore) {
            writeln!(w, "impl<'a> {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "impl {} {{", self.name)?;
        }

        for f in &self.fields {
            f.write_custom_default(w, desc, config)?;
        }

        // Oneofs don't have custom defaults, no need to cycle through them like
        // we did for fields

        writeln!(w, "}}")?;
        writeln!(w)?;

        Ok(())
    }

    fn convert_field_types(&mut self, from: &FieldType, to: &FieldType) {
        for f in self.all_fields_mut().filter(|f| f.typ == *from) {
            f.typ = to.clone();
        }

        // If that type is a map with the fieldtype, it must also be converted.
        for f in self.all_fields_mut() {
            let new_type: FieldType = match f.typ {
                FieldType::Map(ref mut key, ref mut value)
                    if **key == *from && **value == *from =>
                {
                    FieldType::Map(Box::new(to.clone()), Box::new(to.clone()))
                }
                FieldType::Map(ref mut key, ref mut value) if **key == *from => {
                    FieldType::Map(Box::new(to.clone()), value.clone())
                }
                FieldType::Map(ref mut key, ref mut value) if **value == *from => {
                    FieldType::Map(key.clone(), Box::new(to.clone()))
                }
                ref other => other.clone(),
            };
            f.typ = new_type;
        }

        for message in &mut self.messages {
            message.convert_field_types(from, to);
        }
    }

    fn has_lifetime(
        &self,
        desc: &FileDescriptor,
        config: &Config,
        ignore: &mut Vec<MessageIndex>,
    ) -> bool {
        if ignore.contains(&self.index) {
            return false;
        }
        ignore.push(self.index.clone());
        let res = self.all_fields().any(|f| {
            f.typ.has_lifetime(desc, config, f.packed(), ignore)
                && (!f.deprecated || config.add_deprecated_fields)
        });
        ignore.pop();
        res
    }

    fn set_imported(&mut self) {
        self.imported = true;
        for o in self.oneofs.iter_mut() {
            o.imported = true;
        }
        for m in self.messages.iter_mut() {
            m.set_imported();
        }
        for e in self.enums.iter_mut() {
            e.imported = true;
        }
    }

    fn get_modules(&self, desc: &FileDescriptor) -> String {
        get_modules(&self.module, self.imported, desc)
    }

    fn is_unit(&self) -> bool {
        self.fields.is_empty()
            && self.oneofs.is_empty()
            && self.messages.iter().all(|m| m.is_unit())
    }

    fn write_common_uses<W: Write>(
        w: &mut W,
        messages: &[Message],
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if config.nostd {
            writeln!(w, "use alloc::vec::Vec;")?;
        }

        if !config.dont_use_cow {
            if messages.iter().any(|m| {
                m.all_fields()
                    .any(|f| (f.typ.has_cow() || (f.packed() && f.typ.is_fixed_size())))
            }) {
                if config.nostd {
                    writeln!(w, "use alloc::borrow::Cow;")?;
                } else {
                    writeln!(w, "use std::borrow::Cow;")?;
                }
            }
        } else if config.nostd
            && messages
                .iter()
                .any(|m| m.all_fields().any(|f| (f.typ.has_bytes_and_string())))
        {
            writeln!(w, "use alloc::borrow::ToOwned;")?;
        }

        if config.nostd
            && messages.iter().any(|m| {
                desc.owned && m.has_lifetime(desc, config, &mut Vec::new())
                    || m.all_fields().any(|f| f.boxed)
            })
        {
            writeln!(w)?;
            writeln!(w, "use alloc::boxed::Box;")?;
        }

        if messages
            .iter()
            .filter(|m| !m.imported)
            .any(|m| m.all_fields().any(|f| f.typ.is_map()))
        {
            if config.hashbrown {
                writeln!(w, "use hashbrown::HashMap;")?;
                writeln!(w, "type KVMap<K, V> = HashMap<K, V>;")?;
            } else if config.nostd {
                writeln!(w, "use alloc::collections::BTreeMap;")?;
                writeln!(w, "type KVMap<K, V> = BTreeMap<K, V>;")?;
            } else {
                writeln!(w, "use std::collections::HashMap;")?;
                writeln!(w, "type KVMap<K, V> = HashMap<K, V>;")?;
            }
        }

        Ok(())
    }

    fn write_getters<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        let mut ignore = Vec::new();
        if config.dont_use_cow {
            ignore.push(self.index.clone());
        }
        if self.has_lifetime(desc, config, &mut ignore) {
            writeln!(w, "impl<'a> {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "impl {} {{", self.name)?;
        }
        for f in &self.fields {
            f.write_getter(w, desc, config)?;
        }
        writeln!(w, "}}")?;
        writeln!(w)?;
        Ok(())
    }

    fn must_generate_custom_default_consts(&self, desc: &FileDescriptor, config: &Config) -> bool {
        self.fields
            .iter()
            .any(|f| f.must_generate_custom_default_const(desc, config))
    }

    fn must_generate_getters(&self, desc: &FileDescriptor, config: &Config) -> bool {
        self.fields
            .iter()
            .any(|f| f.must_generate_getter(desc, config))
            && config.generate_getters
    }

    fn must_generate_impl_default(&self, desc: &FileDescriptor, config: &Config) -> bool {
        self.fields
            .iter()
            .any(|f| f.must_generate_impl_default(desc, config))
    }

    fn write<W: Write>(&self, w: &mut W, desc: &FileDescriptor, config: &Config) -> Result<()> {
        println!("Writing message {}{}", self.get_modules(desc), self.name);
        writeln!(w)?;

        self.write_definition(w, desc, config)?;
        writeln!(w)?;

        // We can't only check on a field-by-field basis if each field needs to
        // have a default `const` written. This is because if NONE of the fields
        // needs it, we want to know in advance and avoid writing the
        // surrounding `impl` block in the first place to keep the generated
        // code clean.
        if self.must_generate_custom_default_consts(desc, config) {
            self.write_default_consts(w, desc, config)?;
            writeln!(w)?;
        }

        // Ditto comment here as for `must_generate_custom_default_consts()`
        // above
        if self.must_generate_getters(desc, config) {
            self.write_getters(w, desc, config)?;
            writeln!(w)?;
        }

        // Ditto comment here as for `must_generate_custom_default_consts()`
        // above
        if self.must_generate_impl_default(desc, config) {
            self.write_impl_default(w, desc, config)?;
            writeln!(w)?;
        }

        self.write_impl_message_read(w, desc, config)?;
        writeln!(w)?;
        if config.gen_write {
            self.write_impl_message_write(w, desc, config)?;
        }

        if config.gen_info {
            self.write_impl_message_info(w, desc, config)?;
            writeln!(w)?;
        }

        if desc.owned {
            writeln!(w)?;

            if self.has_lifetime(desc, config, &mut Vec::new()) {
                self.write_impl_owned(w, config)?;
            } else {
                self.write_impl_try_from(w)?;
            }
        }

        if !(self.messages.is_empty() && self.enums.is_empty() && self.oneofs.is_empty()) {
            writeln!(w)?;
            writeln!(w, "pub mod mod_{} {{", self.name)?;
            writeln!(w)?;

            Self::write_common_uses(w, &self.messages, desc, config)?;

            if !self.messages.is_empty() || !self.oneofs.is_empty() {
                writeln!(w, "use super::*;")?;
            }
            for m in &self.messages {
                m.write(w, desc, config)?;
            }
            for e in &self.enums {
                e.write(w)?;
            }
            for o in &self.oneofs {
                o.write(w, desc, config)?;
            }

            writeln!(w)?;
            writeln!(w, "}}")?;
        }

        Ok(())
    }

    fn write_definition<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        let mut custom_struct_derive = config.custom_struct_derive.join(", ");

        if !self.must_generate_impl_default(desc, config) {
            custom_struct_derive += "Default";
        }

        if !custom_struct_derive.is_empty() {
            custom_struct_derive += ", ";
        }

        writeln!(w, "#[allow(clippy::derive_partial_eq_without_eq)]")?;

        writeln!(
            w,
            "#[derive({}Debug, PartialEq, Clone)]",
            custom_struct_derive
        )?;

        if let Some(repr) = &config.custom_repr {
            writeln!(w, "#[repr({})]", repr)?;
        }

        if self.is_unit() {
            writeln!(w, "pub struct {} {{ }}", self.name)?;
            return Ok(());
        }

        let mut ignore = Vec::new();
        if config.dont_use_cow {
            ignore.push(self.index.clone());
        }
        if self.has_lifetime(desc, config, &mut ignore) {
            writeln!(w, "pub struct {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "pub struct {} {{", self.name)?;
        }
        for f in &self.fields {
            f.write_definition(w, desc, config)?;
        }
        for o in &self.oneofs {
            o.write_message_definition(w, desc, config)?;
        }
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_impl_default<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if self.is_unit() {
            writeln!(w, "impl Default for {} {{", self.name)?;
            writeln!(w, "    fn default() -> Self {{")?;
            writeln!(w, "        Self {{}}")?;
            writeln!(w, "    }}")?;
            writeln!(w, "}}")?;
            return Ok(());
        }

        let mut ignore = Vec::new();
        if config.dont_use_cow {
            ignore.push(self.index.clone());
        }
        if self.has_lifetime(desc, config, &mut ignore) {
            writeln!(w, "impl<'a> Default for {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "impl Default for {} {{", self.name)?;
        }
        writeln!(w, "    fn default() -> Self {{")?;
        writeln!(w, "        Self {{")?;
        for f in &self.fields {
            f.write_default(w, desc, config)?;
        }
        for o in &self.oneofs {
            o.write_default(w, desc)?;
        }
        writeln!(w, "        }}")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_impl_message_info<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        let mut ignore = Vec::new();
        if config.dont_use_cow {
            ignore.push(self.index.clone());
        }
        if self.has_lifetime(desc, config, &mut ignore) {
            writeln!(w, "impl<'a> MessageInfo for {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "impl MessageInfo for {} {{", self.name)?;
        }
        writeln!(
            w,
            "    const PATH : &'static str = \"{}.{}\";",
            self.module, self.name
        )?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_impl_message_read<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if self.is_unit() {
            writeln!(w, "impl<'a> MessageRead<'a> for {} {{", self.name)?;
            writeln!(
                w,
                "    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {{"
            )?;
            writeln!(w, "        r.read_to_end();")?;
            writeln!(w, "        Ok(Self::default())")?;
            writeln!(w, "    }}")?;
            writeln!(w, "}}")?;
            return Ok(());
        }

        let mut ignore = Vec::new();
        if config.dont_use_cow {
            ignore.push(self.index.clone());
        }
        if self.has_lifetime(desc, config, &mut ignore) {
            writeln!(w, "impl<'a> MessageRead<'a> for {}<'a> {{", self.name)?;
            writeln!(
                w,
                "    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {{"
            )?;
        } else {
            writeln!(w, "impl<'a> MessageRead<'a> for {} {{", self.name)?;
            writeln!(
                w,
                "    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {{"
            )?;
        }

        writeln!(w, "        let mut msg = Self::default();")?;
        writeln!(w, "        while !r.is_eof() {{")?;
        writeln!(w, "            match r.next_tag(bytes) {{")?;
        for f in &self.fields {
            f.write_match_tag(w, desc, config)?;
        }
        for o in &self.oneofs {
            o.write_match_tag(w, desc, config)?;
        }
        writeln!(
            w,
            "                Ok(t) => {{ r.read_unknown(bytes, t)?; }}"
        )?;
        writeln!(w, "                Err(e) => return Err(e),")?;
        writeln!(w, "            }}")?;
        writeln!(w, "        }}")?;
        writeln!(w, "        Ok(msg)")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;

        Ok(())
    }

    fn write_impl_message_write<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if self.is_unit() {
            writeln!(w, "impl MessageWrite for {} {{ }}", self.name)?;
            return Ok(());
        }

        let mut ignore = Vec::new();
        if config.dont_use_cow {
            ignore.push(self.index.clone());
        }
        if self.has_lifetime(desc, config, &mut ignore) {
            writeln!(w, "impl<'a> MessageWrite for {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "impl MessageWrite for {} {{", self.name)?;
        }
        self.write_get_size(w, desc, config)?;
        writeln!(w)?;
        self.write_write_message(w, desc, config)?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_impl_owned<W: Write>(&self, w: &mut W, config: &Config) -> Result<()> {
        write!(
            w,
            r#"
            // IMPORTANT: For any future changes, note that the lifetime parameter
            // of the `proto` field is set to 'static!!!
            //
            // This means that the internals of `proto` should at no point create a
            // mutable reference to something using that lifetime parameter, on pain
            // of UB. This applies even though it may be transmuted to a smaller
            // lifetime later (through `proto()` or `proto_mut()`).
            //
            // At the time of writing, the only possible thing that uses the
            // lifetime parameter is `Cow<'a, T>`, which never does this, so it's
            // not UB.
            //
            #[derive(Debug)]
            struct {name}OwnedInner {{
                buf: Vec<u8>,
                proto: Option<{name}<'static>>,
                _pin: core::marker::PhantomPinned,
            }}

            impl {name}OwnedInner {{
                fn new(buf: Vec<u8>) -> Result<core::pin::Pin<Box<Self>>> {{
                    let inner = Self {{
                        buf,
                        proto: None,
                        _pin: core::marker::PhantomPinned,
                    }};
                    let mut pinned = Box::pin(inner);

                    let mut reader = BytesReader::from_bytes(&pinned.buf);
                    let proto = {name}::from_reader(&mut reader, &pinned.buf)?;

                    unsafe {{
                        let proto = core::mem::transmute::<_, {name}<'_>>(proto);
                        pinned.as_mut().get_unchecked_mut().proto = Some(proto);
                    }}
                    Ok(pinned)
                }}
            }}

            pub struct {name}Owned {{
                inner: core::pin::Pin<Box<{name}OwnedInner>>,
            }}

            #[allow(dead_code)]
            impl {name}Owned {{
                pub fn buf(&self) -> &[u8] {{
                    &self.inner.buf
                }}

                pub fn proto<'a>(&'a self) -> &'a {name}<'a> {{
                    let proto = self.inner.proto.as_ref().unwrap();
                    unsafe {{ core::mem::transmute::<&{name}<'static>, &{name}<'a>>(proto) }}
                }}

                pub fn proto_mut<'a>(&'a mut self) -> &'a mut {name}<'a> {{
                    let inner = self.inner.as_mut();
                    let inner = unsafe {{ inner.get_unchecked_mut() }};
                    let proto = inner.proto.as_mut().unwrap();
                    unsafe {{ core::mem::transmute::<_, &mut {name}<'a>>(proto) }}
                }}
            }}

            impl core::fmt::Debug for {name}Owned {{
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                    self.inner.proto.as_ref().unwrap().fmt(f)
                }}
            }}

            impl TryFrom<Vec<u8>> for {name}Owned {{
                type Error=quick_protobuf::Error;

                fn try_from(buf: Vec<u8>) -> Result<Self> {{
                    Ok(Self {{ inner: {name}OwnedInner::new(buf)? }})
                }}
            }}

            impl TryInto<Vec<u8>> for {name}Owned {{
                type Error=quick_protobuf::Error;

                fn try_into(self) -> Result<Vec<u8>> {{
                    let mut buf = Vec::new();
                    let mut writer = Writer::new(&mut buf);
                    self.inner.proto.as_ref().unwrap().write_message(&mut writer)?;
                    Ok(buf)
                }}
            }}

            impl From<{name}<'static>> for {name}Owned {{
                fn from(proto: {name}<'static>) -> Self {{
                    Self {{
                        inner: Box::pin({name}OwnedInner {{
                            buf: Vec::new(),
                            proto: Some(proto),
                            _pin: core::marker::PhantomPinned,
                        }})
                    }}
                }}
            }}
            "#,
            name = self.name
        )?;

        if config.gen_info {
            write!(
                w,
                r#"
            impl MessageInfo for {name}Owned {{
                const PATH: &'static str = "{module}.{name}";
            }}
            "#,
                name = self.name,
                module = self.module
            )?;
        }
        Ok(())
    }

    fn write_get_size<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        writeln!(w, "    fn get_size(&self) -> usize {{")?;
        writeln!(w, "        0")?;
        for f in &self.fields {
            f.write_get_size(w, desc, config)?;
        }
        for o in self.oneofs.iter() {
            o.write_get_size(w, desc, config)?;
        }
        writeln!(w, "    }}")?;
        Ok(())
    }

    fn write_impl_try_from<W: Write>(&self, w: &mut W) -> Result<()> {
        write!(
            w,
            r#"
            impl TryFrom<&[u8]> for {name} {{
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {{
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok({name}::from_reader(&mut reader, &buf)?)
                }}
            }}
            "#,
            name = self.name
        )?;
        Ok(())
    }

    fn write_write_message<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        writeln!(
            w,
            "    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {{"
        )?;
        for f in &self.fields {
            f.write_write(w, desc, config)?;
        }
        for o in &self.oneofs {
            o.write_write(w, desc, config)?;
        }
        writeln!(w, "        Ok(())")?;
        writeln!(w, "    }}")?;
        Ok(())
    }

    fn sanity_checks(&self, desc: &FileDescriptor) -> Result<()> {
        for f in self.all_fields() {
            // check reserved
            if self
                .reserved_names
                .as_ref()
                .map_or(false, |names| names.contains(&f.name))
                || self
                    .reserved_nums
                    .as_ref()
                    .map_or(false, |nums| nums.contains(&f.number))
            {
                return Err(Error::InvalidMessage(format!(
                    "Error in message {}\n\
                     Field {:?} conflict with reserved fields",
                    self.name, f
                )));
            }

            // check default enums
            if let Some(var) = f.default.as_ref() {
                if let FieldType::Enum(ref e) = f.typ {
                    let e = e.get_enum(desc);
                    e.fields.iter().find(|&(ref name, _)| name == var)
                    .ok_or_else(|| Error::InvalidDefaultEnum(format!(
                                "Error in message {}\n\
                                Enum field {:?} has a default value '{}' which is not valid for enum index {:?}",
                                self.name, f, var, e)))?;
                }
            }
        }
        Ok(())
    }

    fn set_package(&mut self, package: &str, module: &str) {
        // The complication here is that the _package_ (as declared in the proto file) does
        // not directly map to the _module_. For example, the package 'a.A' where A is a
        // message will be the module 'a.mod_A', since we can't reuse the message name A as
        // the submodule containing nested items. Also, protos with empty packages always
        // have a module corresponding to the file name.
        let (child_package, child_module) = if package.is_empty() {
            self.module = module.to_string();
            (self.name.clone(), format!("{}.mod_{}", module, self.name))
        } else {
            self.package = package.to_string();
            self.module = module.to_string();
            (
                format!("{}.{}", package, self.name),
                format!("{}.mod_{}", module, self.name),
            )
        };

        for m in &mut self.messages {
            m.set_package(&child_package, &child_module);
        }
        for m in &mut self.enums {
            m.set_package(&child_package, &child_module);
        }
        for m in &mut self.oneofs {
            m.set_package(&child_package, &child_module);
        }
    }

    fn set_repeated_as_packed(&mut self) {
        for f in self.all_fields_mut() {
            if f.packed.is_none() && f.frequency.is_repeated() {
                f.packed = Some(true);
            }
        }
    }

    fn unset_packed_non_primitives(&mut self) {
        for f in self.all_fields_mut() {
            if !f.typ.is_primitive() && f.packed.is_some() {
                f.packed = None;
            }
        }
    }

    fn sanitize_defaults(&mut self, desc: &FileDescriptor, config: &Config) -> Result<()> {
        for f in self.all_fields_mut() {
            f.sanitize_default(desc, config)?;
        }
        for m in &mut self.messages {
            m.sanitize_defaults(desc, config)?;
        }
        Ok(())
    }

    fn sanitize_names(&mut self) {
        sanitize_keyword(&mut self.name);
        sanitize_keyword(&mut self.package);
        for f in self.fields.iter_mut() {
            sanitize_keyword(&mut f.name);
        }
        for m in &mut self.messages {
            m.sanitize_names();
        }
        for e in &mut self.enums {
            e.sanitize_names();
        }
        for o in &mut self.oneofs {
            o.sanitize_names();
        }
    }

    /// Return an iterator producing references to all the `Field`s of `self`,
    /// including both direct and `oneof` fields.
    pub fn all_fields(&self) -> impl Iterator<Item = &Field> {
        self.fields
            .iter()
            .chain(self.oneofs.iter().flat_map(|o| o.fields.iter()))
    }

    /// Return an iterator producing mutable references to all the `Field`s of
    /// `self`, including both direct and `oneof` fields.
    fn all_fields_mut(&mut self) -> impl Iterator<Item = &mut Field> {
        self.fields
            .iter_mut()
            .chain(self.oneofs.iter_mut().flat_map(|o| o.fields.iter_mut()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct RpcFunctionDeclaration {
    pub name: String,
    pub arg: String,
    pub ret: String,
}

#[derive(Debug, Clone, Default)]
pub struct RpcService {
    pub service_name: String,
    pub functions: Vec<RpcFunctionDeclaration>,
}

impl RpcService {
    fn write_definition<W: Write>(&self, w: &mut W, config: &Config) -> Result<()> {
        (config.custom_rpc_generator)(self, w)
    }
}

pub type RpcGeneratorFunction = Box<dyn Fn(&RpcService, &mut dyn Write) -> Result<()>>;

#[derive(Debug, Clone, Default)]
pub struct Extensions {
    pub from: i32,
    /// Max number is 536,870,911 (2^29 - 1), as defined in the
    /// protobuf docs
    pub to: i32,
}

impl Extensions {
    /// The max field number that can be used as an extension.
    pub fn max() -> i32 {
        536870911
    }
}

#[derive(Debug, Clone, Default)]
pub struct Enumerator {
    pub name: String,
    pub fields: Vec<(String, i32)>,
    pub fully_qualified_fields: Vec<(String, i32)>,
    pub partially_qualified_fields: Vec<(String, i32)>,
    pub imported: bool,
    pub package: String,
    pub module: String,
    pub path: PathBuf,
    pub import: PathBuf,
    pub index: EnumIndex,
}

impl Enumerator {
    fn set_package(&mut self, package: &str, module: &str) {
        self.package = package.to_string();
        self.module = module.to_string();
        self.partially_qualified_fields = self
            .fields
            .iter()
            .map(|f| (format!("{}::{}", &self.name, f.0), f.1))
            .collect();
        self.fully_qualified_fields = self
            .partially_qualified_fields
            .iter()
            .map(|pqf| {
                let fqf = if self.module.is_empty() {
                    pqf.0.clone()
                } else {
                    format!("{}::{}", self.module.replace('.', "::"), pqf.0)
                };
                (fqf, pqf.1)
            })
            .collect();
    }

    fn sanitize_names(&mut self) {
        sanitize_keyword(&mut self.name);
        sanitize_keyword(&mut self.package);
        for f in self.fields.iter_mut() {
            sanitize_keyword(&mut f.0);
        }
    }

    fn get_modules(&self, desc: &FileDescriptor) -> String {
        get_modules(&self.module, self.imported, desc)
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<()> {
        println!("Writing enum {}", self.name);
        writeln!(w)?;
        self.write_definition(w)?;
        writeln!(w)?;
        if self.fields.is_empty() {
            Ok(())
        } else {
            self.write_impl_default(w)?;
            writeln!(w)?;
            self.write_from_i32(w)?;
            writeln!(w)?;
            self.write_from_str(w)
        }
    }

    fn write_definition<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "#[derive(Debug, PartialEq, Eq, Clone, Copy)]")?;
        writeln!(w, "pub enum {} {{", self.name)?;
        for &(ref f, ref number) in &self.fields {
            writeln!(w, "    {} = {},", f, number)?;
        }
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_impl_default<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "impl Default for {} {{", self.name)?;
        writeln!(w, "    fn default() -> Self {{")?;
        // TODO: check with default field and return error if there is no field
        writeln!(w, "        {}", self.partially_qualified_fields[0].0)?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_from_i32<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "impl From<i32> for {} {{", self.name)?;
        writeln!(w, "    fn from(i: i32) -> Self {{")?;
        writeln!(w, "        match i {{")?;
        for &(ref f, ref number) in &self.fields {
            writeln!(w, "            {} => {}::{},", number, self.name, f)?;
        }
        writeln!(w, "            _ => Self::default(),")?;
        writeln!(w, "        }}")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_from_str<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "impl<'a> From<&'a str> for {} {{", self.name)?;
        writeln!(w, "    fn from(s: &'a str) -> Self {{")?;
        writeln!(w, "        match s {{")?;
        for &(ref f, _) in &self.fields {
            writeln!(w, "            {:?} => {}::{},", f, self.name, f)?;
        }
        writeln!(w, "            _ => Self::default(),")?;
        writeln!(w, "        }}")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct OneOf {
    pub name: String,
    pub fields: Vec<Field>,
    pub package: String,
    pub module: String,
    pub imported: bool,
}

impl OneOf {
    pub fn write_default<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        writeln!(
            w,
            "            {}: {},",
            self.name,
            self.write_impl_default_field(desc)
        )?;
        Ok(())
    }

    fn write_impl_default_field(&self, desc: &FileDescriptor) -> String {
        format!("{}OneOf{}::None", self.get_modules(desc), self.name)
    }

    fn has_lifetime(&self, desc: &FileDescriptor, config: &Config) -> bool {
        self.fields.iter().any(|f| {
            f.typ
                .has_lifetime(desc, config, f.packed(), &mut Vec::new())
                && (!f.deprecated || config.add_deprecated_fields)
        })
    }

    fn set_package(&mut self, package: &str, module: &str) {
        self.package = package.to_string();
        self.module = module.to_string();
    }

    fn sanitize_names(&mut self) {
        sanitize_keyword(&mut self.name);
        sanitize_keyword(&mut self.package);
        for f in self.fields.iter_mut() {
            sanitize_keyword(&mut f.name);
        }
    }

    fn get_modules(&self, desc: &FileDescriptor) -> String {
        get_modules(&self.module, self.imported, desc)
    }

    fn write<W: Write>(&self, w: &mut W, desc: &FileDescriptor, config: &Config) -> Result<()> {
        writeln!(w)?;
        self.write_definition(w, desc, config)?;
        writeln!(w)?;
        self.write_impl_default(w, desc, config)?;
        Ok(())
    }

    fn write_definition<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        writeln!(w, "#[derive(Debug, PartialEq, Clone)]")?;
        if self.has_lifetime(desc, config) {
            writeln!(w, "pub enum OneOf{}<'a> {{", self.name)?;
        } else {
            writeln!(w, "pub enum OneOf{} {{", self.name)?;
        }
        for f in &self.fields {
            if f.deprecated {
                if config.add_deprecated_fields {
                    writeln!(w, "    #[deprecated]")?;
                } else {
                    continue;
                }
            }

            let rust_type = f.typ.rust_type(desc, config)?;
            if f.boxed {
                writeln!(w, "    {}(Box<{}>),", f.name, rust_type)?;
            } else {
                writeln!(w, "    {}({}),", f.name, rust_type)?;
            }
        }
        writeln!(w, "    None,")?;
        writeln!(w, "}}")?;

        if cfg!(feature = "generateImplFromForEnums") {
            self.generate_impl_from_for_enums(w, desc, config)
        } else {
            Ok(())
        }
    }

    fn generate_impl_from_for_enums<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        // For the first of each enumeration type, generate an impl From<> for it.
        let mut handled_fields = Vec::new();
        for f in self
            .fields
            .iter()
            .filter(|f| !f.deprecated || config.add_deprecated_fields)
        {
            let rust_type = f.typ.rust_type(desc, config)?;
            if handled_fields.contains(&rust_type) {
                continue;
            }
            writeln!(w, "impl From<{}> for OneOf{} {{", rust_type, self.name)?; // TODO: lifetime.
            writeln!(w, "   fn from(f: {}) -> OneOf{} {{", rust_type, self.name)?;
            writeln!(w, "      OneOf{}::{}(f)", self.name, f.name)?;
            writeln!(w, "   }}")?;
            writeln!(w, "}}")?;

            handled_fields.push(rust_type);
        }

        Ok(())
    }

    fn write_impl_default<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if self.has_lifetime(desc, config) {
            writeln!(w, "impl<'a> Default for OneOf{}<'a> {{", self.name)?;
        } else {
            writeln!(w, "impl Default for OneOf{} {{", self.name)?;
        }
        writeln!(w, "    fn default() -> Self {{")?;
        writeln!(w, "        OneOf{}::None", self.name)?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_message_definition<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        if self.has_lifetime(desc, config) {
            writeln!(
                w,
                "    pub {}: {}OneOf{}<'a>,",
                self.name,
                self.get_modules(desc),
                self.name
            )?;
        } else {
            writeln!(
                w,
                "    pub {}: {}OneOf{},",
                self.name,
                self.get_modules(desc),
                self.name
            )?;
        }
        Ok(())
    }

    fn write_match_tag<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        for f in self
            .fields
            .iter()
            .filter(|f| !f.deprecated || config.add_deprecated_fields)
        {
            let (val, val_cow) = f.typ.read_fn(desc)?;
            if f.boxed {
                writeln!(
                    w,
                    "                Ok({}) => msg.{} = {}OneOf{}::{}(Box::new({})),",
                    f.tag(),
                    self.name,
                    self.get_modules(desc),
                    self.name,
                    f.name,
                    val
                )?;
            } else {
                writeln!(
                    w,
                    "                Ok({}) => msg.{} = {}OneOf{}::{}({}),",
                    f.tag(),
                    self.name,
                    self.get_modules(desc),
                    self.name,
                    f.name,
                    val_cow
                )?;
            }
        }
        Ok(())
    }

    fn write_get_size<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        writeln!(w, "        + match &self.{} {{", self.name)?;
        for f in self
            .fields
            .iter()
            .filter(|f| !f.deprecated || config.add_deprecated_fields)
        {
            let tag_size = sizeof_varint(f.tag());
            if f.typ.is_fixed_size() {
                writeln!(
                    w,
                    "            {}OneOf{}::{}(_) => {} + {},",
                    self.get_modules(desc),
                    self.name,
                    f.name,
                    tag_size,
                    f.typ.get_size("")
                )?;
            } else if f.typ.need_to_dereference() {
                writeln!(
                    w,
                    "            {}OneOf{}::{}(m) => {} + {},",
                    self.get_modules(desc),
                    self.name,
                    f.name,
                    tag_size,
                    f.typ.get_size("*m")
                )?;
            } else {
                writeln!(
                    w,
                    "            {}OneOf{}::{}(ref m) => {} + {},",
                    self.get_modules(desc),
                    self.name,
                    f.name,
                    tag_size,
                    f.typ.get_size("m")
                )?;
            }
        }
        writeln!(
            w,
            "            {}OneOf{}::None => 0,",
            self.get_modules(desc),
            self.name
        )?;
        writeln!(w, "        }}")?;
        Ok(())
    }

    fn write_write<W: Write>(
        &self,
        w: &mut W,
        desc: &FileDescriptor,
        config: &Config,
    ) -> Result<()> {
        writeln!(w, "        match &self.{} {{", self.name)?;
        for f in self
            .fields
            .iter()
            .filter(|f| !f.deprecated || config.add_deprecated_fields)
        {
            if f.typ.need_to_dereference() || f.boxed {
                writeln!(
                    w,
                    "            {}OneOf{}::{}(m) => {{ w.write_with_tag({}, |w| w.{})? }},",
                    self.get_modules(desc),
                    self.name,
                    f.name,
                    f.tag(),
                    f.typ.get_write("*m", f.boxed)
                )?;
            } else {
                writeln!(
                    w,
                    "            {}OneOf{}::{}(m) => {{ w.write_with_tag({}, |w| w.{})? }},",
                    self.get_modules(desc),
                    self.name,
                    f.name,
                    f.tag(),
                    f.typ.get_write("m", f.boxed)
                )?;
            }
        }
        writeln!(
            w,
            "            {}OneOf{}::None => {{}},",
            self.get_modules(desc),
            self.name
        )?;
        writeln!(w, "        }}")?;
        Ok(())
    }
}

pub struct Config {
    pub in_file: PathBuf,
    pub out_file: PathBuf,
    pub single_module: bool,
    pub import_search_path: Vec<PathBuf>,
    pub no_output: bool,
    pub error_cycle: bool,
    pub headers: bool,
    pub dont_use_cow: bool,
    pub custom_struct_derive: Vec<String>,
    pub custom_repr: Option<String>,
    pub custom_rpc_generator: RpcGeneratorFunction,
    pub custom_includes: Vec<String>,
    pub owned: bool,
    pub nostd: bool,
    pub hashbrown: bool,
    pub gen_write: bool,
    pub gen_info: bool,
    pub add_deprecated_fields: bool,
    pub generate_getters: bool,
}

#[derive(Debug, Default, Clone)]
pub struct FileDescriptor {
    pub import_paths: Vec<PathBuf>,
    pub package: String,
    pub syntax: Syntax,
    pub messages: Vec<Message>,
    pub message_extends: Vec<Extend>,
    pub enums: Vec<Enumerator>,
    pub module: String,
    pub rpc_services: Vec<RpcService>,
    pub owned: bool,
}

impl FileDescriptor {
    pub fn run(configs: &[Config]) -> Result<()> {
        for config in configs {
            Self::write_proto(config)?
        }
        Ok(())
    }

    pub fn write_proto(config: &Config) -> Result<()> {
        let mut desc = FileDescriptor::read_proto(&config.in_file, &config.import_search_path)?;
        desc.owned = config.owned;

        if desc.messages.is_empty() && desc.enums.is_empty() {
            // There could had been unsupported structures, so bail early
            return Err(Error::EmptyRead);
        }

        desc.resolve_types()?;
        desc.break_cycles(config.error_cycle)?;
        desc.sanity_checks()?;
        if config.dont_use_cow {
            desc.convert_field_types(&FieldType::StringCow, &FieldType::String_);
            desc.convert_field_types(&FieldType::BytesCow, &FieldType::Bytes_);
        }
        desc.set_defaults(config)?;
        desc.sanitize_names();

        if config.single_module {
            desc.package = "".to_string();
        }

        let (prefix, file_package) = split_package(&desc.package);

        let mut file_stem = if file_package.is_empty() {
            get_file_stem(&config.out_file)?
        } else {
            file_package.to_string()
        };

        if !file_package.is_empty() {
            sanitize_keyword(&mut file_stem);
        }
        let mut out_file = config.out_file.with_file_name(format!("{}.rs", file_stem));

        if !prefix.is_empty() {
            use std::fs::create_dir_all;
            // e.g. package is a.b; we need to create directory 'a' and insert it into the path
            let file = PathBuf::from(out_file.file_name().unwrap());
            out_file.pop();
            for p in prefix.split('.') {
                out_file.push(p);

                if !out_file.exists() {
                    create_dir_all(&out_file)?;
                    update_mod_file(&out_file)?;
                }
            }
            out_file.push(file);
        }
        if config.no_output {
            let imported = |b| if b { " imported" } else { "" };
            println!("source will be written to {}\n", out_file.display());
            for m in &desc.messages {
                println!(
                    "message {} module {}{}",
                    m.name,
                    m.module,
                    imported(m.imported)
                );
            }
            for e in &desc.enums {
                println!(
                    "enum {} module {}{}",
                    e.name,
                    e.module,
                    imported(e.imported)
                );
            }
            return Ok(());
        }

        let name = config.in_file.file_name().and_then(|e| e.to_str()).unwrap();
        let mut w = BufWriter::new(File::create(&out_file)?);
        desc.write(&mut w, name, config)?;
        update_mod_file(&out_file)
    }

    pub fn convert_field_types(&mut self, from: &FieldType, to: &FieldType) {
        // Messages and enums are the only structures with types
        for m in &mut self.messages {
            m.convert_field_types(from, to);
        }
    }

    /// Opens a proto file, reads it and returns raw parsed data
    pub fn read_proto(in_file: &Path, import_search_path: &[PathBuf]) -> Result<FileDescriptor> {
        let file = std::fs::read_to_string(in_file)?;
        let (rem, mut desc) = file_descriptor(&file).map_err(Error::Nom)?;
        let rem = rem.trim();
        if !rem.is_empty() {
            return Err(Error::TrailingGarbage(rem.chars().take(50).collect()));
        }
        for mut m in &mut desc.messages {
            if m.path.as_os_str().is_empty() {
                m.path = in_file.to_path_buf();
                if !import_search_path.is_empty() {
                    if let Ok(p) = m.path.clone().strip_prefix(&import_search_path[0]) {
                        m.import = p.to_path_buf();
                    }
                }
            }
        }
        // proto files with no packages are given an implicit module,
        // since every generated Rust source file represents a module
        desc.module = if desc.package.is_empty() {
            get_file_stem(in_file)?
        } else {
            desc.package.clone()
        };

        desc.fetch_imports(in_file, import_search_path)?;
        Ok(desc)
    }

    fn sanity_checks(&self) -> Result<()> {
        for m in &self.messages {
            m.sanity_checks(self)?;
        }
        Ok(())
    }

    /// Get messages and enums from imports
    fn fetch_imports(&mut self, in_file: &Path, import_search_path: &[PathBuf]) -> Result<()> {
        for m in &mut self.messages {
            m.set_package(&self.package, &self.module);
        }
        for m in &mut self.enums {
            m.set_package(&self.package, &self.module);
        }

        for import in &self.import_paths {
            // this is the same logic as the C preprocessor;
            // if the include path item is absolute, then append the filename,
            // otherwise it is always relative to the file.
            let mut matching_file = None;
            for path in import_search_path {
                let candidate = if path.is_absolute() {
                    path.join(import)
                } else {
                    in_file
                        .parent()
                        .map_or_else(|| path.join(import), |p| p.join(path).join(import))
                };
                if candidate.exists() {
                    matching_file = Some(candidate);
                    break;
                }
            }
            if matching_file.is_none() {
                return Err(Error::InvalidImport(format!(
                    "file {} not found on import path",
                    import.display()
                )));
            }
            let proto_file = matching_file.unwrap();
            let mut f = FileDescriptor::read_proto(&proto_file, import_search_path)?;

            // if the proto has a packge then the names will be prefixed
            let package = f.package.clone();
            let module = f.module.clone();
            self.messages.extend(f.messages.drain(..).map(|mut m| {
                if m.package.is_empty() {
                    m.set_package(&package, &module);
                }
                if m.path.as_os_str().is_empty() {
                    m.path = proto_file.clone();
                }
                if m.import.as_os_str().is_empty() {
                    m.import = import.clone();
                }
                m.set_imported();
                m
            }));
            self.enums.extend(f.enums.drain(..).map(|mut e| {
                if e.package.is_empty() {
                    e.set_package(&package, &module);
                }
                if e.path.as_os_str().is_empty() {
                    e.path = proto_file.clone();
                }
                if e.import.as_os_str().is_empty() {
                    e.import = import.clone();
                }
                e.imported = true;
                e
            }));
        }
        Ok(())
    }

    fn set_defaults(&mut self, config: &Config) -> Result<()> {
        // if proto3, then changes several defaults
        if let Syntax::Proto3 = self.syntax {
            for m in &mut self.messages {
                m.set_repeated_as_packed();
            }
        }
        // this is very inefficient but we don't care ...
        //let msgs = self.messages.clone();
        let copy = self.clone();
        for m in &mut self.messages {
            m.sanitize_defaults(&copy, config)?; //&msgs, &self.enums)?; ???
        }
        // force packed only on primitives
        for m in &mut self.messages {
            m.unset_packed_non_primitives();
        }
        Ok(())
    }

    fn sanitize_names(&mut self) {
        for m in &mut self.messages {
            m.sanitize_names();
        }
        for e in &mut self.enums {
            e.sanitize_names();
        }
    }

    /// Breaks cycles by adding boxes when necessary
    fn break_cycles(&mut self, error_cycle: bool) -> Result<()> {
        // get strongly connected components
        let sccs = self.sccs();

        fn is_cycle(scc: &[MessageIndex], desc: &FileDescriptor) -> bool {
            scc.iter()
                .map(|m| m.get_message(desc))
                .flat_map(|m| m.all_fields())
                .filter(|f| !f.boxed)
                .filter_map(|f| f.typ.message())
                .any(|m| scc.contains(m))
        }

        // sccs are sub DFS trees so if there is a edge connecting a node to
        // another node higher in the scc list, then this is a cycle. (Note that
        // we may have several cycles per scc).
        //
        // Technically we only need to box one edge (optional field) per cycle to
        // have Sized structs. Unfortunately, scc root depend on the order we
        // traverse the graph so such a field is not guaranteed to always be the same.
        //
        // For now, we decide (see discussion in #121) to box all optional fields
        // within a scc. We favor generated code stability over performance.
        for scc in &sccs {
            debug!("scc: {:?}", scc);
            for (i, v) in scc.iter().enumerate() {
                // cycles with v as root
                let cycles = v
                    .get_message(self)
                    .all_fields()
                    .filter_map(|f| f.typ.message())
                    .filter_map(|m| scc[i..].iter().position(|n| n == m))
                    .collect::<Vec<_>>();
                for cycle in cycles {
                    let cycle = &scc[i..i + cycle + 1];
                    debug!("cycle: {:?}", &cycle);
                    for v in cycle {
                        for f in v
                            .get_message_mut(self)
                            .all_fields_mut()
                            .filter(|f| f.frequency.is_optional())
                            .filter(|f| f.typ.message().map_or(false, |m| cycle.contains(m)))
                        {
                            f.boxed = true;
                        }
                    }
                    if is_cycle(cycle, self) {
                        if error_cycle {
                            return Err(Error::Cycle(
                                cycle
                                    .iter()
                                    .map(|m| m.get_message(self).name.clone())
                                    .collect(),
                            ));
                        } else {
                            for v in cycle {
                                warn!(
                                    "Unsound proto file would result in infinite size Messages.\n\
                                     Cycle detected in messages {:?}.\n\
                                     Modifying required fields into optional fields",
                                    cycle
                                        .iter()
                                        .map(|m| &m.get_message(self).name)
                                        .collect::<Vec<_>>()
                                );
                                for f in v
                                    .get_message_mut(self)
                                    .all_fields_mut()
                                    .filter(|f| {
                                        !(f.frequency.is_optional() || f.frequency.is_repeated())
                                    })
                                    .filter(|f| {
                                        f.typ.message().map_or(false, |m| cycle.contains(m))
                                    })
                                {
                                    f.boxed = true;
                                    f.frequency = match f.frequency {
                                        Frequency::Proto2Frequency(_) => {
                                            Frequency::Proto2Frequency(Proto2Frequency::Optional)
                                        }
                                        Frequency::Proto3Frequency(_) => {
                                            Frequency::Proto3Frequency(Proto3Frequency::Optional)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn get_full_names(&mut self) -> (HashMap<String, MessageIndex>, HashMap<String, EnumIndex>) {
        fn rec_full_names(
            m: &mut Message,
            index: &mut MessageIndex,
            full_msgs: &mut HashMap<String, MessageIndex>,
            full_enums: &mut HashMap<String, EnumIndex>,
        ) {
            m.index = index.clone();
            if m.package.is_empty() {
                full_msgs
                    .entry(m.name.clone())
                    .or_insert_with(|| index.clone());
            } else {
                full_msgs
                    .entry(format!("{}.{}", m.package, m.name))
                    .or_insert_with(|| index.clone());
            }
            for (i, e) in m.enums.iter_mut().enumerate() {
                let index = EnumIndex {
                    msg_index: index.clone(),
                    index: i,
                };
                e.index = index.clone();
                full_enums
                    .entry(format!("{}.{}", e.package, e.name))
                    .or_insert(index);
            }
            for (i, m) in m.messages.iter_mut().enumerate() {
                index.push(i);
                rec_full_names(m, index, full_msgs, full_enums);
                index.pop();
            }
        }

        let mut full_msgs = HashMap::new();
        let mut full_enums = HashMap::new();
        let mut index = MessageIndex { indexes: vec![] };
        for (i, m) in self.messages.iter_mut().enumerate() {
            index.push(i);
            rec_full_names(m, &mut index, &mut full_msgs, &mut full_enums);
            index.pop();
        }
        for (i, e) in self.enums.iter_mut().enumerate() {
            let index = EnumIndex {
                msg_index: index.clone(),
                index: i,
            };
            e.index = index.clone();
            if e.package.is_empty() {
                full_enums
                    .entry(e.name.clone())
                    .or_insert_with(|| index.clone());
            } else {
                full_enums
                    .entry(format!("{}.{}", e.package, e.name))
                    .or_insert_with(|| index.clone());
            }
        }
        (full_msgs, full_enums)
    }

    fn resolve_types(&mut self) -> Result<()> {
        let (full_msgs, full_enums) = self.get_full_names();

        fn rec_resolve_types(
            m: &mut Message,
            full_msgs: &HashMap<String, MessageIndex>,
            full_enums: &HashMap<String, EnumIndex>,
        ) -> Result<()> {
            // Interestingly, we can't call all_fields_mut to iterate over the
            // fields here: writing out the field traversal as below lets Rust
            // split m's mutable borrow, permitting the loop body to use fields
            // of `m` other than `fields` and `oneofs`.
            'types: for typ in m
                .fields
                .iter_mut()
                .chain(m.oneofs.iter_mut().flat_map(|o| o.fields.iter_mut()))
                .map(|f| &mut f.typ)
                .flat_map(|typ| match *typ {
                    FieldType::Map(ref mut key, ref mut value) => {
                        vec![&mut **key, &mut **value].into_iter()
                    }
                    _ => vec![typ].into_iter(),
                })
            {
                if let FieldType::MessageOrEnum(name) = typ.clone() {
                    let test_names: Vec<String> = if name.starts_with('.') {
                        vec![name.clone().split_off(1)]
                    } else if m.package.is_empty() {
                        vec![format!("{}.{}", m.name, name), name.clone()]
                    } else {
                        let mut v = vec![
                            format!("{}.{}.{}", m.package, m.name, name),
                            format!("{}.{}", m.package, name),
                        ];
                        for (index, _) in m.package.match_indices('.').rev() {
                            v.push(format!("{}.{}", &m.package[..index], name));
                        }
                        v.push(name.clone());
                        v
                    };
                    for name in &test_names {
                        if let Some(msg) = full_msgs.get(name) {
                            *typ = FieldType::Message(msg.clone());
                            continue 'types;
                        } else if let Some(e) = full_enums.get(name) {
                            *typ = FieldType::Enum(e.clone());
                            continue 'types;
                        }
                    }
                    return Err(Error::MessageOrEnumNotFound(name));
                }
            }
            for m in m.messages.iter_mut() {
                rec_resolve_types(m, full_msgs, full_enums)?;
            }
            Ok(())
        }

        for m in self.messages.iter_mut() {
            rec_resolve_types(m, &full_msgs, &full_enums)?;
        }
        Ok(())
    }

    fn write<W: Write>(&self, w: &mut W, filename: &str, config: &Config) -> Result<()> {
        println!(
            "Found {} messages, and {} enums",
            self.messages.len(),
            self.enums.len()
        );
        if config.headers {
            self.write_headers(w, filename, config)?;
        }
        self.write_package_start(w)?;
        self.write_uses(w, config)?;
        self.write_imports(w)?;
        self.write_enums(w)?;
        self.write_messages(w, config)?;
        self.write_rpc_services(w, config)?;
        self.write_package_end(w)?;
        Ok(())
    }

    fn write_headers<W: Write>(&self, w: &mut W, filename: &str, config: &Config) -> Result<()> {
        writeln!(
            w,
            "// Automatically generated rust module for '{}' file",
            filename
        )?;
        writeln!(w)?;
        writeln!(w, "#![allow(non_snake_case)]")?;
        writeln!(w, "#![allow(non_upper_case_globals)]")?;
        writeln!(w, "#![allow(non_camel_case_types)]")?;
        writeln!(w, "#![allow(unused_imports)]")?;
        writeln!(w, "#![allow(unknown_lints)]")?;
        writeln!(w, "#![allow(clippy::all)]")?;

        if config.add_deprecated_fields {
            writeln!(w, "#![allow(deprecated)]")?;
        }

        writeln!(w, "#![cfg_attr(rustfmt, rustfmt_skip)]")?;
        writeln!(w)?;
        Ok(())
    }

    fn write_package_start<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w)?;
        Ok(())
    }

    fn write_uses<W: Write>(&self, w: &mut W, config: &Config) -> Result<()> {
        if self.messages.iter().all(|m| m.is_unit()) {
            writeln!(
                w,
                "use quick_protobuf::{{BytesReader, Result, MessageInfo, MessageRead, MessageWrite}};"
            )?;
            if self.owned {
                writeln!(w, "use core::convert::{{TryFrom, TryInto}};")?;
            }
            return Ok(());
        }

        Message::write_common_uses(w, &self.messages, self, config)?;

        writeln!(
            w,
            "use quick_protobuf::{{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result, PackedFixed, PackedFixedIntoIter, PackedFixedRefIter}};"
        )?;

        if self.owned {
            writeln!(w, "use core::convert::{{TryFrom, TryInto}};")?;
        }

        writeln!(w, "use quick_protobuf::sizeofs::*;")?;
        for include in &config.custom_includes {
            writeln!(w, "{}", include)?;
        }
        Ok(())
    }

    fn write_imports<W: Write>(&self, w: &mut W) -> Result<()> {
        // even if we don't have an explicit package, there is an implicit Rust module
        // This `use` allows us to refer to the package root.
        // NOTE! I'm suppressing not-needed 'use super::*' errors currently!
        let mut depth = self.package.split('.').count();
        if depth == 0 {
            depth = 1;
        }
        write!(w, "use ")?;
        for _ in 0..depth {
            write!(w, "super::")?;
        }
        writeln!(w, "*;")?;
        Ok(())
    }

    fn write_package_end<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w)?;
        Ok(())
    }

    fn write_enums<W: Write>(&self, w: &mut W) -> Result<()> {
        for m in self.enums.iter().filter(|e| !e.imported) {
            println!("Writing enum {}", m.name);
            writeln!(w)?;
            m.write_definition(w)?;
            writeln!(w)?;
            m.write_impl_default(w)?;
            writeln!(w)?;
            m.write_from_i32(w)?;
            writeln!(w)?;
            m.write_from_str(w)?;
        }
        Ok(())
    }

    fn write_rpc_services<W: Write>(&self, w: &mut W, config: &Config) -> Result<()> {
        for m in self.rpc_services.iter() {
            println!("Writing Rpc {}", m.service_name);
            writeln!(w)?;
            m.write_definition(w, config)?;
        }
        Ok(())
    }

    fn write_messages<W: Write>(&self, w: &mut W, config: &Config) -> Result<()> {
        for m in self.messages.iter().filter(|m| !m.imported) {
            m.write(w, self, config)?;
        }
        Ok(())
    }
}

/// Calculates the tag value
fn tag(number: u32, typ: &FieldType, packed: bool) -> u32 {
    number << 3 | typ.wire_type_num(packed)
}

/// "" is ("",""), "a" is ("","a"), "a.b" is ("a"."b"), and so forth.
fn split_package(package: &str) -> (&str, &str) {
    if package.is_empty() {
        ("", "")
    } else if let Some(i) = package.rfind('.') {
        (&package[0..i], &package[i + 1..])
    } else {
        ("", package)
    }
}

const MAGIC_HEADER: &str = "// Automatically generated mod.rs";

/// Given a file path, create or update the mod.rs file within its folder
fn update_mod_file(path: &Path) -> Result<()> {
    let mut file = path.to_path_buf();
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    let name = file.file_stem().unwrap().to_string_lossy().to_string();
    file.pop();
    file.push("mod.rs");
    let matches = "pub mod ";
    let mut present = false;
    let mut exists = false;
    if let Ok(f) = File::open(&file) {
        exists = true;
        let mut first = true;
        for line in BufReader::new(f).lines() {
            let line = line?;
            if first {
                if !line.contains(MAGIC_HEADER) {
                    // it is NOT one of our generated mod.rs files, so don't modify it!
                    present = true;
                    break;
                }
                first = false;
            }
            if let Some(i) = line.find(matches) {
                let rest = &line[i + matches.len()..line.len() - 1];
                if rest == name {
                    // we already have a reference to this module...
                    present = true;
                    break;
                }
            }
        }
    }
    if !present {
        let mut f = if exists {
            OpenOptions::new().append(true).open(&file)?
        } else {
            let mut f = File::create(&file)?;
            writeln!(f, "{}", MAGIC_HEADER)?;
            f
        };

        writeln!(f, "pub mod {};", name)?;
    }
    Ok(())
}

/// get the proper sanitized file stem from an input file path
fn get_file_stem(path: &Path) -> Result<String> {
    let mut file_stem = path
        .file_stem()
        .and_then(|f| f.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| Error::OutputFile(format!("{}", path.display())))?;

    file_stem = file_stem.replace(|c: char| !c.is_alphanumeric(), "_");
    // will now be properly alphanumeric, but may be a keyword!
    sanitize_keyword(&mut file_stem);
    Ok(file_stem)
}
