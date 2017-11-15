use std::io::{Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
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
            FieldType::String_ => Some("Cow::Borrowed(\"\")"),
            FieldType::Bytes => Some("Cow::Borrowed(b\"\")"),
            FieldType::Enum(_) => self
                .find_enum(&desc.messages, &desc.enums)
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
                    for m in msgs.iter() {
                        found = self.find_enum(&m.messages,&m.enums);
                        if found.is_some() { break; }
                    }
                }

                found
            }
            _ => None,
        }
    }

    fn has_lifetime(&self, desc: &FileDescriptor, packed: bool) -> bool {
        match *self {
            FieldType::String_ | FieldType::Bytes => true, // Cow<[u8]>
            FieldType::Message(_) => self.find_message(&desc.messages).map_or(false, |m| m.has_lifetime(desc)),
            FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double |
            FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => packed, // Cow<[M]>
            FieldType::Map(ref m) => {
                let &(ref key, ref value) = &**m;
                key.has_lifetime(desc, false) || value.has_lifetime(desc, false)
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
            FieldType::String_ => "Cow<'a, str>".to_string(),
            FieldType::Bytes => "Cow<'a, [u8]>".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Enum(ref e) => match self.find_enum(&desc.messages, &desc.enums) {
                Some(e) => format!("{}{}", e.get_modules(desc), e.name),
                None => bail!("Could not find enum {} in {:?}", e, desc.enums)
            },
            FieldType::Message(ref msg) => match self.find_message(&desc.messages) {
                Some(m) => {
                    let lifetime = if m.has_lifetime(desc) { "<'a>" } else { "" };
                    format!("{}{}{}", m.get_modules(desc), m.name, lifetime)
                },
                None => bail!(format!("Could not find message {} in {:?}", msg, desc.messages))
            },
            FieldType::Map(ref t) => {
                let &(ref key, ref value) = &**t;
                format!("HashMap<{}, {}>", key.rust_type(desc)?, value.rust_type(desc)?)
            }
        })
    }

    /// Returns the relevant function to read the data, both for regular and Cow wrapped
    fn read_fn(&self, desc: &FileDescriptor) -> Result<(String, String)> {
        Ok(match *self {
            FieldType::Message(ref msg) => match self.find_message(&desc.messages) {
                Some(m) => {
                    let m = format!("r.read_message::<{}{}>(bytes)", m.get_modules(desc), m.name);
                    (m.clone(), m)
                }
                None => bail!(format!("Could not find message {}", msg))
            },
            FieldType::Map(_) =>  bail!("There should be a special case for maps"),
            FieldType::String_ | FieldType::Bytes => {
                let m = format!("r.read_{}(bytes)", self.proto_type());
                let cow = format!("{}.map(Cow::Borrowed)", m);
                (m, cow)
            },
            _ => {
                let m = format!("r.read_{}(bytes)", self.proto_type());
                (m.clone(), m)
            }
        })
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

    fn sanitize_default(&mut self, desc: &FileDescriptor) -> Result<()> {
        if let Some(ref mut d) = self.default {
            *d = match &*self.typ.rust_type(desc)? {
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
        Ok(())
    }

    fn has_regular_default(&self, desc: &FileDescriptor) -> bool {
        self.default.is_none()
            || self.default.as_ref().map(|d| &**d) == self.typ.regular_default(desc)
    }

    fn tag(&self) -> u32 {
        tag(self.number as u32, &self.typ, self.packed())
    }

    fn write_definition<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        write!(w, "    pub {}: ", self.name)?;
        let rust_type = self.typ.rust_type(desc)?;
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

    fn write_match_tag<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {

        // special case for FieldType::Map: destructure tuple before inserting in HashMap
        if let FieldType::Map(ref m) = self.typ {
            let &(ref key, ref value) = &**m;

            writeln!(w, "                Ok({}) => {{", self.tag())?;
            writeln!(w, "                    let (key, value) = r.read_map(bytes, |r, bytes| {}, |r, bytes| {})?;",
                     key.read_fn(desc)?.1, value.read_fn(desc)?.1)?;
            writeln!(w, "                    msg.{}.insert(key, value);", self.name)?;
            writeln!(w, "                }}")?;
            return Ok(());
        }

        let (val, val_cow) = self.typ.read_fn(desc)?;
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

fn get_modules(module: &str, imported: bool, desc: &FileDescriptor) -> String {
    let skip = if desc.package.is_empty() && ! imported { 1 } else { 0 };
    module
        .split('.').filter(|p| !p.is_empty())
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
    pub module: String, // 'package' corresponding to actual generated Rust module
}

impl Message {

    fn is_leaf(&self, leaf_messages: &[String]) -> bool {
        self.imported ||
            self.fields.iter()
            .chain(self.oneofs.iter().flat_map(|o| o.fields.iter()))
            .all(|f| f.is_leaf(leaf_messages) || f.deprecated)
    }

    fn has_lifetime(&self, desc: &FileDescriptor) -> bool {
        self.fields.iter()
            .chain(self.oneofs.iter().flat_map(|o| o.fields.iter()))
            .any(|f| match f.typ {
                FieldType::Message(ref m) if &m[..] == self.name => false,
                ref t => t.has_lifetime(desc, f.packed()),
            })
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
        get_modules(&self.module,self.imported,desc)
    }

    fn is_unit(&self) -> bool {
        self.fields.is_empty() && self.oneofs.is_empty()
    }

    fn write<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        println!("Writing message {}{}", self.get_modules(desc), self.name);
        writeln!(w, "")?;

        self.write_definition(w, desc)?;
        writeln!(w, "")?;
        self.write_impl_message_read(w, desc)?;
        writeln!(w, "")?;
        self.write_impl_message_write(w, desc)?;

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
                m.write(w, desc)?;
            }
            for e in &self.enums {
                e.write(w)?;
            }
            for o in &self.oneofs {
                o.write(w, desc)?;
            }

            writeln!(w, "")?;
            writeln!(w, "}}")?;
        }

        Ok(())
    }

    fn write_definition<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        writeln!(w, "#[derive(Debug, Default, PartialEq, Clone)]")?;
        if self.is_unit() {
            writeln!(w, "pub struct {} {{ }}", self.name)?;
            return Ok(());
        }

        if self.has_lifetime(desc) {
            writeln!(w, "pub struct {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "pub struct {} {{", self.name)?;
        }
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            f.write_definition(w, desc)?;
        }
        for o in &self.oneofs {
            o.write_message_definition(w, desc)?;
        }
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_impl_message_read<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        if self.is_unit() {
            writeln!(w, "impl<'a> MessageRead<'a> for {} {{", self.name)?;
            writeln!(w, "    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {{")?;
            writeln!(w, "        r.read_to_end();")?;
            writeln!(w, "        Ok(Self::default())")?;
            writeln!(w, "    }}")?;
            writeln!(w, "}}")?;
            return Ok(());
        }

        if self.has_lifetime(desc) {
            writeln!(w, "impl<'a> MessageRead<'a> for {}<'a> {{", self.name)?;
            writeln!(w, "    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {{")?;
        } else {
            writeln!(w, "impl<'a> MessageRead<'a> for {} {{", self.name)?;
            writeln!(w, "    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {{")?;
        }

        let unregular_defaults = self.fields.iter()
            .filter(|f| !f.has_regular_default(desc))
            .collect::<Vec<_>>();
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
            f.write_match_tag(w, desc)?;
        }
        for o in &self.oneofs {
            o.write_match_tag(w, desc)?;
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

    fn write_impl_message_write<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        if self.is_unit() {
            writeln!(w, "impl MessageWrite for {} {{ }}", self.name)?;
            return Ok(());
        }

        if self.has_lifetime(desc) {
            writeln!(w, "impl<'a> MessageWrite for {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "impl MessageWrite for {} {{", self.name)?;
        }
        self.write_get_size(w,desc)?;
        writeln!(w, "")?;
        self.write_write_message(w,desc)?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_get_size<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        writeln!(w, "    fn get_size(&self) -> usize {{")?;
        writeln!(w, "        0")?;
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            f.write_get_size(w)?;
        }
        for o in self.oneofs.iter() {
            o.write_get_size(w,desc)?;
        }
        writeln!(w, "    }}")?;
        Ok(())
    }

    fn write_write_message<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        writeln!(w, "    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {{")?;
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            f.write_write(w)?;
        }
        for o in &self.oneofs {
            o.write_write(w,desc)?;
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
            (format!("{}.{}", package, self.name), format!("{}.mod_{}", module, self.name))
        };

        for m in &mut self.messages { m.set_package(&child_package,&child_module); }
        for m in &mut self.enums { m.set_package(&child_package,&child_module); }
        for m in &mut self.oneofs { m.set_package(&child_package,&child_module); }
    }

    /// Searches for a matching message in all message
    ///
    /// If none is found, then it is an enum
    fn set_enums(&mut self, desc: &FileDescriptor) {
        for f in self.fields.iter_mut()
            .chain(self.oneofs.iter_mut().flat_map(|o| o.fields.iter_mut()))
        {
            if f.typ.find_message(&desc.messages).is_none() {
                if let FieldType::Message(m) = f.typ.clone() {
                    f.typ = FieldType::Enum(m);
                }
            }
        }
        for m in &mut self.messages {
            m.set_enums(desc);
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

    fn sanitize_defaults(&mut self, desc: &FileDescriptor) -> Result<()> {
        for f in self.fields.iter_mut()
            .chain(self.oneofs.iter_mut().flat_map(|o| o.fields.iter_mut())) {
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
}

#[derive(Debug, Clone)]
pub struct Enumerator {
    pub name: String,
    pub fields: Vec<(String, i32)>,
    pub imported: bool,
    pub package: String,
    pub module: String,
}

impl Enumerator {

    fn set_package(&mut self, package: &str, module: &str) {
        self.package = package.to_string();
        self.module = module.to_string();
    }

    fn sanitize_names(&mut self) {
        sanitize_keyword(&mut self.name);
        sanitize_keyword(&mut self.package);
        for f in self.fields.iter_mut() {
            sanitize_keyword(&mut f.0);
        }
    }

    fn get_modules(&self, desc: &FileDescriptor) -> String {
        get_modules(&self.module,self.imported,desc)
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
    pub module: String,
    pub imported: bool,
}

impl OneOf {

    fn has_lifetime(&self, desc: &FileDescriptor) -> bool {
        self.fields.iter().any(|f| !f.deprecated && f.typ.has_lifetime(desc, f.packed()))
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
        get_modules(&self.module,self.imported,desc)
    }

    fn write<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        writeln!(w, "")?;
        self.write_definition(w, desc)?;
        writeln!(w, "")?;
        self.write_impl_default(w, desc)?;
        Ok(())
    }

    fn write_definition<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        writeln!(w, "#[derive(Debug, PartialEq, Clone)]")?;
        if self.has_lifetime(desc) {
            writeln!(w, "pub enum OneOf{}<'a> {{", self.name)?;
        } else {
            writeln!(w, "pub enum OneOf{} {{", self.name)?;
        }
        for f in &self.fields {
            writeln!(w, "    {}({}),", f.name, f.typ.rust_type(desc)?)?;
        }
        writeln!(w, "    None,")?;
        writeln!(w, "}}")?;
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
            writeln!(w, "    pub {}: {}OneOf{}<'a>,", self.name, self.get_modules(desc), self.name)?;
        } else {
            writeln!(w, "    pub {}: {}OneOf{},", self.name, self.get_modules(desc), self.name)?;
        }
        Ok(())
    }

    fn write_match_tag<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            let (val, val_cow) = f.typ.read_fn(desc)?;
            if f.boxed {
                writeln!(w, "                Ok({}) => msg.{} = {}OneOf{}::{}(Box::new({}?)),",
                       f.tag(), self.name, self.get_modules(desc), self.name, f.name, val)?;
            } else {
                writeln!(w, "                Ok({}) => msg.{} = {}OneOf{}::{}({}?),",
                       f.tag(), self.name, self.get_modules(desc), self.name, f.name, val_cow)?;
            }
        }
        Ok(())
    }

    fn write_get_size<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        writeln!(w, "        + match self.{} {{", self.name)?;
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            let tag_size = sizeof_varint(f.tag());
            if f.typ.is_fixed_size() {
                writeln!(w, "            {}OneOf{}::{}(_) => {} + {},",
                         self.get_modules(desc), self.name, f.name, tag_size, f.typ.get_size(""))?;
            } else {
                writeln!(w, "            {}OneOf{}::{}(ref m) => {} + {},",
                         self.get_modules(desc), self.name, f.name, tag_size, f.typ.get_size("m"))?;
            }
        }
        writeln!(w, "            {}OneOf{}::None => 0,", self.get_modules(desc), self.name)?;
        write!(w, "    }}")?;
        Ok(())
    }

    fn write_write<W: Write>(&self, w: &mut W, desc: &FileDescriptor) -> Result<()> {
        write!(w, "        match self.{} {{", self.name)?;
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            writeln!(w, "            {}OneOf{}::{}(ref m) => {{ w.write_with_tag({}, |w| w.{})? }},",
                     self.get_modules(desc), self.name, f.name, f.tag(), f.typ.get_write("m", f.boxed))?;
        }
        writeln!(w, "            {}OneOf{}::None => {{}},", self.get_modules(desc), self.name)?;
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
}

