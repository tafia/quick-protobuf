use std::io::{Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf, Component};
use std::fs::File;

use errors::{Result, Error, ErrorKind};
use parser::file_descriptor;
use keywords::sanitize_keyword;

fn sizeof_varint(v: u32) -> usize {
    match v {
        0x0...0x7F => 1,
        0x80...0x3FFF => 2,
        0x4000...0x1FFFFF => 3,
        0x200000...0xFFFFFFF => 4,
        _ => 5,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Syntax {
    Proto2,
    Proto3,
}

impl Default for Syntax {
    fn default() -> Syntax {
        Syntax::Proto2
    }
}

#[derive(Debug, Clone)]
pub enum Frequency {
    Optional,
    Repeated,
    Required,
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
    Enum(String),
    Fixed64,
    Sfixed64,
    Double,
    String_,
    Bytes,
    Message(String),
    Fixed32,
    Sfixed32,
    Float,
    Map(Box<(FieldType, FieldType)>),
}

impl FieldType {

    fn has_cow(&self) -> bool {
        match *self {
            FieldType::Bytes | FieldType::String_ => true,
            FieldType::Map(ref m) => {
                let &(ref k, ref v) = &**m;
                k.has_cow() || v.has_cow()
            }
            _ => false,
        }
    }

    fn is_map(&self) -> bool {
        match *self {
            FieldType::Map(_) => true,
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
        match *self {
            FieldType::Int32 | FieldType::Sint32 | FieldType::Int64 |
            FieldType::Sint64 | FieldType::Uint32 | FieldType::Uint64 |
            FieldType::Bool | FieldType::Enum(_) => 0,
            FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double => 1,
            FieldType::String_ | FieldType::Bytes | 
                FieldType::Message(_) | FieldType::Map(_) => 2,
            FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => 5,
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
            FieldType::Bytes => "bytes",
            FieldType::Message(_) => "message",
            FieldType::Map(_) => "map",
        }
    }

    fn is_fixed_size(&self) -> bool {
        match self.wire_type_num_non_packed() {
            1 | 5 => true,
            _ => false,
        }
    }

    fn regular_default<'a, 'b>(&'a self, msgs: &'b [Message], enums: &'b [Enumerator]) -> Option<&'b str> {
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
            FieldType::String_ => Some("Cow::Borrowed(\"\")"),
            FieldType::Bytes => Some("Cow::Borrowed(b\"\")"),
            FieldType::Enum(_) => self
                .find_enum(msgs, enums)
                .and_then(|e| e.fields.iter().find(|&&(_, i)| i == 0))
                .map(|ref e| &*e.0),
            FieldType::Message(_) => None,
            FieldType::Map(_) => None,
        }
    }

    /// Searches for message corresponding to the current type
    ///
    /// Searches first basic name then within nested messages
    fn find_message<'a, 'b>(&'a self, msgs: &'b [Message]) -> Option<&'b Message> {
        match *self {
            FieldType::Message(ref m) => {
                let mut found = match m.rfind('.') {
                    Some(p) => {
                        let package = &m[..p];
                        let name = &m[(p + 1)..];
                        msgs.iter().find(|m| m.package == package && m.name == name)
                    },
                    None => msgs.iter().find(|m2| m2.name == &m[..]),
                };

                if found.is_none() {
                    // recursively search into nested messages
                    for m in msgs {
                        found = self.find_message(&m.messages);
                        if found.is_some() { break; }
                    }
                }

                found
            },
            _ => None,
        }
    }

    /// Searches for enum corresponding to the current type
    fn find_enum<'a, 'b>(&'a self, msgs: &'b [Message], enums: &'b [Enumerator]) -> Option<&'b Enumerator> {
        match *self {
            FieldType::Enum(ref m) => {
                let mut found = match m.rfind('.') {
                    Some(p) => {
                        let package = &m[..p];
                        let name = &m[(p + 1)..];
                        enums.iter().find(|m| m.package == package && m.name == name)
                    },
                    None => enums.iter().find(|m2| m2.name == &m[..]),
                };

                if found.is_none() {
                    // recursively search into nested messages
                    for m in msgs {
                        found = self.find_enum(&m.messages, &m.enums);
                        if found.is_some() { break; }
                    }
                }

                found
            }
            _ => None,
        }
    }

    fn has_lifetime(&self, msgs: &[Message], packed: bool) -> bool {
        match *self {
            FieldType::String_ | FieldType::Bytes => true, // Cow<[u8]>
            FieldType::Message(_) => self.find_message(msgs).map_or(false, |m| m.has_lifetime(msgs)),
            FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double |
            FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => packed, // Cow<[M]>
            FieldType::Map(ref m) => {
                let &(ref key, ref value) = &**m;
                key.has_lifetime(msgs, false) || value.has_lifetime(msgs, false)
            }
            _ => false,
        }
    }

    fn rust_type(&self, msgs: &[Message], enums: &[Enumerator]) -> String {
        match *self {
            FieldType::Int32 | FieldType::Sint32 | FieldType::Sfixed32 => "i32".to_string(),
            FieldType::Int64 | FieldType::Sint64 | FieldType::Sfixed64 => "i64".to_string(),
            FieldType::Uint32 | FieldType::Fixed32 => "u32".to_string(),
            FieldType::Uint64 | FieldType::Fixed64 => "u64".to_string(),
            FieldType::Double => "f64".to_string(),
            FieldType::Float => "f32".to_string(),
            FieldType::String_ => "Cow<'a, str>".to_string(),
            FieldType::Bytes => "Cow<'a, [u8]>".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Enum(ref e) => match self.find_enum(msgs, enums) {
                Some(e) => format!("{}{}", e.get_modules(), e.name),
                None => unreachable!(format!("Could not find enum {} in {:?}", e, enums)),
            },
            FieldType::Message(ref msg) => match self.find_message(msgs) {
                Some(m) => {
                    let lifetime = if m.has_lifetime(msgs) { "<'a>" } else { "" };
                    format!("{}{}{}", m.get_modules(), m.name, lifetime)
                },
                None => unreachable!(format!("Could not find message {} in {:?}", msg, msgs)),
            },
            FieldType::Map(ref t) => {
                let &(ref key, ref value) = &**t;
                format!("HashMap<{}, {}>", key.rust_type(msgs, enums), value.rust_type(msgs, enums))
            }
        }
    }

    /// Returns the relevant function to read the data, both for regular and Cow wrapped
    fn read_fn(&self, msgs: &[Message]) -> (String, String) {
        match *self {
            FieldType::Message(ref msg) => match self.find_message(msgs) {
                Some(m) => {
                    let m = format!("r.read_message(bytes, {}{}::from_reader)", m.get_modules(), m.name);
                    (m.clone(), m)
                }
                None => unreachable!(format!("Could not find message {}", msg)),
            },
            FieldType::Map(_) => unreachable!("There should be a special case for maps"),
            FieldType::String_ | FieldType::Bytes => {
                let m = format!("r.read_{}(bytes)", self.proto_type());
                let cow = format!("{}.map(Cow::Borrowed)", m);
                (m, cow)
            },
            _ => {
                let m = format!("r.read_{}(bytes)", self.proto_type());
                (m.clone(), m)
            }
        }
    }

    fn get_size(&self, s: &str) -> String {
        match *self {
            FieldType::Int32 | FieldType::Int64 | FieldType::Uint32 | FieldType::Uint64 |
            FieldType::Bool | FieldType::Enum(_) => format!("sizeof_varint(*({}) as u64)", s),
            FieldType::Sint32 => format!("sizeof_sint32(*({}))", s),
            FieldType::Sint64 => format!("sizeof_sint64(*({}))", s),

            FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double => "8".to_string(),
            FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => "4".to_string(),

            FieldType::String_ | FieldType::Bytes => format!("sizeof_len(({}).len())", s),

            FieldType::Message(_) => format!("sizeof_len(({}).get_size())", s),
            
            FieldType::Map(ref m) => {
                let &(ref k, ref v) = &**m;
                format!("2 + {} + {}", k.get_size("k"), v.get_size("v"))
            }
        }
    }

    fn get_write(&self, s: &str, boxed: bool) -> String {
        match *self {
            FieldType::Enum(_) => format!("write_enum(*{} as i32)", s),

            FieldType::Int32 | FieldType::Sint32 | FieldType::Int64 |
            FieldType::Sint64 | FieldType::Uint32 | FieldType::Uint64 |
            FieldType::Bool |
            FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double |
            FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => {
                format!("write_{}(*{})", self.proto_type(), s)
            },

            FieldType::String_ => format!("write_string(&**{})", s),
            FieldType::Bytes => format!("write_bytes(&**{})", s),

            FieldType::Message(_) if boxed => format!("write_message(&**{})", s),
            FieldType::Message(_) => format!("write_message({})", s),
            
            FieldType::Map(ref m) => {
                let &(ref k, ref v) = &**m;
                format!("write_map({}, {}, |w| w.{}, {}, |w| w.{})",
                        self.get_size(""), 
                        tag(1, k, false), k.get_write("k", false),
                        tag(2, v, false), v.get_write("v", false))
            }
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

    /// searches if the message must be boxed
    fn is_leaf(&self, leaf_messages: &[String]) -> bool {
        match self.typ {
            FieldType::Message(ref s) => {
                match self.frequency {
                    Frequency::Repeated => true,
                    _ => {
                        let typ = match s.rfind('.') {
                            Some(p) => &s[p + 1..],
                            None => &s[..],
                        };
                        leaf_messages.iter().any(|m| &*m == &typ)
                    }
                }
            }
            _ => true,
        }
    }

    fn sanitize_default(&mut self, msgs: &[Message], enums: &[Enumerator]) {
        if let Some(ref mut d) = self.default {
            *d = match &*self.typ.rust_type(msgs, enums) {
                "u32" => format!("{}u32", *d),
                "u64" => format!("{}u64", *d),
                "i32" => format!("{}i32", *d),
                "i64" => format!("{}i64", *d),
                "f32" => match &*d.to_lowercase() {
                    "inf" => "::std::f32::INFINITY".to_string(),
                    "-inf" => "::std::f32::NEG_INFINITY".to_string(),
                    "nan" => "::std::f32::NAN".to_string(),
                    _ => format!("{}f32", *d),
                },
                "f64" => match &*d.to_lowercase() {
                    "inf" => "::std::f64::INFINITY".to_string(),
                    "-inf" => "::std::f64::NEG_INFINITY".to_string(),
                    "nan" => "::std::f64::NAN".to_string(),
                    _ => format!("{}f64", *d),
                },
                "Cow<'a, str>" => format!("Cow::Borrowed({})", d),
                "Cow<'a, [u8]>" => format!("Cow::Borrowed(b{})", d),
                "bool" => format!("{}", d.parse::<bool>().unwrap()),
                e => format!("{}::{}", e, d), // enum, as message and map do not have defaults
            }
        }
    }

    fn has_regular_default(&self, msgs: &[Message], enums: &[Enumerator]) -> bool {
        self.default.is_none() 
            || self.default.as_ref().map(|d| &**d) == self.typ.regular_default(msgs, enums)
    }

    fn tag(&self) -> u32 {
        tag(self.number as u32, &self.typ, self.packed())
    }

    fn write_definition<W: Write>(&self, w: &mut W, msgs: &[Message], enums: &[Enumerator]) -> Result<()> {
        write!(w, "    pub {}: ", self.name)?;
        let rust_type = self.typ.rust_type(msgs, enums);
        match self.frequency {
            _ if self.boxed => writeln!(w, "Option<Box<{}>>,", rust_type)?,
            Frequency::Optional if self.default.is_some() => writeln!(w, "{},", rust_type)?,
            Frequency::Optional => writeln!(w, "Option<{}>,", rust_type)?,
            Frequency::Repeated if self.packed() && self.typ.is_fixed_size() => {
                writeln!(w, "Cow<'a, [{}]>,", rust_type)?;
            },
            Frequency::Repeated => writeln!(w, "Vec<{}>,", rust_type)?,
            Frequency::Required => writeln!(w, "{},", rust_type)?,
        }
        Ok(())
    }

    fn write_match_tag<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {

        // special case for FieldType::Map: destructure tuple before inserting in HashMap
        if let FieldType::Map(ref m) = self.typ {
            let &(ref key, ref value) = &**m;

            writeln!(w, "                Ok({}) => {{", self.tag())?;
            writeln!(w, "                    let (key, value) = r.read_map(bytes, |r, bytes| {}, |r, bytes| {})?;", 
                     key.read_fn(msgs).1, value.read_fn(msgs).1)?;
            writeln!(w, "                    msg.{}.insert(key, value);", self.name)?;
            writeln!(w, "                }}")?;
            return Ok(());
        }

        let (val, val_cow) = self.typ.read_fn(msgs);
        let name = &self.name;
        write!(w, "                Ok({}) => ", self.tag())?;
        match self.frequency {
            _ if self.boxed => writeln!(w, "msg.{} = Some(Box::new({}?)),", name, val)?,
            Frequency::Required => writeln!(w, "msg.{} = {}?,", name, val_cow)?,
            Frequency::Optional if self.default.is_some() => writeln!(w, "msg.{} = {}?,", name, val_cow)?,
            Frequency::Optional => writeln!(w, "msg.{} = Some({}?),", name, val_cow)?,
            Frequency::Repeated if self.packed() && self.typ.is_fixed_size() => {
                writeln!(w, "msg.{} = Cow::Borrowed(r.read_packed_fixed(bytes)?),", name)?;
            },
            Frequency::Repeated if self.packed() => {
                writeln!(w, "msg.{} = r.read_packed(bytes, |r, bytes| {})?,", name, val_cow)?;
            },
            Frequency::Repeated => writeln!(w, "msg.{}.push({}?),", name, val_cow)?,
        }
        Ok(())
    }

    fn write_get_size<W: Write>(&self, w: &mut W) -> Result<()> {
        write!(w, "        + ")?;
        let tag_size = sizeof_varint(self.tag());
        match self.frequency {
            Frequency::Required if self.typ.is_map() => {
                writeln!(w, "self.{}.iter().map(|(k, v)| {} + sizeof_len({})).sum::<usize>()", 
                         self.name, tag_size, self.typ.get_size(""))?;
            }
            Frequency::Required => writeln!(w, "{} + {}", tag_size, self.typ.get_size(&format!("&self.{}", self.name)))?,
            Frequency::Optional => {
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
                        writeln!(w, "if self.{} == {} {{ 0 }} else {{ {} + {} }}", 
                               self.name, d, tag_size, self.typ.get_size(&format!("&self.{}", self.name)))?;
                    }
                }
            }
            Frequency::Repeated => {
                if self.packed() {
                    write!(w, "if self.{}.is_empty() {{ 0 }} else {{ {} + ", self.name, tag_size)?;
                    match self.typ.wire_type_num_non_packed() {
                        1 => writeln!(w, "sizeof_len(self.{}.len() * 8) }}", self.name)?,
                        5 => writeln!(w, "sizeof_len(self.{}.len() * 4) }}", self.name)?,
                        _ => writeln!(w, "sizeof_len(self.{}.iter().map(|s| {}).sum::<usize>()) }}", 
                                      self.name, self.typ.get_size("s"))?,
                    }
                } else {
                    match self.typ.wire_type_num_non_packed() {
                        1 => writeln!(w, "({} + 8) * self.{}.len()", tag_size, self.name)?,
                        5 => writeln!(w, "({} + 4) * self.{}.len()", tag_size, self.name)?,
                        _ => writeln!(w, "self.{}.iter().map(|s| {} + {}).sum::<usize>()", 
                                      self.name, tag_size, self.typ.get_size("s"))?,
                    }
                }
            }
        }
        Ok(())
    }

    fn write_write<W: Write>(&self, w: &mut W) -> Result<()> {
        match self.frequency {
            Frequency::Required if self.typ.is_map() => {
                writeln!(w, "        for (k, v) in self.{}.iter() {{ w.write_with_tag({}, |w| w.{})?; }}",
                         self.name, self.tag(), self.typ.get_write("", false))?;
            }
            Frequency::Required => {
                writeln!(w, "        w.write_with_tag({}, |w| w.{})?;", 
                         self.tag(), self.typ.get_write(&format!("&self.{}", self.name), self.boxed))?;
            }
            Frequency::Optional => match self.default.as_ref() {
                None => {
                    writeln!(w, "        if let Some(ref s) = self.{} {{ w.write_with_tag({}, |w| w.{})?; }}",
                             self.name, self.tag(), self.typ.get_write("s", self.boxed))?;
                },
                Some(d) => {
                    writeln!(w, "        if self.{} != {} {{ w.write_with_tag({}, |w| w.{})?; }}",
                             self.name, d, self.tag(), self.typ.get_write(&format!("&self.{}", self.name), self.boxed))?;
                }
            },
            Frequency::Repeated if self.packed() && self.typ.is_fixed_size() => {
                writeln!(w, "        w.write_packed_fixed_with_tag({}, &self.{})?;", self.tag(), self.name)?
            }
            Frequency::Repeated if self.packed() => {
                writeln!(w, "        w.write_packed_with_tag({}, &self.{}, |w, m| w.{}, &|m| {})?;",
                         self.tag(), self.name, self.typ.get_write("m", self.boxed), self.typ.get_size("m"))?
            }
            Frequency::Repeated => {
                writeln!(w, "        for s in &self.{} {{ w.write_with_tag({}, |w| w.{})?; }}", 
                         self.name, self.tag(), self.typ.get_write("s", self.boxed))?;
            }
        }
        Ok(())
    }
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
}

impl Message {

    fn is_leaf(&self, leaf_messages: &[String]) -> bool {
        self.imported || 
            self.fields.iter()
            .chain(self.oneofs.iter().flat_map(|o| o.fields.iter()))
            .all(|f| f.is_leaf(leaf_messages) || f.deprecated)
    }

    fn has_lifetime(&self, msgs: &[Message]) -> bool {
        self.fields.iter()
            .chain(self.oneofs.iter().flat_map(|o| o.fields.iter()))
            .any(|f| match f.typ {
                FieldType::Message(ref m) if &m[..] == self.name => false,
                ref t => t.has_lifetime(msgs, f.packed()),
            })
    }

    fn get_modules(&self) -> String {
        self.package
            .split('.').filter(|p| !p.is_empty())
            .map(|p| format!("mod_{}::", p))
            .collect()
    }

    fn is_unit(&self) -> bool {
        self.fields.is_empty() && self.oneofs.is_empty()
    }

    fn write<W: Write>(&self, w: &mut W, msgs: &[Message], enums: &[Enumerator]) -> Result<()> {
        println!("Writing message {}{}", self.get_modules(), self.name);
        writeln!(w, "")?;

        self.write_definition(w, msgs, enums)?;
        writeln!(w, "")?;
        self.write_impl_message_read(w, msgs, enums)?;
        writeln!(w, "")?;
        self.write_impl_message_write(w, msgs)?;

        if !(self.messages.is_empty() && self.enums.is_empty() && self.oneofs.is_empty()) {
            writeln!(w, "")?;
            writeln!(w, "pub mod mod_{} {{", self.name)?;
            writeln!(w, "")?;
            if self.messages.iter().any(|m| m.fields.iter()
                                        .chain(m.oneofs.iter().flat_map(|o| o.fields.iter()))
                                        .any(|f| f.typ.has_cow())) {
                writeln!(w, "use std::borrow::Cow;")?;
            }
            if self.messages.iter().any(|m| m.fields.iter()
                                        .chain(m.oneofs.iter().flat_map(|o| o.fields.iter()))
                                        .any(|f| f.typ.is_map())) {
                writeln!(w, "use std::collections::HashMap;")?;
            }
            if !self.messages.is_empty() || !self.oneofs.is_empty() {
                writeln!(w, "use super::*;")?;
            }
            for m in &self.messages {
                m.write(w, msgs, enums)?;
            }
            for e in &self.enums {
                e.write(w)?;
            }
            for o in &self.oneofs {
                o.write(w, msgs, enums)?;
            }

            writeln!(w, "")?;
            writeln!(w, "}}")?;
        }

        Ok(())
    }

    fn write_definition<W: Write>(&self, w: &mut W, msgs: &[Message], enums: &[Enumerator]) -> Result<()> {
        writeln!(w, "#[derive(Debug, Default, PartialEq, Clone)]")?;
        if self.is_unit() {
            writeln!(w, "pub struct {} {{ }}", self.name)?;
            return Ok(());
        }

        if self.has_lifetime(msgs) {
            writeln!(w, "pub struct {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "pub struct {} {{", self.name)?;
        }
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            f.write_definition(w, msgs, enums)?;
        }
        for o in &self.oneofs {
            o.write_message_definition(w, msgs)?;
        }
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_impl_message_read<W: Write>(&self, w: &mut W, msgs: &[Message], enums: &[Enumerator]) -> Result<()> {
        if self.is_unit() {
            writeln!(w, "impl {} {{", self.name)?;
            writeln!(w, "    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {{")?;
            writeln!(w, "        r.read_to_end();")?;
            writeln!(w, "        Ok(Self::default())")?;
            writeln!(w, "    }}")?;
            writeln!(w, "}}")?;
            return Ok(());
        }

        if self.has_lifetime(msgs) {
            writeln!(w, "impl<'a> {}<'a> {{", self.name)?;
            writeln!(w, "    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {{")?;
        } else {
            writeln!(w, "impl {} {{", self.name)?;
            writeln!(w, "    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {{")?;
        }

        let unregular_defaults = self.fields.iter()
            .filter(|f| !f.has_regular_default(msgs, enums)).collect::<Vec<_>>();
        if unregular_defaults.is_empty() {
            writeln!(w, "        let mut msg = Self::default();")?;
        } else {
            writeln!(w, "        let mut msg = {} {{", self.name)?;
            for f in unregular_defaults {
                writeln!(w, "            {}: {},", f.name, f.default.as_ref().unwrap())?;
            }
            writeln!(w, "            ..Self::default()")?;
            writeln!(w, "        }};")?;
        }
        writeln!(w, "        while !r.is_eof() {{")?;
        writeln!(w, "            match r.next_tag(bytes) {{")?;
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            f.write_match_tag(w, msgs)?;
        }
        for o in &self.oneofs {
            o.write_match_tag(w, msgs)?;
        }
        writeln!(w, "                Ok(t) => {{ r.read_unknown(bytes, t)?; }}")?;
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

    fn write_impl_message_write<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        if self.is_unit() {
            writeln!(w, "impl MessageWrite for {} {{ }}", self.name)?;
            return Ok(());
        }

        if self.has_lifetime(msgs) {
            writeln!(w, "impl<'a> MessageWrite for {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "impl MessageWrite for {} {{", self.name)?;
        }
        self.write_get_size(w)?;
        writeln!(w, "")?;
        self.write_write_message(w)?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_get_size<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "    fn get_size(&self) -> usize {{")?;
        writeln!(w, "        0")?;
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            f.write_get_size(w)?;
        }
        for o in self.oneofs.iter() {
            o.write_get_size(w)?;
        }
        writeln!(w, "    }}")?;
        Ok(())
    }

    fn write_write_message<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {{")?;
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            f.write_write(w)?;
        }
        for o in &self.oneofs {
            o.write_write(w)?;
        }
        writeln!(w, "        Ok(())")?;
        writeln!(w, "    }}")?;
        Ok(())
    }

    fn sanity_checks(&self) -> Result<()> {
        // checks for reserved fields
        for f in self.fields.iter()
            .chain(self.oneofs.iter().flat_map(|o| o.fields.iter()))
        {
            if self.reserved_names.as_ref().map_or(false, |names| names.contains(&f.name)) || 
                self.reserved_nums.as_ref().map_or(false, |nums| nums.contains(&f.number)) {
                return Err(ErrorKind::InvalidMessage(
                        format!("Error in message {}\nField {:?} conflict with reserved fields",
                                self.name, f)).into());
            }
        }
        Ok(())
    }

    fn set_package(&mut self, package: &str) {
        // set package = current_package.package.name to nested messages
        if package.is_empty() {
            for m in &mut self.messages { m.set_package(&self.name); }
            for m in &mut self.enums { m.set_package(&self.name); }
            for m in &mut self.oneofs { m.set_package(&self.name); }
        } else {
            self.package = package.to_string();
            let child_package = format!("{}.{}", package, self.name);
            for m in &mut self.messages { m.set_package(&child_package); }
            for m in &mut self.enums { m.set_package(&child_package); }
            for m in &mut self.oneofs { m.set_package(&child_package); }
        }
    }

    /// Searches for a matching message in all message
    ///
    /// If none is found, 
    fn set_enums(&mut self, msgs: &[Message]) {
        for f in self.fields.iter_mut()
            .chain(self.oneofs.iter_mut().flat_map(|o| o.fields.iter_mut()))
        {
            if f.typ.find_message(&msgs).is_none() {
                if let FieldType::Message(m) = f.typ.clone() {
                    f.typ = FieldType::Enum(m);
                    f.boxed = false;
                }
            }
        }
        for m in &mut self.messages {
            m.set_enums(msgs);
        }
    }

    fn set_map_required(&mut self) {
        for f in self.fields.iter_mut()
            .chain(self.oneofs.iter_mut().flat_map(|o| o.fields.iter_mut()))
        {
            if let FieldType::Map(_) = f.typ {
                f.frequency = Frequency::Required;
            }
        }
        for m in &mut self.messages {
            m.set_map_required();
        }
    }

    fn set_repeated_as_packed(&mut self) {
        for f in self.fields.iter_mut()
            .chain(self.oneofs.iter_mut().flat_map(|o| o.fields.iter_mut()))
        {
            if f.packed.is_none() { 
                if let Frequency::Repeated = f.frequency { 
                    f.packed = Some(true); 
                }
            }
        }
    }

    fn sanitize_defaults(&mut self, msgs: &[Message], enums: &[Enumerator]) {
        for f in self.fields.iter_mut()
            .chain(self.oneofs.iter_mut().flat_map(|o| o.fields.iter_mut())) {
            f.sanitize_default(msgs, enums);
        }
        for m in &mut self.messages {
            m.sanitize_defaults(msgs, enums);
        }
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
}

#[derive(Debug, Clone)]
pub struct Enumerator {
    pub name: String,
    pub fields: Vec<(String, i32)>,
    pub imported: bool,
    pub package: String,
}

impl Enumerator {

    fn set_package(&mut self, package: &str) {
        self.package = package.to_string();
    }

    fn sanitize_names(&mut self) {
        sanitize_keyword(&mut self.name);
        sanitize_keyword(&mut self.package);
        for f in self.fields.iter_mut() {
            sanitize_keyword(&mut f.0);
        }
    }

    fn get_modules(&self) -> String {
        self.package
            .split('.').filter(|p| !p.is_empty())
            .map(|p| format!("mod_{}::", p))
            .collect()
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<()> {
        println!("Writing enum {}", self.name);
        writeln!(w, "")?;
        self.write_definition(w)?;
        writeln!(w, "")?;
        if self.fields.is_empty() {
            Ok(())
        } else {
            self.write_impl_default(w)?;
            writeln!(w, "")?;
            self.write_from_i32(w)
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
        writeln!(w, "        {}::{}", self.name, self.fields[0].0)?;
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
}

#[derive(Debug, Clone, Default)]
pub struct OneOf {
    pub name: String,
    pub fields: Vec<Field>,
    pub package: String,
}

impl OneOf {

    fn has_lifetime(&self, msgs: &[Message]) -> bool {
        self.fields.iter().any(|f| !f.deprecated && f.typ.has_lifetime(msgs, f.packed()))
    }

    fn set_package(&mut self, package: &str) {
        self.package = package.to_string();
    }

    fn sanitize_names(&mut self) {
        sanitize_keyword(&mut self.name);
        sanitize_keyword(&mut self.package);
        for f in self.fields.iter_mut() {
            sanitize_keyword(&mut f.name);
        }
    }

    fn get_modules(&self) -> String {
        self.package
            .split('.').filter(|p| !p.is_empty())
            .map(|p| format!("mod_{}::", p))
            .collect()
    }

    fn write<W: Write>(&self, w: &mut W, msgs: &[Message], enums: &[Enumerator]) -> Result<()> {
        writeln!(w, "")?;
        self.write_definition(w, msgs, enums)?;
        writeln!(w, "")?;
        self.write_impl_default(w, msgs)?;
        Ok(())
    }

    fn write_definition<W: Write>(&self, w: &mut W, msgs: &[Message], enums: &[Enumerator]) -> Result<()> {
        writeln!(w, "#[derive(Debug, PartialEq, Clone)]")?;
        if self.has_lifetime(msgs) {
            writeln!(w, "pub enum OneOf{}<'a> {{", self.name)?;
        } else {
            writeln!(w, "pub enum OneOf{} {{", self.name)?;
        }
        for f in &self.fields {
            writeln!(w, "    {}({}),", f.name, f.typ.rust_type(msgs, enums))?;
        }
        writeln!(w, "    None,")?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_impl_default<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        if self.has_lifetime(msgs) {
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

    fn write_message_definition<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        if self.has_lifetime(msgs) {
            writeln!(w, "    pub {}: {}OneOf{}<'a>,", self.name, self.get_modules(), self.name)?;
        } else {
            writeln!(w, "    pub {}: {}OneOf{},", self.name, self.get_modules(), self.name)?;
        }
        Ok(())
    }

    fn write_match_tag<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            let (val, val_cow) = f.typ.read_fn(msgs);
            if f.boxed {
                writeln!(w, "                Ok({}) => msg.{} = {}OneOf{}::{}(Box::new({}?)),", 
                       f.tag(), self.name, self.get_modules(), self.name, f.name, val)?;
            } else {
                writeln!(w, "                Ok({}) => msg.{} = {}OneOf{}::{}({}?),", 
                       f.tag(), self.name, self.get_modules(), self.name, f.name, val_cow)?;
            }
        }
        Ok(())
    }

    fn write_get_size<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "        + match self.{} {{", self.name)?;
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            let tag_size = sizeof_varint(f.tag());
            if f.typ.is_fixed_size() {
                writeln!(w, "            {}OneOf{}::{}(_) => {} + {},", 
                         self.get_modules(), self.name, f.name, tag_size, f.typ.get_size(""))?;
            } else {
                writeln!(w, "            {}OneOf{}::{}(ref m) => {} + {},", 
                         self.get_modules(), self.name, f.name, tag_size, f.typ.get_size("m"))?;
            }
        }
        writeln!(w, "            {}OneOf{}::None => 0,", self.get_modules(), self.name)?;
        write!(w, "    }}")?;
        Ok(())
    }

    fn write_write<W: Write>(&self, w: &mut W) -> Result<()> {
        write!(w, "        match self.{} {{", self.name)?;
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            writeln!(w, "            {}OneOf{}::{}(ref m) => {{ w.write_with_tag({}, |w| w.{})? }},", 
                     self.get_modules(), self.name, f.name, f.tag(), f.typ.get_write("m", f.boxed))?;
        }
        writeln!(w, "            {}OneOf{}::None => {{}},", self.get_modules(), self.name)?;
        write!(w, "    }}")?;
        Ok(())
    }

}

pub struct Config<P: AsRef<Path>> {
    pub in_file: P,
    pub out_file: P,
    pub single_module: bool,
}

#[derive(Debug, Default)]
pub struct FileDescriptor {
    pub import_paths: Vec<PathBuf>,
    pub package: String,
    pub syntax: Syntax,
    pub messages: Vec<Message>,
    pub enums: Vec<Enumerator>,
}

impl FileDescriptor {

    pub fn write_proto<P: AsRef<Path>>(config: &Config<P>) -> Result<()> {
        let mut desc = FileDescriptor::read_proto(&config.in_file)?;
        desc.fetch_imports(config.in_file.as_ref())?;

        let mut leaf_messages = Vec::new();
        break_cycles(&mut desc.messages, &mut leaf_messages);

        desc.sanity_checks(config.in_file.as_ref())?;
        desc.set_enums();
        desc.set_defaults();
        desc.sanitize_names();

        if config.single_module {
            desc.package = "".to_string();
        }

        let name = config.in_file.as_ref().file_name().and_then(|e| e.to_str()).unwrap();
        let out_file = {
            let mut file_stem: String = config.out_file.as_ref().file_stem()
                .and_then(|f| f.to_str())
                .map(|s| s.to_string())
                .ok_or_else::<Error, _>(|| ErrorKind::OutputFile(config.out_file.as_ref().to_owned()).into())?;

            file_stem = file_stem.chars().map(|c| match c {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => c,
                _ => '_',
            }).collect();
            sanitize_keyword(&mut file_stem);
            config.out_file.as_ref().with_file_name(format!("{}.rs", file_stem))
        };
        let mut w = BufWriter::new(File::create(out_file)?);
        desc.write(&mut w, name)
    }

    /// Opens a proto file, reads it and returns raw parsed data
    fn read_proto<P: AsRef<Path>>(in_file: P) -> Result<FileDescriptor> {
        let mut buf = Vec::new();
        {
            let f = File::open(&in_file)?;
            let mut reader = BufReader::new(f);
            reader.read_to_end(&mut buf)?;
        }
        let desc = file_descriptor(&buf).to_result()?;
        Ok(desc)
    }

    fn sanity_checks(&self, file: &Path) -> Result<()> {
        for i in &self.import_paths {
            // search if the corresponding file exists
            if i.is_file() {
                if !get_imported_path(file, i).exists() {
                    return Err(ErrorKind::InvalidImport(
                            format!("File {} not found", i.display())).into());
                }
            }
        }
        for m in &self.messages {
            m.sanity_checks()?;
        }
        Ok(())
    }

    /// Get messages and enums from imports
    fn fetch_imports(&mut self, in_file: &Path) -> Result<()> {
        for m in &mut self.messages {
            m.set_package("");
        }
        for m in &mut self.enums {
            m.set_package("");
        }
        for p in &self.import_paths {
            let import_path = get_imported_path(&in_file, p);
            let mut f = FileDescriptor::read_proto(&import_path)?;
            f.fetch_imports(&import_path)?;

            // if the proto has a packge then the names will be prefixed
            let package = f.package.clone();
            self.messages.extend(f.messages.drain(..).map(|mut m| {
                m.set_package(&package);
                m.imported = true;
                m
            }));
            self.enums.extend(f.enums.drain(..).map(|mut e| {
                e.set_package(&package);
                e.imported = true;
                e
            }));
        }
        Ok(())
    }

    fn set_defaults(&mut self) {
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
        let msgs = self.messages.clone();
        for m in &mut self.messages {
            m.sanitize_defaults(&msgs, &self.enums);
        }
    }

    fn sanitize_names(&mut self) {
        for m in &mut self.messages {
            m.sanitize_names();
        }
        for e in &mut self.enums {
            e.sanitize_names();
        }
    }

    fn set_enums(&mut self) {
        // this is very inefficient but we don't care ...
        let msgs = self.messages.clone();

        for m in &mut self.messages {
            m.set_enums(&msgs);
        }
    }

    fn write<W: Write>(&self, w: &mut W, filename: &str) -> Result<()> {
        println!("Found {} messages, and {} enums", self.messages.len(), self.enums.len());
        self.write_headers(w, filename)?;
        self.write_package_start(w)?;
        self.write_uses(w)?;
        self.write_imports(w)?;
        self.write_enums(w)?;
        self.write_messages(w)?;
        self.write_package_end(w)?;
        println!("Done processing {}", filename);
        Ok(())
    }

    fn write_headers<W: Write>(&self, w: &mut W, filename: &str) -> Result<()> {
        writeln!(w, "//! Automatically generated rust module for '{}' file", filename)?;
        writeln!(w, "")?;
        writeln!(w, "#![allow(non_snake_case)]")?;
        writeln!(w, "#![allow(non_upper_case_globals)]")?;
        writeln!(w, "#![allow(non_camel_case_types)]")?;
        writeln!(w, "")?;
        Ok(())
    }

    fn write_uses<W: Write>(&self, w: &mut W) -> Result<()> {
        if self.messages.is_empty() {
            return Ok(());
        }
        if self.messages.iter().all(|m| m.is_unit()) {
            writeln!(w, "use quick_protobuf::{{BytesReader, Result, MessageWrite}};")?;
            return Ok(());
        }
        writeln!(w, "use std::io::Write;")?;
        if self.messages.iter().any(|m| m.fields.iter()
                                    .chain(m.oneofs.iter().flat_map(|o| o.fields.iter()))
                                    .any(|f| f.typ.has_cow())) {
            writeln!(w, "use std::borrow::Cow;")?;
        }
        if self.messages.iter().any(|m| m.fields.iter()
                                    .chain(m.oneofs.iter().flat_map(|o| o.fields.iter()))
                                    .any(|f| f.typ.is_map())) {
            writeln!(w, "use std::collections::HashMap;")?;
        }
        writeln!(w, "use quick_protobuf::{{MessageWrite, BytesReader, Writer, Result}};")?;
        writeln!(w, "use quick_protobuf::sizeofs::*;")?;
        Ok(())
    }

    fn write_imports<W: Write>(&self, w: &mut W) -> Result<()> {
        if self.import_paths.is_empty() {
            return Ok(());
        }
        writeln!(w, "")?;
        for i in &self.import_paths {
            write!(w, "use super::")?;
            // exit from current package encapsulation
            for _ in self.package.split('.').filter(|p| !p.is_empty()) {
                write!(w, "super::")?;
            }
            for c in i.components() {
                match c {
                    Component::RootDir | Component::Prefix(_) => return Err(ErrorKind::InvalidImport(
                            "Cannot import from absolute path".to_string()).into()),
                    Component::CurDir => continue,
                    Component::ParentDir => { write!(w, "super::")?; },
                    Component::Normal(path) => {
                        if path.to_str().map_or(false, |s| s.contains('.')) {
                            let mut file_stem = Path::new(path).file_stem().unwrap()
                                .to_string_lossy().to_string();
                            sanitize_keyword(&mut file_stem);
                            writeln!(w, "{}::*;", file_stem)?;
                        } else {
                            let mut file_stem = path.to_string_lossy().to_string();
                            sanitize_keyword(&mut file_stem);
                            write!(w, "{}::", file_stem)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn write_package_start<W: Write>(&self, w: &mut W) -> Result<()> {
        if self.package.is_empty() { return Ok(()); }
        for p in self.package.split('.') { writeln!(w, "pub mod mod_{} {{", p)?; }
        writeln!(w, "")?;
        Ok(())
    }

    fn write_package_end<W: Write>(&self, w: &mut W) -> Result<()> {
        if self.package.is_empty() { return Ok(()); }
        writeln!(w, "")?;
        for _ in self.package.split('.') { writeln!(w, "}}")?; }
        Ok(())
    }

    fn write_enums<W: Write>(&self, w: &mut W) -> Result<()> {
        for m in self.enums.iter().filter(|e| !e.imported) {
            println!("Writing enum {}", m.name);
            writeln!(w, "")?;
            m.write_definition(w)?;
            writeln!(w, "")?;
            m.write_impl_default(w)?;
            writeln!(w, "")?;
            m.write_from_i32(w)?;
        }
        Ok(())
    }

    fn write_messages<W: Write>(&self, w: &mut W) -> Result<()> {
        for m in self.messages.iter().filter(|m| !m.imported) {
            m.write(w, &self.messages, &self.enums)?;
        }
        Ok(())
    }
}

fn get_imported_path<P: AsRef<Path>, Q: AsRef<Path>>(in_file: P, import: Q) -> PathBuf {
    in_file.as_ref().parent().map_or_else(|| import.as_ref().into(), |p| p.join(import.as_ref()))
}

/// Breaks cycles by adding boxes when necessary
///
/// Cycles means one Message calls itself at some point
fn break_cycles(messages: &mut [Message], leaf_messages: &mut Vec<String>) {
    
    for m in messages.iter_mut() {
        break_cycles(&mut m.messages, leaf_messages);
    }

    let message_names = messages.iter().map(|m| m.name.to_string()).collect::<Vec<_>>();

    let mut undef_messages = (0..messages.len()).collect::<Vec<_>>();
    while !undef_messages.is_empty() {
        let len = undef_messages.len();
        let mut new_undefs = Vec::new();
        for i in undef_messages {
            if messages[i].is_leaf(&leaf_messages) {
                leaf_messages.push(message_names[i].clone())
            } else {
                new_undefs.push(i);
            }
        }
        undef_messages = new_undefs;
        if len == undef_messages.len() {
            // try boxing messages, one by one ...
            let k = undef_messages.pop().unwrap();
            {
                let mut m = messages[k].clone();
                for f in m.fields.iter_mut()
                    .chain(m.oneofs.iter_mut().flat_map(|o| o.fields.iter_mut()))
                {
                    if !f.is_leaf(&leaf_messages) {
                        f.frequency = Frequency::Optional;
                        f.boxed = true;
                    }
                }
                messages[k] = m;
            }
        }
    }
}

/// Calculates the tag value
fn tag(number: u32, typ: &FieldType, packed: bool) -> u32 {
    number << 3 | typ.wire_type_num(packed)
}
