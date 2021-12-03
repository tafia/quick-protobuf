use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Syntax {
    Proto2,
    Proto3,
}

impl Default for Syntax {
    fn default() -> Syntax {
        Syntax::Proto2
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Frequency {
    Optional,
    Repeated,
    Required,
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
    pub fn is_primitive(&self) -> bool {
        match *self {
            FieldType::Message(_)
            | FieldType::Map(_, _)
            | FieldType::StringCow
            | FieldType::BytesCow
            | FieldType::String_
            | FieldType::Bytes_ => false,
            _ => true,
        }
    }

    fn has_cow(&self) -> bool {
        match *self {
            FieldType::BytesCow | FieldType::StringCow => true,
            FieldType::Map(ref k, ref v) => k.has_cow() || v.has_cow(),
            _ => false,
        }
    }

    fn has_bytes_and_string(&self) -> bool {
        match *self {
            FieldType::Bytes_ | FieldType::String_ => true,
            _ => false,
        }
    }

    fn is_map(&self) -> bool {
        match *self {
            FieldType::Map(_, _) => true,
            _ => false,
        }
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
        match self.wire_type_num_non_packed() {
            1 | 5 => true,
            _ => false,
        }
    }

    fn regular_default<'a, 'b>(&'a self, desc: &'b FileDescriptor) -> Option<&'b str> {
        match *self {
            FieldType::Int32 => Some("0i32"),
            FieldType::Sint32 => Some("0i32"),
            FieldType::Int64 => Some("0i64"),
            FieldType::Sint64 => Some("0i64"),
            FieldType::Uint32 => Some("0u32"),
            FieldType::Uint64 => Some("0u64"),
            FieldType::Bool => Some("false"),
            FieldType::Fixed32 => Some("0u32"),
            FieldType::Sfixed32 => Some("0i32"),
            FieldType::Float => Some("0f32"),
            FieldType::Fixed64 => Some("0u64"),
            FieldType::Sfixed64 => Some("0i64"),
            FieldType::Double => Some("0f64"),
            FieldType::StringCow => Some("\"\""),
            FieldType::BytesCow => Some("Cow::Borrowed(b\"\")"),
            FieldType::String_ => Some("String::default()"),
            FieldType::Bytes_ => Some("vec![]"),
            FieldType::Enum(ref e) => {
                let e = e.get_enum(desc);
                Some(&*e.fully_qualified_fields[0].0)
            }
            FieldType::Message(_) => None,
            FieldType::Map(_, _) => None,
            FieldType::MessageOrEnum(_) => unreachable!("Message / Enum not resolved"),
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
        packed: bool,
        ignore: &mut Vec<MessageIndex>,
    ) -> bool {
        match *self {
            FieldType::StringCow | FieldType::BytesCow => true, // Cow<[u8]>
            FieldType::Message(ref m) => m.get_message(desc).has_lifetime(desc, ignore),
            FieldType::Fixed64
            | FieldType::Sfixed64
            | FieldType::Double
            | FieldType::Fixed32
            | FieldType::Sfixed32
            | FieldType::String_
            | FieldType::Bytes_
            | FieldType::Float => packed, // Cow<[M]>
            FieldType::Map(ref key, ref value) => {
                key.has_lifetime(desc, false, ignore) || value.has_lifetime(desc, false, ignore)
            }
            _ => false,
        }
    }

    fn rust_type(&self, desc: &FileDescriptor) -> Result<String> {
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
                let lifetime = if m.has_lifetime(desc, &mut Vec::new()) {
                    "<'a>"
                } else {
                    ""
                };
                format!("{}{}{}", m.get_modules(desc), m.name, lifetime)
            }
            FieldType::Map(ref key, ref value) => format!(
                "KVMap<{}, {}>",
                key.rust_type(desc)?,
                value.rust_type(desc)?
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
            | FieldType::Enum(_) => format!("sizeof_varint(*({}) as u64)", s),
            FieldType::Sint32 => format!("sizeof_sint32(*({}))", s),
            FieldType::Sint64 => format!("sizeof_sint64(*({}))", s),

            FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double => "8".to_string(),
            FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => "4".to_string(),

            FieldType::StringCow | FieldType::BytesCow => format!("sizeof_len(({}).len())", s),

            FieldType::String_ | FieldType::Bytes_ => format!("sizeof_len(({}).len())", s),

            FieldType::Message(_) => format!("sizeof_len(({}).get_size())", s),

            FieldType::Map(ref k, ref v) => {
                format!("2 + {} + {}", k.get_size("k"), v.get_size("v"))
            }
            FieldType::MessageOrEnum(_) => unreachable!("Message / Enum not resolved"),
        }
    }

    fn get_write(&self, s: &str, boxed: bool) -> String {
        match *self {
            FieldType::Enum(_) => format!("write_enum(*{} as i32)", s),

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
            | FieldType::Float => format!("write_{}(*{})", self.proto_type(), s),

            FieldType::StringCow => format!("write_string(&**{})", s),
            FieldType::BytesCow => format!("write_bytes(&**{})", s),
            FieldType::String_ => format!("write_string(&**{})", s),
            FieldType::Bytes_ => format!("write_bytes(&**{})", s),

            FieldType::Message(_) if boxed => format!("write_message(&**{})", s),
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
    fn packed(&self) -> bool {
        self.packed.unwrap_or(false)
    }

    fn sanitize_default(&mut self, desc: &FileDescriptor) -> Result<()> {
        if let Some(ref mut d) = self.default {
            *d = match &*self.typ.rust_type(desc)? {
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
                "Cow<'a, str>" => format!("Cow::Borrowed({})", d),
                "Cow<'a, [u8]>" => format!("Cow::Borrowed(b{})", d),
                "String" => format!("String::from({})", d),
                "Bytes" => format!(r#"b{}"#, d),
                "Vec<u8>" => format!("b{}.to_vec()", d),
                "bool" => format!("{}", d.parse::<bool>().unwrap()),
                e => format!("{}::{}", e, d), // enum, as message and map do not have defaults
            }
        }
        Ok(())
    }

    fn has_regular_default(&self, desc: &FileDescriptor) -> bool {
        self.default.is_none()
            || self.default.as_ref().map(|d| &**d) == self.typ.regular_default(desc)
    }

    fn tag(&self) -> u32 {
        tag(self.number as u32, &self.typ, self.packed())
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
        write!(w, "    pub {}: ", self.name)?;
        let rust_type = self.typ.rust_type(desc)?;
        match self.frequency {
            _ if self.boxed => writeln!(w, "Option<Box<{}>>,", rust_type)?,
            Frequency::Optional
                if desc.syntax == Syntax::Proto2 && self.default.is_none()
                    || self.typ.message().is_some() =>
            {
                writeln!(w, "Option<{}>,", rust_type)?
            }
            Frequency::Repeated
                if self.packed() && self.typ.is_fixed_size() && !config.dont_use_cow =>
            {
                writeln!(w, "Cow<'a, [{}]>,", rust_type)?;
            }
            Frequency::Repeated => writeln!(w, "Vec<{}>,", rust_type)?,
            Frequency::Required | Frequency::Optional => writeln!(w, "{},", rust_type)?,
        }
        Ok(())
    }

    fn write_match_tag<W: Write>(&self, w: &mut W, desc: &FileDescriptor, config: &Config) -> Result<()> {
        if self.deprecated && !config.add_deprecated_fields {
            return Ok(());
        }

        // special case for FieldType::Map: destructure tuple before inserting in HashMap
        if let FieldType::Map(ref key, ref value) = self.typ {
            writeln!(w, "                Ok({}) => {{", self.tag())?;
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
        }

        let (val, val_cow) = self.typ.read_fn(desc)?;
        let name = &self.name;
        write!(w, "                Ok({}) => ", self.tag())?;
        match self.frequency {
            _ if self.boxed => writeln!(w, "msg.{} = Some(Box::new({})),", name, val)?,
            Frequency::Optional
                if desc.syntax == Syntax::Proto2 && self.default.is_none()
                    || self.typ.message().is_some() =>
            {
                writeln!(w, "msg.{} = Some({}),", name, val_cow)?
            }
            Frequency::Required | Frequency::Optional => {
                writeln!(w, "msg.{} = {},", name, val_cow)?
            }
            Frequency::Repeated if self.packed() && self.typ.is_fixed_size() => {
                writeln!(w, "msg.{} = r.read_packed_fixed(bytes)?.into(),", name)?;
            }
            Frequency::Repeated if self.packed() => {
                writeln!(
                    w,
                    "msg.{} = r.read_packed(bytes, |r, bytes| Ok({}))?,",
                    name, val_cow
                )?;
            }
            Frequency::Repeated => writeln!(w, "msg.{}.push({}),", name, val_cow)?,
        }
        Ok(())
    }

    fn write_get_size<W: Write>(&self, w: &mut W, desc: &FileDescriptor, config: &Config) -> Result<()> {
        if self.deprecated && !config.add_deprecated_fields {
            return Ok(());
        }

        write!(w, "        + ")?;
        let tag_size = sizeof_varint(self.tag());
        match self.frequency {
            Frequency::Optional
                if desc.syntax == Syntax::Proto2 || self.typ.message().is_some() =>
            {
                // TODO this might be incorrect behavior for proto2
                match self.default.as_ref() {
                    None => {
                        write!(w, "self.{}.as_ref().map_or(0, ", self.name)?;
                        if self.typ.is_fixed_size() {
                            writeln!(w, "|_| {} + {})", tag_size, self.typ.get_size(""))?;
                        } else {
                            writeln!(w, "|m| {} + {})", tag_size, self.typ.get_size("m"))?;
                        }
                    }
                    Some(d) => {
                        writeln!(
                            w,
                            "if self.{} == {} {{ 0 }} else {{ {} + {} }}",
                            self.name,
                            d,
                            tag_size,
                            self.typ.get_size(&format!("&self.{}", self.name))
                        )?;
                    }
                }
            }
            Frequency::Required if self.typ.is_map() => {
                writeln!(
                    w,
                    "self.{}.iter().map(|(k, v)| {} + sizeof_len({})).sum::<usize>()",
                    self.name,
                    tag_size,
                    self.typ.get_size("")
                )?;
            }
            Frequency::Optional => match self.typ {
                FieldType::Bytes_ => writeln!(
                    w,
                    "if self.{}.is_empty() {{ 0 }} else {{ {} + {} }}",
                    self.name,
                    tag_size,
                    self.typ.get_size(&format!("&self.{}", self.name))
                )?,
                _ => writeln!(
                    w,
                    "if self.{} == {} {{ 0 }} else {{ {} + {} }}",
                    self.name,
                    self.default.as_ref().map_or_else(
                        || self.typ.regular_default(desc).unwrap_or("None"),
                        |s| s.as_str()
                    ),
                    tag_size,
                    self.typ.get_size(&format!("&self.{}", self.name))
                )?,
            },
            Frequency::Required => writeln!(
                w,
                "{} + {}",
                tag_size,
                self.typ.get_size(&format!("&self.{}", self.name))
            )?,
            Frequency::Repeated => {
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
                            "sizeof_len(self.{}.iter().map(|s| {}).sum::<usize>()) }}",
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
                            "self.{}.iter().map(|s| {} + {}).sum::<usize>()",
                            self.name,
                            tag_size,
                            self.typ.get_size("s")
                        )?,
                    }
                }
            }
        }
        Ok(())
    }

    fn write_write<W: Write>(&self, w: &mut W, desc: &FileDescriptor, config: &Config) -> Result<()> {
        if self.deprecated && !config.add_deprecated_fields {
            return Ok(());
        }

        match self.frequency {
            Frequency::Optional
                if desc.syntax == Syntax::Proto2 || self.typ.message().is_some() =>
            {
                match self.default.as_ref() {
                    None => {
                        writeln!(
                            w,
                            "        if let Some(ref s) = \
                             self.{} {{ w.write_with_tag({}, |w| w.{})?; }}",
                            self.name,
                            self.tag(),
                            self.typ.get_write("s", self.boxed)
                        )?;
                    }
                    Some(d) => {
                        writeln!(
                            w,
                            "        if self.{} != {} {{ w.write_with_tag({}, |w| w.{})?; }}",
                            self.name,
                            d,
                            self.tag(),
                            self.typ
                                .get_write(&format!("&self.{}", self.name), self.boxed)
                        )?;
                    }
                }
            }
            Frequency::Optional => match self.typ {
                FieldType::Bytes_ => {
                    writeln!(
                        w,
                        "        if !self.{}.is_empty() {{ w.write_with_tag({}, |w| w.{})?; }}",
                        self.name,
                        self.tag(),
                        self.typ
                            .get_write(&format!("&self.{}", self.name), self.boxed)
                    )?;
                }
                _ => {
                    writeln!(
                        w,
                        "        if self.{} != {} {{ w.write_with_tag({}, |w| w.{})?; }}",
                        self.name,
                        self.default.as_ref().map_or_else(
                            || self.typ.regular_default(desc).unwrap_or("None"),
                            |s| s.as_str()
                        ),
                        self.tag(),
                        self.typ
                            .get_write(&format!("&self.{}", self.name), self.boxed)
                    )?;
                }
            },
            Frequency::Required if self.typ.is_map() => {
                writeln!(
                    w,
                    "        for (k, v) in self.{}.iter() {{ w.write_with_tag({}, |w| w.{})?; }}",
                    self.name,
                    self.tag(),
                    self.typ.get_write("", false)
                )?;
            }
            Frequency::Required => {
                writeln!(
                    w,
                    "        w.write_with_tag({}, |w| w.{})?;",
                    self.tag(),
                    self.typ
                        .get_write(&format!("&self.{}", self.name), self.boxed)
                )?;
            }
            Frequency::Repeated if self.packed() && self.typ.is_fixed_size() => writeln!(
                w,
                "        w.write_packed_fixed_with_tag({}, &self.{})?;",
                self.tag(),
                self.name
            )?,
            Frequency::Repeated if self.packed() => writeln!(
                w,
                "        w.write_packed_with_tag({}, &self.{}, |w, m| w.{}, &|m| {})?;",
                self.tag(),
                self.name,
                self.typ.get_write("m", self.boxed),
                self.typ.get_size("m")
            )?,
            Frequency::Repeated => {
                writeln!(
                    w,
                    "        for s in &self.{} {{ w.write_with_tag({}, |w| w.{})?; }}",
                    self.name,
                    self.tag(),
                    self.typ.get_write("s", self.boxed)
                )?;
            }
        }
        Ok(())
    }
}

fn get_modules(module: &str, imported: bool, desc: &FileDescriptor) -> String {
    let skip = if desc.package.is_empty() && !imported {
        1
    } else {
        0
    };
    module
        .split('.')
        .filter(|p| !p.is_empty())
        .skip(skip)
        .map(|p| format!("{}::", p))
        .collect()
}

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
}