#[derive(Debug, Default, Clone)]
pub struct FileDescriptor {
    pub import_paths: Vec<PathBuf>,
    pub package: String,
    pub syntax: Syntax,
    pub messages: Vec<Message>,
    pub enums: Vec<Enumerator>,
    pub module: String,
}

impl FileDescriptor {

    pub fn write_proto(config: &Config) -> Result<()> {
        let mut desc = FileDescriptor::read_proto(&config.in_file, &config.import_search_path)?;

        if desc.messages.is_empty() && desc.enums.is_empty() {
            // There could had been unsupported structures, so bail early
            bail!(ErrorKind::EmptyRead);
        }

        desc.set_enums();

        let mut leaf_messages = Vec::new();
        break_cycles(&mut desc.messages, &mut leaf_messages);

        desc.sanity_checks()?;
        desc.set_defaults()?;
        desc.sanitize_names();

        if config.single_module {
            desc.package = "".to_string();
        }


        let (prefix,file_package) = split_package(&desc.package);

        let mut file_stem = if file_package.is_empty() {
            get_file_stem(&config.out_file)?
        } else {
            file_package.to_string()
        };

        if ! file_package.is_empty() {
            sanitize_keyword(&mut file_stem);
        }
        let mut out_file = config.out_file.with_file_name(format!("{}.rs", file_stem));

        if ! prefix.is_empty() {
            use std::fs::create_dir_all;
            // e.g. package is a.b; we need to create directory 'a' and insert it into the path
            let file = PathBuf::from(out_file.file_name().unwrap());
            out_file.pop();
            for p in prefix.split('.') {
                out_file.push(p);
            }
            if ! out_file.exists() {
                create_dir_all(&out_file)?;
                update_mod_file(&out_file)?;
            }
            out_file.push(file);
        }
        if config.no_output {
            let imported = |b| if b {" imported"} else {""};
            println!("source will be written to {}\n",out_file.display());
            for m in &desc.messages {
                println!("message {} module {}{}",m.name,m.module,imported(m.imported));
            }
            for e in &desc.enums {
                println!("enum {} module {}{}",e.name,e.module,imported(e.imported));
            }
            return Ok(());
        }

        let name = config.in_file.file_name().and_then(|e| e.to_str()).unwrap();
        let mut w = BufWriter::new(File::create(&out_file)?);
        desc.write(&mut w, name)?;
        update_mod_file(&out_file)
    }