impl Message {
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

    fn has_lifetime(&self, desc: &FileDescriptor, ignore: &mut Vec<MessageIndex>) -> bool {
        if ignore.contains(&&self.index) {
            return false;
        }
        ignore.push(self.index.clone());
        let res = self
            .all_fields()
            .any(|f| f.typ.has_lifetime(desc, f.packed(), ignore));
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
        self.fields.is_empty() && self.oneofs.is_empty()
    }

    fn write_common_uses<W: Write>(
        w: &mut W,
        messages: &Vec<Message>,
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
        } else if config.nostd {
            if messages
                .iter()
                .any(|m| m.all_fields().any(|f| (f.typ.has_bytes_and_string())))
            {
                writeln!(w, "use alloc::borrow::ToOwned;")?;
            }
        }

        if config.nostd
            && messages.iter().any(|m| {
                desc.owned && m.has_lifetime(desc, &mut Vec::new())
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

    fn write<W: Write>(&self, w: &mut W, desc: &FileDescriptor, config: &Config) -> Result<()> {
        println!("Writing message {}{}", self.get_modules(desc), self.name);
        writeln!(w)?;

        self.write_definition(w, desc, config)?;
        writeln!(w)?;
        self.write_impl_message_read(w, desc, config)?;
        writeln!(w)?;
        self.write_impl_message_write(w, desc, config)?;

        if config.gen_info {
            self.write_impl_message_info(w, desc, config)?;
            writeln!(w)?;
        }

        if desc.owned && self.has_lifetime(desc, &mut Vec::new()) {
            writeln!(w)?;
            self.write_impl_owned(w, config)?;
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
        if !custom_struct_derive.is_empty() {
            custom_struct_derive += ", ";
        }

        writeln!(
            w,
            "#[derive({}Debug, Default, PartialEq, Clone)]",
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
        if self.has_lifetime(desc, &mut ignore) {
            writeln!(w, "pub struct {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "pub struct {} {{", self.name)?;
        }
        for f in &self.fields {
            f.write_definition(w, desc, config)?;
        }
        for o in &self.oneofs {
            o.write_message_definition(w, desc)?;
        }
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
        if self.has_lifetime(desc, &mut ignore) {
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
        if self.has_lifetime(desc, &mut ignore) {
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

        let unregular_defaults = self
            .fields
            .iter()
            .filter(|f| !f.has_regular_default(desc))
            .collect::<Vec<_>>();
        if unregular_defaults.is_empty() {
            writeln!(w, "        let mut msg = Self::default();")?;
        } else {
            writeln!(w, "        let mut msg = {} {{", self.name)?;
            for f in unregular_defaults {
                writeln!(
                    w,
                    "            {}: {},",
                    f.name,
                    f.default.as_ref().unwrap()
                )?;
            }
            writeln!(w, "            ..Self::default()")?;
            writeln!(w, "        }};")?;
        }
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

        // TODO: write impl default when special default?
        // alternatively set the default value directly when reading

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
        if self.has_lifetime(desc, &mut ignore) {
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
            #[derive(Debug)]
            struct {name}OwnedInner {{
                buf: Vec<u8>,
                proto: {name}<'static>,
                _pin: core::marker::PhantomPinned,
            }}

            impl {name}OwnedInner {{
                fn new(buf: Vec<u8>) -> Result<core::pin::Pin<Box<Self>>> {{
                    let inner = Self {{
                        buf,
                        proto: unsafe {{ core::mem::MaybeUninit::zeroed().assume_init() }},
                        _pin: core::marker::PhantomPinned,
                    }};
                    let mut pinned = Box::pin(inner);

                    let mut reader = BytesReader::from_bytes(&pinned.buf);
                    let proto = {name}::from_reader(&mut reader, &pinned.buf)?;

                    unsafe {{
                        let proto = core::mem::transmute::<_, {name}<'static>>(proto);
                        pinned.as_mut().get_unchecked_mut().proto = proto;
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

                pub fn proto(&self) -> &{name} {{
                    &self.inner.proto
                }}
            }}

            impl core::fmt::Debug for {name}Owned {{
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                    self.inner.proto.fmt(f)
                }}
            }}

            impl Deref for {name}Owned {{
                type Target = {name}<'static>;

                fn deref(&self) -> &Self::Target {{
                    &self.inner.proto
                }}
            }}

            impl DerefMut for {name}Owned {{
                fn deref_mut(&mut self) -> &mut Self::Target {{
                    unsafe {{ &mut self.inner.as_mut().get_unchecked_mut().proto }}
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
                    self.deref().write_message(&mut writer)?;
                    Ok(buf)
                }}
            }}

            #[cfg(feature = "test_helpers")]
            impl<'a> From<{name}<'a>> for {name}Owned {{
                fn from(proto: {name}) -> Self {{
                    use quick_protobuf::{{MessageWrite, Writer}};

                    let mut buf = Vec::new();
                    let mut writer = Writer::new(&mut buf);
                    proto.write_message(&mut writer).expect("bad proto serialization");
                    Self {{ inner: {name}OwnedInner::new(buf).unwrap() }}
                }}
            }}
            "#,
            name = self.name
        )?;

        if config.gen_info {
            write!(w, r#"
            impl MessageInfo for {name}Owned {{
                const PATH: &'static str = "{module}.{name}";
            }}
            "#, name = self.name, module = self.module)?;
        }
        Ok(())
    }

    fn write_get_size<W: Write>(&self, w: &mut W, desc: &FileDescriptor, config: &Config) -> Result<()> {
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

    fn write_write_message<W: Write>(&self, w: &mut W, desc: &FileDescriptor, config: &Config) -> Result<()> {
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
        let (child_package, child_module) = if package.is_empty() && module.is_empty() {
            (self.name.clone(), format!("mod_{}", self.name))
        } else if package.is_empty() {
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

    fn set_map_required(&mut self) {
        for f in self.all_fields_mut() {
            if let FieldType::Map(_, _) = f.typ {
                f.frequency = Frequency::Required;
            }
        }
        for m in &mut self.messages {
            m.set_map_required();
        }
    }

    fn set_repeated_as_packed(&mut self) {
        for f in self.all_fields_mut() {
            if f.packed.is_none() {
                if let Frequency::Repeated = f.frequency {
                    f.packed = Some(true);
                }
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

    fn sanitize_defaults(&mut self, desc: &FileDescriptor) -> Result<()> {
        for f in self.all_fields_mut() {
            f.sanitize_default(desc)?;
        }
        for m in &mut self.messages {
            m.sanitize_defaults(desc)?;
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
                    format!("{}::{}", self.module.replace(".", "::"), pqf.0)
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
    fn has_lifetime(&self, desc: &FileDescriptor) -> bool {
        self.fields
            .iter()
            .any(|f| !f.deprecated && f.typ.has_lifetime(desc, f.packed(), &mut Vec::new()))
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
        self.write_impl_default(w, desc)?;
        Ok(())
    }

    fn write_definition<W: Write>(&self, w: &mut W, desc: &FileDescriptor, config: &Config) -> Result<()> {
        writeln!(w, "#[derive(Debug, PartialEq, Clone)]")?;
        if self.has_lifetime(desc) {
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

            let rust_type = f.typ.rust_type(desc)?;
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
        for f in self.fields.iter().filter(|f| !f.deprecated || config.add_deprecated_fields) {
            let rust_type = f.typ.rust_type(desc)?;
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

    fn write_impl_default<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        if self.has_lifetime(desc) {
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

    fn write_message_definition<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        if self.has_lifetime(desc) {
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

    fn write_match_tag<W: Write>(&self, w: &mut W, desc: &FileDescriptor, config: &Config) -> Result<()> {
        for f in self.fields.iter().filter(|f| !f.deprecated || config.add_deprecated_fields) {
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

    fn write_get_size<W: Write>(&self, w: &mut W, desc: &FileDescriptor, config: &Config) -> Result<()> {
        writeln!(w, "        + match self.{} {{", self.name)?;
        for f in self.fields.iter().filter(|f| !f.deprecated || config.add_deprecated_fields) {
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
        write!(w, "    }}")?;
        Ok(())
    }

    fn write_write<W: Write>(&self, w: &mut W, desc: &FileDescriptor, config: &Config) -> Result<()> {
        write!(w, "        match self.{} {{", self.name)?;
        for f in self.fields.iter().filter(|f| !f.deprecated || config.add_deprecated_fields) {
            writeln!(
                w,
                "            {}OneOf{}::{}(ref m) => {{ w.write_with_tag({}, |w| w.{})? }},",
                self.get_modules(desc),
                self.name,
                f.name,
                f.tag(),
                f.typ.get_write("m", f.boxed)
            )?;
        }
        writeln!(
            w,
            "            {}OneOf{}::None => {{}},",
            self.get_modules(desc),
            self.name
        )?;
        write!(w, "    }}")?;
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
    pub gen_info: bool,
    pub add_deprecated_fields: bool,
}

#[derive(Debug, Default, Clone)]
pub struct FileDescriptor {
    pub import_paths: Vec<PathBuf>,
    pub package: String,
    pub syntax: Syntax,
    pub messages: Vec<Message>,
    pub enums: Vec<Enumerator>,
    pub module: String,
    pub rpc_services: Vec<RpcService>,
    pub owned: bool,
}

impl FileDescriptor {
    pub fn run(configs: &[Config]) -> Result<()> {
        for config in configs {
            Self::write_proto(&config)?
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
        desc.set_defaults()?;
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
        let mut buf = Vec::new();
        {
            let f = File::open(in_file)?;
            let mut reader = BufReader::new(f);
            reader.read_to_end(&mut buf)?;
        }
        let mut desc = file_descriptor(&buf).to_result().map_err(Error::Nom)?;
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

        desc.fetch_imports(&in_file, import_search_path)?;
        Ok(desc)
    }

    fn sanity_checks(&self) -> Result<()> {
        for m in &self.messages {
            m.sanity_checks(&self)?;
        }
        Ok(())
    }

    /// Get messages and enums from imports
    fn fetch_imports(&mut self, in_file: &Path, import_search_path: &[PathBuf]) -> Result<()> {
        for m in &mut self.messages {
            m.set_package("", &self.module);
        }
        for m in &mut self.enums {
            m.set_package("", &self.module);
        }

        for import in &self.import_paths {
            // this is the same logic as the C preprocessor;
            // if the include path item is absolute, then append the filename,
            // otherwise it is always relative to the file.
            let mut matching_file = None;
            for path in import_search_path {
                let candidate = if path.is_absolute() {
                    path.join(&import)
                } else {
                    in_file
                        .parent()
                        .map_or_else(|| path.join(&import), |p| p.join(path).join(&import))
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

    fn set_defaults(&mut self) -> Result<()> {
        // set map fields as required (they are equivalent to repeated message)
        for m in &mut self.messages {
            m.set_map_required();
        }
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
            m.sanitize_defaults(&copy)?; //&msgs, &self.enums)?; ???
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
                            .filter(|f| f.frequency == Frequency::Optional)
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
                                    .filter(|f| f.frequency == Frequency::Required)
                                    .filter(|f| {
                                        f.typ.message().map_or(false, |m| cycle.contains(m))
                                    })
                                {
                                    f.boxed = true;
                                    f.frequency = Frequency::Optional;
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
                full_msgs.insert(m.name.clone(), index.clone());
            } else {
                full_msgs.insert(format!("{}.{}", m.package, m.name), index.clone());
            }
            for (i, e) in m.enums.iter_mut().enumerate() {
                let index = EnumIndex {
                    msg_index: index.clone(),
                    index: i,
                };
                e.index = index.clone();
                full_enums.insert(format!("{}.{}", e.package, e.name), index);
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
                full_enums.insert(e.name.clone(), index.clone());
            } else {
                full_enums.insert(format!("{}.{}", e.package, e.name), index.clone());
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
                        vec![name.clone(), format!("{}.{}", m.name, name)]
                    } else {
                        vec![
                            name.clone(),
                            format!("{}.{}", m.package, name),
                            format!("{}.{}.{}", m.package, m.name, name),
                        ]
                    };
                    for name in &test_names {
                        if let Some(msg) = full_msgs.get(&*name) {
                            *typ = FieldType::Message(msg.clone());
                            continue 'types;
                        } else if let Some(e) = full_enums.get(&*name) {
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
            return Ok(());
        }

        Message::write_common_uses(w, &self.messages, self, config)?;

        writeln!(
            w,
            "use quick_protobuf::{{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result}};"
        )?;

        if self.owned {
            write!(
                w,
                "use core::{{convert::{{TryFrom, TryInto}}, ops::{{Deref, DerefMut}}}};"
            )?;
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
            m.write(w, &self, config)?;
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
                if line.find(MAGIC_HEADER).is_none() {
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