    /// Opens a proto file, reads it and returns raw parsed data
    fn read_proto(in_file: &Path, import_search_path: &[PathBuf]) -> Result<FileDescriptor> {
        let mut buf = Vec::new();
        {
            let f = File::open(in_file)?;
            let mut reader = BufReader::new(f);
            reader.read_to_end(&mut buf)?;
        }
        let mut desc = file_descriptor(&buf).to_result()?;

        // proto files with no packages are given an implicit module,
        // since every generated Rust source file represents a module
        desc.module = if desc.package.is_empty() {
            get_file_stem(in_file)?.clone()
        } else {
            desc.package.clone()
        };

        desc.fetch_imports(&in_file, import_search_path)?;
        Ok(desc)
    }

    fn sanity_checks(&self) -> Result<()> {
        for m in &self.messages {
            m.sanity_checks()?;
        }
        Ok(())
    }


    /// Get messages and enums from imports
    fn fetch_imports(&mut self, in_file: &Path, import_search_path: &[PathBuf]) -> Result<()> {
        for m in &mut self.messages {
            m.set_package("",&self.module);
        }
        for m in &mut self.enums {
            m.set_package("",&self.module);
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
                    in_file.parent()
                        .map_or_else(|| path.join(&import),
                         |p| p.join(path).join(&import))
                };
                if candidate.exists() {
                    matching_file = Some(candidate);
                    break;
                }
            }
            if matching_file.is_none() {
                return Err(ErrorKind::InvalidImport(
                        format!("file {} not found on import path", import.display())).into());
            }
            let proto_file = matching_file.unwrap();
            let mut f = FileDescriptor::read_proto(&proto_file, import_search_path)?;

            // if the proto has a packge then the names will be prefixed
            let package = f.package.clone();
            let module = f.module.clone();
            self.messages.extend(f.messages.drain(..).map(|mut m| {
                if m.package.is_empty() { m.set_package(&package,&module); }
                m.set_imported();
                m
            }));
            self.enums.extend(f.enums.drain(..).map(|mut e| {
                if e.package.is_empty() { e.set_package(&package,&module); }
                e.imported = true;
                e
            }));
        }
        Ok(())
    }

    fn set_defaults(&mut self) -> Result<()>{
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

    fn set_enums(&mut self) {
        let copy = self.clone();

        for m in &mut self.messages {
            m.set_enums(&copy);
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
        writeln!(w, "#![allow(unused_imports)]")?;
        writeln!(w, "#![allow(unknown_lints)]")?;
        writeln!(w, "#![allow(clippy)]")?;

        writeln!(w, "#![cfg_attr(rustfmt, rustfmt_skip)]")?;
        writeln!(w, "")?;
        Ok(())
    }

    fn write_uses<W: Write>(&self, w: &mut W) -> Result<()> {
        if self.messages.iter().all(|m| m.is_unit()) {
            writeln!(w, "use quick_protobuf::{{BytesReader, Result, MessageRead, MessageWrite}};")?;
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
        writeln!(w, "use quick_protobuf::{{MessageRead, MessageWrite, BytesReader, Writer, Result}};")?;
        writeln!(w, "use quick_protobuf::sizeofs::*;")?;
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
            write!(w,"super::")?;
        }
        writeln!(w,"*;")?;
        Ok(())
    }

    fn write_package_start<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "")?;
        Ok(())
    }

    fn write_package_end<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "")?;
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
            m.write(w, &self)?;
        }
        Ok(())
    }
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

/// "" is ("",""), "a" is ("","a"), "a.b" is ("a"."b"), and so forth.
fn split_package(package: &str) -> (&str,&str) {
    if package.is_empty() {
        ("","")
    } else {
        if let Some(i) = package.rfind('.') {
            (&package[0..i],&package[i+1..])
        } else {
            ("",package)
        }
    }
}

const MAGIC_HEADER: &'static str = "//! Automatically generated mod.rs";

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
                let rest = &line[i+matches.len()..line.len()-1];
                if rest == name {
                    // we already have a reference to this module...
                    present = true;
                    break;
                }
            }
        }
    }
    if ! present {
        let mut f = if exists {
            OpenOptions::new().append(true).open(&file)?
        } else {
            let mut f = File::create(&file)?;
            write!(f,"{}\n", MAGIC_HEADER)?;
            f
        };
        write!(f,"pub mod {};\n",name)?;
    }
    Ok(())
}

/// get the proper sanitized file stem from an input file path
fn get_file_stem(path: &Path) -> Result<String> {
     let mut file_stem = path.file_stem()
        .and_then(|f| f.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| Error::from(ErrorKind::OutputFile(path.to_owned())))?;

    file_stem = file_stem.replace(|c: char| ! c.is_alphanumeric(),"_");
    // will now be properly alphanumeric, but may be a keyword!
    sanitize_keyword(&mut file_stem);
    Ok(file_stem)
}
